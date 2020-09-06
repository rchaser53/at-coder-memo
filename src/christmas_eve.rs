use proconio::{input, fastout};

#[fastout]
fn main() {
  input! {
    n: usize,
    k: usize,
    mut vals: [usize;n]
  }
  vals.sort();
  
  let mut min = usize::max_value();
  let mut left = 0;
  for i in (k-1)..n {
    min = std::cmp::min(min, vals[i] - vals[left]);
    left += 1;
  }
  println!("{}", min);
}