#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

trait AsyncFnTrait {
    type RititReturnRun<'_async_lifetime>: core::future::Future<Output = ()>;
    fn run<'_async_lifetime>() -> Self::RititReturnRun<'_async_lifetime>;
}
struct AsyncStruct;
async fn hello() -> u8 {
    1
}
impl AsyncFnTrait for AsyncStruct {
    type RititReturnRun<'_async_lifetime> = impl core::future::Future<Output = ()>;
    fn run<'_async_lifetime>() -> Self::RititReturnRun<'_async_lifetime> {
        async move {
            hello().await;
        }
    }
}
fn main() {}