# rust notes

![](https://www.rust-lang.org/logos/rust-logo-blk.svg)

[rustup](https://rustup.rs/) is an installer for the systems programming language Rust.
It is an official Rust project.

## reading ![The Rust Programming Language](https://learning.oreilly.com/library/cover/9781098122539/)

Interactive learning: https://rust-book.cs.brown.edu/

Terminology
* *prelude*
  * By default, Rust has a set of items defined in the standard library that it brings into the scope of every program. This set is called the prelude.
  * If a type you want to use isn’t in the prelude, you have to bring that type into scope explicitly with a use statement.
* *associated function*
  * a function that's implemented on a type
  * example: `String::new()`
    * `::` indicates `new` is an associated function of the `String` type
* *enumeration*, *enum*, *variant*
  * *enumeration*, aka *enum*, is a type that can be in one of multiple possible states
  * each possible state is called a *variant*
  * `Result`'s variants are `Ok` and `Err`
* *crate*
  * a crate is a collection of Rust source code files
  * binary crate: an executable
  * library crate: contains code that is intended to be used in other programs and can't be executed on its own
  * You won’t just know which traits to use and which methods and functions to call from a crate, so each crate has documentation
* *Semantic Versioning*, *SemVer*
  * a standard for writing version numbers
  * https://semver.org/
* *shadow*
  * *shadowing* lets us reuse a variable name
  * often used when converting a value from one type to another

`rustup`:
* update to a newly released version
  * `rustup update`
* uninstall Rust and `rustup`
  * `rustup self uninstall`
* open the local doc in browser
  * `rustup doc`

> Rust style is to indent with four spaces

> using a ! means that you’re calling a macro instead of a normal function

> Cargo is Rust’s build system and package manager.

> In Rust, packages of code are referred to as crates.

cargo:
* create a project
  * `cargo new`
* build a project
  * `cargo build`
* build and run a project
  * `cargo run`
* build a project without producing a binary
  * `cargo check`
* build with optimizations
  * `cargo release`
* build documentation provided by all dependencies locally and open it in browser
  * `cargo doc --open`

*Cargo.lock* file
* created on the first run of `cargo build`
* following builds will use versions from Cargo.lock file
* often checked into source control because it's important for reproducible builds
* `cargo update` will ignore the Cargo.lock file and figure out all the latest versions that fit the specifications in Cargo.toml

pattern matching
  * a `match` expression is made up of `arms`
  * an `arm` consits of a `pattern` to match against, and the code that should be run if the value given to match fits that arm's pattern
  * Rust takes the value given to match and looks through each arm’s pattern in turn
  * the match expression ends after the first successful match

numbers
  * Unless otherwise specified, Rust defaults to an `i32`

*immutable variables* vs *constants*
* `mut` is not allowed to be used with constants
* the type of constants must be annotated
* constants can be declared in any scope, including the global scope
* constants may be set only to a constant expression, not the result of a value that could only be computed at runtime

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
