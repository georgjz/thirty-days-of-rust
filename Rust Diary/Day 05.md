# Day 5
Goals:
* Simple, minimal library example

## Modularity
Code can be split into multiple modules, then multiple files.
A package may contain multiple binary crates and one optional library crate.

*Modules*/*use* to control scope, privacy of paths.
*Paths* a way of naming items, such as structs, functions, etc.

*Binary crate*: compiles to executable, has `main` function
*Library crate*: compiles to executable, no `main`

Cargo starts compilation from the *crate root* (usually, `src/main.rs`) which also forms the root module.

### Modules
**Paths** name items, `use` brings paths into scope, `pub` makes paths public. 

Declare a module with `mod`, `mod foo;` will make the compiler look for inlined code, `src/foo.rs`, and `src/foo/mod.rs`.

Submodule `bar` inside module `foo` can be inlined, `src/foo/bar.rs`, or `src/foo/bar/mod.rs`.

The path to an item `baz` in the submodule `bar` is `crate::foo::bar::baz`. The `crate` part is the root module formed by the root crate.

Paths can be absolute or relative, based on `self` and `super`.

Items are private by default, **but child modules can use all parent items**. Children can see the context in which they are defined (this needs looking it, it's almost sound reasoning, lol).

This module system is super weird and verbose. The mixing of private and public modules seems to follow some strange logic.
