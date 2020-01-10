#[proc_macro_test::hello]
fn wrapped_function() {}

/*proc_macro_test::make_pub! {
    static X: u32 = "foo";
}*/

#[test]
fn works() {
    assert_eq!(wrapped_function(), 42);
}
