# Rust Recap

## Road map

### CS fundamental: Stack, heap, pointers, etc

### Memory Management Fundamental

* Manual Memory Management

* garbage collector

### Primitives, variables and Control Flow

### Memory Safety with Ownership& Borrowing

### Type system

* Option

* Result: error handling

* Struct

* Enums

### How to structure project

Rust module system, how to managem dependency

### Polyorphism with generics and traits

### Error handling

### Functional features

closures, function ptrs, iterators

### Concurrency, async/.await

### Macro system

### Unsafe Rust, FFi

## Must know

* Build/Run: `cargo new`, `cargo build`, ...

* Print to console

* Primitive variables: [example](https://compiler-explorer.com/z/zrfY4Tzaq)

* Arrays/Tuples: [example](https://compiler-explorer.com/z/x41z6v5vs)

* Tuples: [example](https://compiler-explorer.com/z/x41z6v5vs)

* function: [example](https://compiler-explorer.com/z/WvYoM4TjM)

* Mutability, Ref, Borrowing

* Arrays and Slices

* Strings

* Loops (if, for, while)

* Match statement (switch cases) [example](https://compiler-explorer.com/z/TM87M4cY5)


## Different Pointers 

A pointer is just a variable that holds memory address, this address points to some data.

## What is a smart pointer?

Smart pointer in Rust is a concept orignated from C++ and others, they are data structures that act like a pointer but have additional capacities and metadata in them.

In Rust, a smart ptr is defined mostly as a struct that implements `Deref`, `Drop` traits.

### The `Deref` trait

Allow one to define the behavior when we want to use the type as a reference like accessing an inner field of the inner struct.

### The `Drop` trait

Allows us to define the behavior when the pointer goes out of scope.

## The different pointers in Rustlang

### References

The most common `pointer` in Rust, one creates it with `&` and they borrow the value they point to

### String

String type in Rust is a smart pointer, basically a `Vec<u8>`.

### `Vec<T>`

`Vec` is a smart pointer that is a growable array.

### `Box<T>`

Box is a smart pointer that holds a pointer to data on heap.

### `RC<T>`

`RC` is a single threaded reference counted pointer designated for multiple owndership, in some sitiation we cannot determine an exclusive owner for some data we use `Rc` which holds a counter and give owner access of some data on the heap to multiple owners and everytime it drops it will decrement the counter and when reached 0 destroy the data, remember that `Rc` is not thread safe.

### `Arc<T>`

`Arc` has exact same semantics as `Rc` except that it uses an atomic reference counter
which then makes Arc a thread safe pointer which one can pass to multiple threads safely, basically makes multiple owners on multiple threads available.

### `Ref<T>, RefMut<T>, RefCell<T>`

Basically applies all borrow check rules in runtime so in situation where one need the compiler to ignore borrow checking for us, we can use `RefCell`, of course since underness of `RefCell` is unsafe code hence not recommended in most cases ;)

## Summary

- `Rc<T>` enable multiple owners of the same data

- `Box<T>` and `RefCell<T>` have single owners

- `Box<T>` allows immutable borrows checked at compile time; `RefCell<T>` allows immutable or mutable borrows checked at runtime.

- `Rc<T>` allows only immutable bowwows checked at compile time; `RefCell<T>` allows immutable or mutable borrows checked at run time

- Since `RefCell<T>` allows mutable borrow checked at run time, one can mutate the value inside the `RefCell<T>` even when the `RefCell<T>` is immutable.



# References

https://doc.rust-lang.org/rust-by-example

https://rust-by-example-ext.com/index.html

https://rust-lang-nursery.github.io/rust-cookbook

https://ggbaker.ca/prog-langs/

https://www.hackertouch.com/rust-programming-language.html

https://lborb.github.io/book/

https://crates.io/crates/cargo-modules

https://learning-rust.github.io/docs/d3.modules.html

https://stackoverflow.com/questions/57756927/rust-modules-confusion-when-there-is-main-rs-and-lib-rs


