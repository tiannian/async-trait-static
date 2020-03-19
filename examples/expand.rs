#![feature(prelude_import)]
#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

async fn hello() -> u8 {
    1
}

trait Er {}

struct Eri {}

impl Er for Eri {}

trait MyTrait {
    fn test_fn(&self, u: u8) -> Self::ReturnTypeFutureReturnTypeTestFn<'_>;
    fn test_fn_no_async(&self) -> u8;
    fn test_fn_by_mut_reference(
        &self,
        s: impl Er,
    ) -> Self::ReturnTypeFutureReturnTypeTestFnByMutReference<'_>;
    type ReturnTypeFutureReturnTypeTestFn<'a>: core::future::Future<Output = u8>;
    type ReturnTypeFutureReturnTypeTestFnByMutReference<'a>: core::future::Future<Output = u8>;
}
struct TraitStruct;
impl MyTrait for TraitStruct {
    fn test_fn(&self, u: u8) -> Self::ReturnTypeFutureReturnTypeTestFn<'_> {
        async move { hello().await }
    }
    fn test_fn_no_async(&self) -> u8 {
        1
    }
    fn test_fn_by_mut_reference(
        &self,
        s: impl Er,
    ) -> Self::ReturnTypeFutureReturnTypeTestFnByMutReference<'_> {
        // async move { self.test_fn(1).await }
        async fn _inner<T: Er>(_self: &TraitStruct, e: T) -> u8 {
            1
        }
        _inner(self, s)
    }
    type ReturnTypeFutureReturnTypeTestFn<'a> = impl core::future::Future<Output = u8>;
    type ReturnTypeFutureReturnTypeTestFnByMutReference<'a> =
        impl core::future::Future<Output = u8>;
}
fn main() {
    let s = TraitStruct;
}
