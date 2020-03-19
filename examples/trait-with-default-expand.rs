#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

type ReturnTypeFutureReturnTypeTestFnDefault<'a> = impl core::future::Future<Output = u8>;
trait MyTrait {
    fn test_fn(&self) -> Self::ReturnTypeFutureReturnTypeTestFn<'_>;
    fn test_fn_default(&self) -> ReturnTypeFutureReturnTypeTestFnDefault<'_> {
        async fn _inner<AsyncTrait: MyTrait + ?Sized>(_self: &AsyncTrait) -> u8 {
            1
        }
        _inner(self)
    }
    type ReturnTypeFutureReturnTypeTestFn<'a>: core::future::Future<Output = u8>;
}
struct TraitStruct;
impl MyTrait for TraitStruct {
    fn test_fn(&self) -> Self::ReturnTypeFutureReturnTypeTestFn<'_> {
        async move { 1 }
    }
    type ReturnTypeFutureReturnTypeTestFn<'a> = impl core::future::Future<Output = u8>;
}
fn main() {
    let s = TraitStruct;
}

