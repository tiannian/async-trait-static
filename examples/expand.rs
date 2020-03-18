#![feature(prelude_import)]
#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]
use async_trait_static::async_trait;
async fn hello() -> u8 {
    1
}
trait MyTrait {
    fn test_fn(self) -> Self::ReturnTypeFutureReturnTypeTestFn;
    fn test_fn_by_mut_reference() -> Self::ReturnTypeFutureReturnTypeTestFnByMutReference;
    type ReturnTypeFutureReturnTypeTestFn: core::future::Future<Output = u8>;
    type ReturnTypeFutureReturnTypeTestFnByMutReference: core::future::Future<Output = ()>;
}
struct TraitStruct;
impl MyTrait for TraitStruct {
    fn test_fn(self) -> Self::ReturnTypeFutureReturnTypeTestFn {
        async move { 1 }
    }
    fn test_fn_by_mut_reference() -> Self::ReturnTypeFutureReturnTypeTestFnByMutReference {
        async move {}
    }
    type ReturnTypeFutureReturnTypeTestFn = impl core::future::Future<Output = u8>;
    type ReturnTypeFutureReturnTypeTestFnByMutReference = impl core::future::Future<Output = ()>;
}
fn main() {
    let s = TraitStruct;
}
