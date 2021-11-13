use proconio::input;
fn main() {
  input! {
    n: usize,
    vals: [usize;n]
  }
  
  let mut result = 0;
  let (mut l, mut r, mut xor, mut sum) =  (0, 0, 0, 0);
  while l < n {
    while r < n && sum ^ vals[r] == sum + vals[r] {
      sum += vals[r];
      r += 1;
    }
    result += r - l;
    if l == r {
      r += 1;
    } else {
      sum -= vals[l];
    }
    l += 1;
  }
  
  println!("{}", result);
}