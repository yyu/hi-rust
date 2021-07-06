# rust notes

![](https://www.rust-lang.org/logos/rust-logo-blk.svg)

[rustup](https://rustup.rs/) is an installer for the systems programming language Rust.
It is an official Rust project.

## reading ![The Rust Programming Language](https://learning.oreilly.com/library/cover/9781098122539/)

> Rust style is to indent with four spaces

> using a ! means that you’re calling a macro instead of a normal function

> Cargo is Rust’s build system and package manager.

> Several rules determine what a package can contain.
> * A package must contain zero or one library crates, and no more.
> * It can contain as many binary crates as you’d like, but
> * it must contain at least one crate (either library or binary).

> Modules aren’t useful only for organizing your code. They also define Rust’s *privacy boundary*: the line that encapsulates the implementation details external code isn’t allowed to know about, call, or rely on. So, if you want to make an item like a function or struct private, you put it in a module.
>
> The way privacy works in Rust is that all items (functions, methods, structs, enums, modules, and constants) are private by default.
> * Items in a parent module can’t use the private items inside child modules, but
> * items in child modules can use the items in their ancestor modules.

`pub`: `struct` vs `enum`

> If we use `pub` before a struct definition, we make the struct public, but the struct’s fields will still be private. We can make each field public or not on a case-by-case basis.
>
> In contrast, if we make an enum public, all of its variants are then public.
>
> * Enums aren’t very useful unless their variants are public; it would be annoying to have to annotate all enum variants with `pub` in every case, so the default for enum variants is to be public.
> * Structs are often useful without their fields being public, so struct fields follow the general rule of everything being private by default unless annotated with `pub`.

`use`: function vs `struct`/`enum`

> Bringing the function’s parent module into scope with use so we have to specify the parent module when calling the function makes it clear that the function isn’t locally defined while still minimizing repetition of the full path.
>
> On the other hand, when bringing in structs, enums, and other items with `use`, it’s idiomatic to specify the full path.
>
> There’s no strong reason behind this idiom: it’s just the convention that has emerged, and folks have gotten used to reading and writing Rust code this way.

`pub use`

> Re-exporting is useful when the internal structure of your code is different from how programmers calling your code would think about the domain. For example, in this restaurant metaphor, the people running the restaurant think about “front of house” and “back of house.” But customers visiting a restaurant probably won’t think about the parts of the restaurant in those terms. With `pub use`, we can write our code with one structure but expose a different structure. Doing so makes our library well organized for programmers working on the library and programmers calling the library.

`std`

> Because the standard library is shipped with the Rust language, we don’t need to change *Cargo.toml* to include `std`. But we do need to refer to it with use to bring items from there into our package’s scope. For example, with `HashMap` we would use this line:
>
> `use std::collections::HashMap;`
>
> This is an absolute path starting with `std`, the name of the standard library crate.
