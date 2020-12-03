#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

trait AsyncFnTrait {
    fn run<T>(&self, t: T) -> Self::ReturnTypeFutureReturnTypeRun<'_, T>;
    type ReturnTypeFutureReturnTypeRun<'a, T>: core::future::Future<Output = ()>;
}

struct AsyncStruct;

impl AsyncStruct {
    async fn hello(&self) -> u8 { 1 }
}

impl AsyncFnTrait for AsyncStruct {
    fn run<T>(&self, t: T) -> Self::ReturnTypeFutureReturnTypeRun<'_, T> {
        async move  { self.hello().await; }
    }
    type ReturnTypeFutureReturnTypeRun<'a, T> = impl core::future::Future<Output = ()>;
}

fn main() {}