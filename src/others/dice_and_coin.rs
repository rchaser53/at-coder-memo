use proconio::input;

fn main() {
  input! {
    n: usize,
    k: usize,
  }
  
  let f_n = n as f64;
  let mut total = 0f64;
  for mut i in 1..=n {
    let mut count = 1f64;
    while i < k {
      i *= 2;
      count *= 2f64;
    }
    total += 1f64 / (f_n * count);
  }
  
  println!("{}", total);
}