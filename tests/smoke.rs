#[proc_macro_test::hello]
fn wrapped_function() {}

mod private {
    proc_macro_test::make_pub! {
        static X: u32 = 1;
    }
}

pub static Y: u32 = private::X;

#[test]
fn works() {
    assert_eq!(wrapped_function(), 42);
}
