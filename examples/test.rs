#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

use async_trait_static::async_trait;
trait AsyncFnTrait {
    type RititReturnRun<'_async_lifetime>: core::future::Future<Output = &'_async_lifetime u8>;
    fn run<'_async_lifetime>(&'_async_lifetime self) -> Self::RititReturnRun<'_async_lifetime>;
}
struct AsyncStruct;
impl AsyncStruct {
    async fn hello(&self) -> u8 {
        1
    }
}
impl AsyncFnTrait for AsyncStruct {
    type RititReturnRun<'_async_lifetime> =
        impl core::future::Future<Output = &'_async_lifetime u8>;
    fn run<'_async_lifetime>(&'_async_lifetime self) -> Self::RititReturnRun<'_async_lifetime> {
        async move {
            self.hello().await;
            &0
        }
    }
}
fn main() {}