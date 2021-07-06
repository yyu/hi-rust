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
