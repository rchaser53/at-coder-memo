use proconio::input;
use std::collections::HashMap;

fn gcd(a:usize, b:usize) -> usize {
  if a == 0 {
    return b
  }
  gcd(b % a, a)
}

fn factorize(mut n:usize) -> HashMap<usize, bool> {
  let base_n = n;
  let mut map: HashMap<usize, bool> = HashMap::new();
  let mut index = 2;
  while index * index <= base_n {
    while n % index == 0 {
      map.insert(index, true);
      n /= index;
    }
    index += 1;
  }
  if n != 1 {
    map.insert(n, true);
  }
  map
}

fn main() {
  input! {
    a: usize,
    b: usize
  }
  
  println!("{}", factorize(gcd(a,b)).keys().len() + 1);
}