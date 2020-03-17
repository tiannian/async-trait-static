#![feature(type_alias_impl_trait)]
use async_trait_static::async_trait;

async fn hello() -> u8 {
    println!("hello");
    1
}

struct TraitStruct;

type FutureReturnTypeTestFnDefaultMyTrait = impl core::future::Future<Output = u8>;
trait MyTrait {
    fn test_fn<'async_trait>(&self) -> Self::FutureReturnTypeTestFn;
    fn test_fn_default<'async_trait>(&self) -> FutureReturnTypeTestFnDefaultMyTrait {
        async move { self.test_fn() }
    }
    type FutureReturnTypeTestFn: core::future::Future<Output = u8>;
}
impl MyTrait for TraitStruct {
    fn test_fn<'async_trait>(&self) -> Self::FutureReturnTypeTestFn {
        async move { hello().await }
    }
    type FutureReturnTypeTestFn = impl core::future::Future<Output = u8>;
}

fn main() {
    let s = TraitStruct;
    s.test_fn();
    s.test_fn_default();
}
