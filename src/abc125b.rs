use proconio::input;

fn main() {
  input! {
    n: usize,
    values:[usize;n],
    costs:[usize;n]
  }
  
  let mut max = 0;
  for i in 0..n {
    if costs[i] < values[i] {
      max += values[i] - costs[i];
    }
  }

  println!("{}", max);
}