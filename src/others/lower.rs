use proconio::input;

fn main() {
  input! {
    n: usize,
    hs: [usize;n],
  }
  let mut result = 0;
  let mut max = 0;
  let mut last = 0;
  for v in hs {
    if last < v {
      result = std::cmp::max(max, result);
      max = 0;
    } else {
      max += 1;
    }
    last = v;
  }
  println!("{}", std::cmp::max(max, result)); 
}