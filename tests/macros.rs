macro_rules! assert_similar {
  ($left:expr, $right:expr) => {
    assert!(($left - $right).abs() < 0.0000001)
  };
}

#[test]
#[should_panic]
fn test_assert_similar_1() {
  assert_similar!(4_f64, 5_f64);
}

#[test]
#[should_panic]
fn test_assert_similar_2() {
  assert_similar!(6_f64, 6.0000001);
}

#[test]
fn test_assert_similar_3() {
  assert_similar!(6_f64, 6.00000009);
}
