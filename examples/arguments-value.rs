trait Trait {
    type ReturnTypeTestArgVal: core::future::Future;

    fn test_fn_val(&self, a: u8) -> Self::ReturnTypeTestArgVal;

    type ReturnTypeTestArgRef: core::future::Future;

    fn test_fn_mut(&self, a: &u8) -> Self::ReturnTypeTestArgRef;
}

fn main() {}
