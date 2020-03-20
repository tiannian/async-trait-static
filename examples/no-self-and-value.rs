trait Trait {
    type ReturnTypeTestFnValue: core::future::Future;

    fn test_fn_value(self) -> Self::ReturnTypeTestFnValue;

    type ReturnTypeTestFnNoSelf: core::future::Future;

    fn test_fn_no_self() -> Self::ReturnTypeTestFnNoSelf;
}

fn main() {}
