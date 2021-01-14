# Async trait methods for `no_std`

Features like `async-trait`, avoid using `Box` and `dyn`. You can use async keywork in trait without alloc.

Thanks to crate [async-trait](https://github.com/dtolnay/async-trait), some code from these.

WARNING: This crate use some unstable even incomplete feature. You will get some warning from compiler.

If you want to use crate, please add `#![feature(type_alias_impl_trait)]` and `#![feature(generic_associated_types)]`
to you crate's root file.

This crate support `async` in trait through `#[async_trait]` and sup

## Support syntax

- `async` in trait. `#[async_trait]`.
- `impl trait` as return in trait. `#[ritit]`.

## Features Status

- [X] `Self`
  - [X] `Self` by reference.
  - [X] `Self` by value.
  - [X] `Self` by mut reference.
  - [X] no `Self`.
  - [X] any type of `Self`. (need test)
  - [ ] `Self` by mut value. (It seems unimplementable)
- [X] Any number of arguments, any return value.
  - [X] Arguments.
    - [X] As value.
    - [X] As reference without lifetime.
  - [X] Return value expect reference (return reference at `Lifetime return`). (need test)
- [ ] Lifetime parameters. (need test)
  - [ ] Lifetime arguments.
  - [ ] Lifetime return.
- [X] Associated types support. (need test)
- [X] Having async and non-async functions in the same trait.
- [X] support default `async fn` implementations in trait.
- [X] Generic type parameters.
  - [X] Generic arguments.
  - [X] Generic return.
  - [ ] `impl trait` in arguments. (need implement)

## Usage

Please enable feature `type_alias_impl_trait` and `generic_associated_types`;

### async_trait

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

### ritit
```rust
#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

use async_trait_static::ritit;

#[ritit]
trait AsyncFnTrait {
    fn run<T: Clone>(&self, t: T) -> impl core::future::Future<Output = ()>;
    fn deff(&self) -> impl core::future::Future<Output = u8> {
        async move  { 1 }
    }
}

struct AsyncStruct;

impl AsyncStruct {
    async fn hello(&self) -> u8 {
        1
    }
}

#[ritit]
impl AsyncFnTrait for AsyncStruct {
    fn run<T: Clone>(&self, t: T) -> impl core::future::Future<Output = ()> {
        async move {
            self.hello().await;
        }
    }
}
```

