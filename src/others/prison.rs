use proconio::input;

fn main() {
  input! {
    n: usize,
    m: usize,
    gate: [(usize, usize);m]
  }
  
  let mut left = gate[0].0;
  let mut right = gate[0].1;
  
  for (l, r) in gate {
    left = std::cmp::max(left, l);
    right = std::cmp::min(right, r);
  }
  
  if right < left {
    println!("0");
  } else {
    println!("{}", right - left + 1);
  }
}