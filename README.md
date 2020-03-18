# Async trait methods for `no_std`

Features like `async-trait`, avoid using `Box` and `dyn`.

This crate is ready for `#![no_std]` when [PR69033](https://github.com/rust-lang/rust/pull/69033) merged.

Thanks to crate [async-trait](https://github.com/dtolnay/async-trait), some code from these.

WARNING: This crate use some unstable even incomplete feature. 

## Features

- [ ] `Self`
  - [X] `Self` by reference.
  - [X] `Self` by value.
  - [X] `Self` by mut value.
  - [X] no `Self`.
  - [ ] any type of `Self`.
- [ ] Any number of arguments, any return value.
  - [ ] Arguments (Wait test).
  - [X] Return value.
- [ ] Generic type parameters and lifetime parameters.
  - [ ] Generic arguments.
  - [ ] Generic Return.
- [ ] Associated types support.
- [X] Having async and non-async functions in the same trait.
- [ ] support default implementations in trait.

## Works
- [X] Solve lifetime for `self` reference.
- [ ] Find a way to process default implementations in trait.
  - [X] Don't define `type alias` as `associated type`.
  - [ ] Error: `type parameter Self is part of concrete type but not used in parameter list for the impl Trait type alias`

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

