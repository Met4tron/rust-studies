fn main () {
  // Integers
  let sum = 4 + 4;
  assert_eq!(sum, 8);

  let sub = 10 - 5;
  assert_eq!(sub, 5);

  let div = 10 / 2;
  assert_eq!(div, 5);

  let mul = 10 * 10;
  assert_eq!(mul, 100);

  let percentage = (10 * 100) / 100;
  assert_eq!(percentage, 10);

  let percentage_float = (10.0 * 100.0) / 100.0;
  assert_eq!(percentage_float, 10.0);

  // Conversoes

  let i64_to_f64 = f64::from(1000);
  assert_eq!(i64_to_f64, 1000.0);

  let str_to_i32: i32 = "100".parse().unwrap();
  assert_eq!(str_to_i32, 100);

}