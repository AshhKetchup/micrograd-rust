use micrograd::value::ValueRef;

#[test]
fn multiplication_calculates_the_result() {
    let a = ValueRef::leaf(2);
    let b = ValueRef::leaf(3);
    let result = a * b;

    assert_eq!(result.data(), 6);
}
