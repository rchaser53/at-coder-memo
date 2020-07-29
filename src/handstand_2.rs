use proconio::input;
use std::collections::HashMap;

fn helper(mut n: usize) -> (usize, usize) {
  let a = n % 10;
  let mut b = 0;
  while 0 < n {
    b = n;
    n /= 10;
  }
  (a, b)
}

fn main() {
  input! {
    n: usize,
  }

  let mut map: HashMap<(usize, usize), usize> = HashMap::new();
  for i in 1..=n {
    let p = helper(i);
    let mut entry = map.entry(p).or_insert(0);
    *entry += 1;
  }

  let mut count = 0;
  for i in 1..=n {
    let (a, b) = helper(i);
    count += map.get(&(b,a)).unwrap_or(&0);
  }

  println!("{}", count);
}