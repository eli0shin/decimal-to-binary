pub fn convert (n: i64, r: String) -> String {
  if n < 1 {
    return r;
  }

  convert(
    n / 2,
    format!("{}{}", (n % 2).to_string(), r),
  )
}