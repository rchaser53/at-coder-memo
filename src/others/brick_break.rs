use proconio::input;

fn main() {
  input! {
    n: usize,
    ai: [usize;n]
  }
  
  let len = ai.len();
  let mut count = 0;
  let mut val = 1;
  for v in ai {
    if val == v {
      val += 1;
    } else {
      count += 1;
    }
  }
  
  if len == count {
    println!("-1");
  } else {
    println!("{}", count);
  }
}