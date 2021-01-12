#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

type RititReturnDeff<'_async_lifetime, S> = impl core::future::Future<Output = u8>;

trait AsyncFnTrait {
    type RititReturnRun<'_async_lifetime>: core::future::Future<Output = ()>;

    fn run<'_async_lifetime>(&'_async_lifetime self) -> Self::RititReturnRun<'_async_lifetime>;
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
    type RititReturnRun<'_async_lifetime> = impl core::future::Future<Output = ()>;
    fn run<'_async_lifetime>(&'_async_lifetime self) -> Self::RititReturnRun<'_async_lifetime> {
        async move {
            self.hello().await;
        }
    }
}

fn main() {
    let s = AsyncStruct;
    async {
        s.deff().await;
    };
}

