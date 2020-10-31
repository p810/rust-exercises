## Quick notes
- **Packages** encapsulate crates
    * Contains a `Cargo.toml` file that describes how to build its crates
    * Can only contain up to one library crate, does not have to include one
    * Can contain any number of binary crates, and must contain at least one
    * Multiple binary crates can be included under `src/bin/`, where each file is a binary crate
- **Crates** are a tree of modules that yields a library or executable
    * The **crate root** is a source file that the compiler starts from and acts as the entry point for a crate
    * `src/main.rs` is the default root of a binary crate unless one is specified, similar to `index.js` in Node
    * For library crates, the default root is `src/lib.rs`
    * If both files are included then Rust will compile two crates (one binary and one library)
    * A crate will group related functionality together under its name, allowing consumers to reference its contents
    * The contents of either two files will be available under a module named `crate`
- **Modules** are essentially namespaced scopes for related aspects
    * Items in modules are private by default and must explicitly be marked as `pub` to be available in outer scopes
    * Child items in nested module scopes have access to items from their parents/ancestors, but their ancestors cannot see them unless marked as public
    * Marking a module as public will allow that module's ancestors to access its contents, but not anything outside
    * To allow third parties to call items within a module, the items and module must be marked as public
- **Paths** refer to the naming of entities, e.g. a struct or function
    * **Absolute paths** start from a crate root by using a module's name
    * **Relative paths** start from the current module and use `self`, `super` (for the parent scope/module), or an identifier in the module's scope
    * Double colons (`::`) are used as separators for tokens in a path, similar to PHP's backslash for namespaces

### Idiomatic imports
According to Ch. 7.4, "[Creating Idiomatic `use` Paths](https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#creating-idiomatic-use-paths):"

> Bringing \[a\] function’s parent module into scope with use so we have to specify the parent module when calling the function makes it clear that the function isn’t locally defined while still minimizing repetition of the full path.

So for example this..:

```rust
mod foo {
    pub mod bar {
        pub fn say_hello_to(name: &str) -> String {
            format!("Hello, {}!", name)
        }
    }
}

use crate::foo::bar;

pub fn greet(name: &str) -> () {
    println!("{}", bar::say_hello_to(name));
}
```

... is preferred over the following form:

```rust
use crate::foo::bar::say_hello_to;
```

But, it goes on to say the following about items other than functions:

> On the other hand, when bringing in structs, enums, and other items with use, it’s idiomatic to specify the full path. \[...\] There’s no strong reason behind this idiom: it’s just the convention that has emerged, and folks have gotten used to reading and writing Rust code this way.

I don't think that there's really all that much cause for confusion with the function paths being fully imported, especially since the IDE can be spruced up with a language server and whatnot to show the definition site on hover. 

The docs go on to introduce the `as` keyword in the following section, saying:

> The exception to this idiom is if we’re bringing two items with the same name into scope with use statements, because Rust doesn’t allow that. \[...\] we can specify `as` and a new local name, or alias, for the type. 

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn foo() -> Result {
    // returns an instance of std::fmt::Result
}

fn bar() -> IoResult {
    // returns an instance of std::io::Result
}
```

The alternative approach being:

```rust
use std::fmt;
use std::io;

fn foo() -> fmt::Result {}
fn bar() -> io::Result {}
```

Both choices are considered idiomatic Rust code.