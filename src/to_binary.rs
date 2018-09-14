pub fn main (n: i64, r: String) -> String {
  if n < 1 {
    return r;
  }

  main(
    n / 2,
    format!("{}{}", (n % 2).to_string(), r),
  )
}