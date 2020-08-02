use proconio::input;

fn helper(v: usize) -> bool {
  if v <= 1 {
    return false
  }

  if v == 2 {
    return true
  }

  if v % 2 == 0 {
    return false
  }
  
  let mut index = 3;
  while index < v {
    if v % index == 0 {
      return false
    }
    index += 2;
  }

  true
}

fn main() {
  input! {
    x: usize
  }
  let max = 10usize.pow(5) + 10;
  
  for i in x..=max {
    if helper(i) {
      println!("{}", i);
      return
    }
  }
}