use proconio::input;
 
fn main() {
  input! {
    n: usize
  }

  let mut a = 1;
  let mut b = n;
  let mut min = usize::max_value();
  while a <= b {
    if n % a == 0 {
      min = std::cmp::min(min, (a - 1) + (n / a - 1));
    }
    a += 1;
    b = n / a;
  }

  println!("{}", min);
}