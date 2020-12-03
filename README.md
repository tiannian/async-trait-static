# Async trait methods for `no_std`

Features like `async-trait`, avoid using `Box` and `dyn`.

This crate is ready for `#![no_std]` require `ngihtly` after `2020-04-06`.

Thanks to crate [async-trait](https://github.com/dtolnay/async-trait), some code from these.

WARNING: This crate use some unstable even incomplete feature. You will get some warning from compiler.

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
- [X] Lifetime parameters.
  - [X] Lifetime arguments.
  - [ ] Lifetime return.
- [X] Associated types support.
- [X] Having async and non-async functions in the same trait.

## Incomplete Feature (WIP)

> These feature all require `generic_associated_types` support type arguments.

> Now `generic_associated_types` support type arguments. (nightly-2020-12-03).

Following issue: [#44265](https://github.com/rust-lang/rust/issues/44265)

- [ ] support default `async fn` implementations in trait (Wait feature `generic_associated_types` support type arguments).
- [ ] Generic type parameters.
  - [ ] Generic arguments (Wait for feature `generic_associated_types` support type arguments).
  - [ ] Generic return (Wait for feature `generic_associated_types` support type arguments).
  - [ ] `impl trait` in arguments (Wait for feature `generic_associated_types` support type arguments).

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

