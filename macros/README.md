# Rust macros

## What is it
Macros enable you to write code that writes other code, which is known as metaprogramming.

Macros provide functionality similar to functions but without the runtime cost. There is some compile-time cost, however, since macros are expanded during compile time.

Rust macros are very different from macros in C. Rust macros are applied to the token tree whereas C macros are text substitution.

## Type of macros

* **Declarative macros** enable us 
to write stuff similar to a match expression that operates on Rust code we provide as arguments. It uses the code we provide to generate code that replaces the macro invocation

* **Procedural macros** allow us to operate on AST of the Rust code it is given. A proc macro is a function from a `TokenStream` (or two) to another `TokenStream`, wherwe the output replaces the macro invocation.