
    [2020-02-16 14:41:24]~/___/github/hi-rust/rust-lang_book/projects(master)$ cargo new hello_cargo
         Created binary (application) `hello_cargo` package
    [2020-02-16 14:42:17]~/___/github/hi-rust/rust-lang_book/projects(master)$ tree -a
    .
    ├── hello_cargo
    │   ├── Cargo.toml
    │   └── src
    │       └── main.rs
    └── hello_world
        ├── main
        └── main.rs

    3 directories, 4 files
    [2020-02-16 14:42:49]~/___/github/hi-rust/rust-lang_book/projects(master)$ vimcat hello_cargo/Cargo.toml
    [package]
    name = "hello_cargo"
    version = "0.1.0"
    authors = ["y² <yy@yuyuan.org>"]
    edition = "2018"

    # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

    [dependencies]
    [2020-02-16 14:43:18]~/___/github/hi-rust/rust-lang_book/projects(master)$ vimcat hello_cargo/src/main.rs
    fn main() {
        println!("Hello, world!");
    }
    [2020-02-16 14:48:25]~/___/github/hi-rust/rust-lang_book/projects(master)$ cd hello_cargo/
    [2020-02-16 14:48:28]~/___/github/hi-rust/rust-lang_book/projects/hello_cargo(master)$ cargo build
       Compiling hello_cargo v0.1.0 (/home/yuanyu/___/github/hi-rust/rust-lang_book/projects/hello_cargo)
        Finished dev [unoptimized + debuginfo] target(s) in 0.25s
    [2020-02-16 14:48:34]~/___/github/hi-rust/rust-lang_book/projects/hello_cargo(master)$ tree -a
    .
    ├── Cargo.lock
    ├── Cargo.toml
    ├── src
    │   └── main.rs
    └── target
        ├── debug
        │   ├── build
        │   ├── .cargo-lock
        │   ├── deps
        │   │   ├── hello_cargo-c9aa63d1aaa59939
        │   │   └── hello_cargo-c9aa63d1aaa59939.d
        │   ├── examples
        │   ├── .fingerprint
        │   │   └── hello_cargo-c9aa63d1aaa59939
        │   │       ├── bin-hello_cargo-c9aa63d1aaa59939
        │   │       ├── bin-hello_cargo-c9aa63d1aaa59939.json
        │   │       ├── dep-bin-hello_cargo-c9aa63d1aaa59939
        │   │       └── invoked.timestamp
        │   ├── hello_cargo
        │   ├── hello_cargo.d
        │   └── incremental
        │       └── hello_cargo-2wrq5qu63srad
        │           ├── s-fkqfr85nzd-16isfuk-30fgcmxv7utrq
        │           │   ├── 1zpdfq6yalyo728v.o
        │           │   ├── 29gab5x0y9sym3mi.o
        │           │   ├── 3huoj5xnrtnapmun.o
        │           │   ├── 4tf4uztoa0199ywx.o
        │           │   ├── 59bdmswnhsf8o5rw.o
        │           │   ├── 59o24b9pp4bvnnbf.o
        │           │   ├── dep-graph.bin
        │           │   ├── query-cache.bin
        │           │   └── work-products.bin
        │           └── s-fkqfr85nzd-16isfuk.lock
        └── .rustc_info.json

    11 directories, 23 files
    [2020-02-16 14:48:37]~/___/github/hi-rust/rust-lang_book/projects/hello_cargo(master)$ ./target/debug/hello_cargo
    Hello, world!
    [2020-02-16 14:49:00]~/___/github/hi-rust/rust-lang_book/projects/hello_cargo(master)$ cargo run
        Finished dev [unoptimized + debuginfo] target(s) in 0.01s
         Running `target/debug/hello_cargo`
    Hello, world!

