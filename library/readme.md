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