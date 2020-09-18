use proconio::input;

fn main() {
  input! {
    n: usize,
    vals: [usize; n]
  }
  
  let mut result = 0;
  for v in vals {
    result += v - 1;
  }
  println!("{}", result);
}