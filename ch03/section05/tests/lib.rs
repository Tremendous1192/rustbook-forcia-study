use section05::add;

#[test]
fn integration_test() {
    assert_eq!(3, add(1, 2));
}
