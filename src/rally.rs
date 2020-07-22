use proconio::input;
 
fn main() {
  input! {
    N: i32,
    X: [i32;N as usize]
  }
  let mut total = 0;
  for v in X.iter() {
    total += v;
  }

  let average = total / N;
  let mut min = i32::max_value();
  min = std::cmp::min(min, helper(&X, average - 1));
  min = std::cmp::min(min, helper(&X, average));
  min = std::cmp::min(min, helper(&X, average + 1));

  println!("{}", min);
}
  
fn helper(arr: &[i32], val: i32) -> i32 {
  let mut total = 0;
  for v in arr.iter() {
    total += (v - val).pow(2);
  }
  total
}
