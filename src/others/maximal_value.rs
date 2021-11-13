use proconio::input;

fn main() {
  input! {
    n: usize,
    bs: [usize;n-1]
  }
  
  let mut sum = bs[0];
  for i in 0..bs.len() - 1 {
    sum += std::cmp::min(bs[i], bs[i+1]);
  }
  sum += bs[bs.len()-1];
  
  println!("{}", sum);
}