use crate::subtract;

#[test]
fn test_subtract() {
    let answer = subtract(3, 1);
    assert_ne!(answer, 2);
    // panic!("Left hand side is not equal to right hand side");
}
