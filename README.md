# Async trait methods for `no_std`

Features like `async-trait`, avoid using `Box` and `dyn`.

This crate is ready for `#![no_std]` when [PR69033](https://github.com/rust-lang/rust/pull/69033) merged.

Thanks to crate [async-trait](https://github.com/dtolnay/async-trait), some code from these.

WARNING: This crate use some unstable even incomplete feature.

## Features

- [X] `Self`
  - [X] `Self` by reference.
  - [X] `Self` by value.
  - [X] `Self` by mut reference.
  - [X] no `Self`.
  - [X] any type of `Self`.
- [X] Any number of arguments, any return value.
  - [X] Arguments.
    - [X] As value.
    - [X] As reference without lifetime.
  - [X] Return value expect reference (return reference at `Lifetime return`).
- [ ] Generic type parameters and lifetime parameters.
  - [ ] Generic arguments (Wait feature `generic_associated_types` support type arguments).
  - [ ] Generic return (Wait feature `generic_associated_types` support type arguments).
  - [ ] Lifetime arguments.
  - [ ] Lifetime return.
  - [ ] `impl trait` in arguments (Wait feature `generic_associated_types` support type arguments).
- [X] Associated types support.
- [X] Having async and non-async functions in the same trait.
- [ ] support default `async fn` implementations in trait (Wait feature `generic_associated_types` support type arguments).

## Works

- [X] Solve lifetime for `self` reference.
- [ ] Rename name of future's GAT lifetime.
- [ ] Parse any type of `Self`.
- [ ] Find a way to process default implementations in trait.
  - [ ] Wait feature `generic_associated_types` support type arguments.

## Usage

Please enable feature `type_alias_impl_trait` and `generic_associated_types`;

```rust
#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

use async_trait_static::async_trait;

async fn hello() -> u8 {
    1
}

#[async_trait]
trait AsyncFnTrait {
    async fn run(&self);
}

struct AsyncStruct;

#[async_trait]
impl AsyncFnTrait for AsyncStruct {
    async fn run(&self) {
        hello().await;
    }
}

```

