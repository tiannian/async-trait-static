//Compiled
#![allow(incomplete_features)]

#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

use async_trait_static::ritit;
use core::future::Future;

#[ritit]
trait MixedFnTrait {
    fn run_sync(&self, a: u8) -> i16;
    fn run_async(&self, b: u16) -> impl Future<Output = i8>;
}

struct MixedStruct;

#[ritit]
impl MixedFnTrait for MixedStruct {
    fn run_sync(&self, _a: u8) -> i16 { 0 }
    fn run_async(&self, _b: u16) -> impl Future<Output = i8> { async move { 1 } }
}

fn main() {
    let ms = MixedStruct{};
    let _ = ms.run_sync(2);
    let _ = async { ms.run_async(3) };
}
