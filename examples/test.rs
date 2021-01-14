#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

use async_trait_static::async_trait;
type RititReturnDeff<'_async_lifetime, RititS> = impl core::future::Future<Output = u8>;
trait AsyncFnTrait {
    type RititReturnRun<'_async_lifetime, 'a>: core::future::Future<Output = u8>;
    fn run<'_async_lifetime, 'a>(
        &'_async_lifetime self,
        i: &'a u8,
    ) -> Self::RititReturnRun<'_async_lifetime, 'a>
    where
        'a: '_async_lifetime;
    fn deff<'_async_lifetime>(&'_async_lifetime self) -> RititReturnDeff<'_async_lifetime, Self>
    where
        Self: Sized,
    {
        async move { 1 }
    }
}
struct AsyncStruct;
impl AsyncStruct {
    async fn hello(&self) -> u8 {
        1
    }
}
impl AsyncFnTrait for AsyncStruct {
    type RititReturnRun<'_async_lifetime, 'a> = impl core::future::Future<Output = u8>;
    fn run<'_async_lifetime, 'a>(
        &'_async_lifetime self,
        i: &'a u8,
    ) -> Self::RititReturnRun<'_async_lifetime, 'a>
    where
        'a: '_async_lifetime,
    {
        async move {
            self.hello().await;
            0
        }
    }
}
fn main() {}
