# Async trait methods for `no_std`

Features like `async-trait`, avoid using `Box` and `dyn`.

This crate is ready for `#![no_std]` when [PR69033](https://github.com/rust-lang/rust/pull/69033) merged.

Thanks to crate [async-trait](https://github.com/dtolnay/async-trait), some code from these.

WARNING: This crate use some unstable even incomplete feature.

## Design

Re-implementation a new `Future`.

## Features

- [ ] `Self`
  - [ ] `Self` by reference.
  - [ ] `Self` by value.
  - [ ] `Self` by mut reference.
  - [ ] no `Self`.
  - [ ] any type of `Self`.
- [ ] Any number of arguments, any return value.
  - [ ] Arguments.
    - [ ] As value.
    - [ ] As reference without lifetime.
  - [ ] Return value expect reference (return reference at `Lifetime return`).
  - [ ] `impl trait` in arguments (due to `type_alias_impl_trait` require all arguments must be concrete type.).
- [ ] Generic type parameters and lifetime parameters.
  - [ ] Generic arguments.
  - [ ] Generic return.
  - [ ] Lifetime arguments.
  - [ ] Lifetime return.
- [ ] Associated types support.
- [ ] Having async and non-async functions in the same trait.
- [ ] support default implementations in trait.

## Works

- [ ] Solve lifetime for `self` reference.
- [ ] Rename name of future's GAT lifetime.
- [ ] Parse any type of `Self`.
- [ ] Find a way to process default implementations in trait.
  - [ ] Don't define `type alias` as `associated type`.
  - [ ] Error: `type_alias_impl_trait` as `fn`'s return type require all arguments must be concrete type.

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

