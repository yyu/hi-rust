/*
    # 76. Procedural Macros

    Procedural macros in Rust allow developers to extend the language and automate code generation. They are often used to create custom `derive` attributes or other annotations that modify or add to Rust's functionality.

    In this challenge, you'll implement a procedural macro to derive a custom trait called `Describe`. The `Describe` trait should provide a method, `describe`, that returns a string containing the name of the struct and its fields with their values. This macro should generate an implementation of the trait automatically when applied to a struct.

    ## Your Task

    Your task is to create a procedural macro named `derive_describe` that can be used to generate the `Describe` trait implementation for any struct. The generated `describe` method should return a string representation of the struct, including its name and all its fields with their respective values.

    For example, applying the macro to a struct like:

    ```rust
    #[derive(Describe)]
    struct Point {
        x: i32,
        y: i32,
    }
    ```

    Should generate a `Describe` implementation such that:

    ```rust
    let p = Point { x: 1, y: 2 };
    assert_eq!(p.describe(), "Point { x: 1, y: 2 }");
    ```

    ### Requirements

    1. The `derive_describe` macro should:
       - Generate the `Describe` trait implementation for the struct it is applied to.
       - Support structs with named fields. (Tuple structs or unit structs are not required for this challenge.)

    2. The `describe` method should return a properly formatted string, including the struct name and fields with their values.

    ## Good to know

    Procedural macros must be defined in a separate crate from the one where they are used. This is because procedural macros are compiled to a shared library that is loaded by the Rust compiler at compile time. The shared library must be compiled before the crate that uses the macro.

    In your `Cargo.toml`, you should define a new crate for the procedural macro:

    ```toml
    [lib]
    proc-macro = true
    ```

    This is already done for this challenge, but it's important to understand the setup.

    ## Hints

    <details>
        <summary>Click here to reveal hints</summary>

    - Use the `syn` crate to parse the input `TokenStream` into an Abstract Syntax Tree (AST).
    - Use the `quote` crate to generate Rust code as a `TokenStream`.

    </details>
*/

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Describe)]
pub fn derive_describe(input: TokenStream) -> TokenStream {
    // TODO: Implement the procedural macro here
    let ast = syn::parse(input).unwrap();
    impl_describe_macro(&ast)
}

fn impl_describe_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let field_prints = if let syn::Data::Struct(data_struct) = &ast.data {
        if let syn::Fields::Named(fields_named) = &data_struct.fields {
            // Map over each field to generate a .field("name", &self.name) call
            fields_named.named.iter().map(|f| {
                let field_name = f.ident.as_ref().unwrap();
                let field_name_str = field_name.to_string();
                quote! {
                    .field(#field_name_str, &self.#field_name)
                }
            }).collect::<Vec<_>>()
        } else {
            Vec::new()
        }
    } else {
        panic!("only supports structs with named fields");
    };

    let generated = quote! {
        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!(#name))
                    #(#field_prints)*
                    .finish()
            }
        }

        impl Describe for #name {
            fn describe(&self) -> String {
                format!("{:?}", self)
            }
        }
    };
    generated.into()
}

// Example Test
//#[test]
//fn test_example() {
//    #[derive(Describe)]
//    struct Person {
//        name: String,
//        age: u32,
//    }
//
//    let person = Person {
//        name: "Alice".to_string(),
//        age: 30,
//    };
//
//    assert_eq!(person.describe(), "Person { name: \"Alice\", age: 30 }");
//}
