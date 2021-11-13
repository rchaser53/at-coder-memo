use proconio::input;

fn helper(a: usize) -> usize {
  let cnt = (a + 1) / 2;
  let mut result = cnt % 2;
  if a % 2 == 0 {
    result ^ a
  } else {
    result
  }
}

fn main() {
  input! {
    a: usize,
    b: usize,
  }
  
  println!("{}", helper(b) ^ helper(a-1));
}