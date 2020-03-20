use std::pin::Pin;

trait Trait {
    type ReturnTypeTestFnValue: core::future::Future<Output = ()>;

    fn test_fn_value(self: Pin<&Self>) -> Self::ReturnTypeTestFnValue;

    type ReturnTypeTestFnRef: core::future::Future<Output = ()>;

    fn test_fn_ref(self: &Pin<&Self>) -> Self::ReturnTypeTestFnRef;
}

fn main() {}
