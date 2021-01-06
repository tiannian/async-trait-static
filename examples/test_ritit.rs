//Compiled

#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

use async_trait_static::ritit;

#[ritit]
trait AsyncFnTrait {
    fn run<T: Clone>(&self, t: T) -> impl core::future::Future<Output = ()>;
    // fn deff(&self) -> impl core::future::Future<Output = u8> {
    //     async move  { 1 }
    // }
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

fn main() {}
