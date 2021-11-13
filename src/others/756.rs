use proconio::input;
use std::collections::*;

fn prime_factorization(limit: usize) -> Vec<usize> {
  let mut n = limit;
  let mut i = 2;
  let mut result = vec![];
  while i * i <= limit {
    if n % i == 0 {
      result.push(i);
      n /= i;
    } else {
      i += 1;
    }
  }
  
  if 1 < n {
    result.push(n);
  }
  result
}

fn main() {
  input! {
    n: usize,
  }
  
  let mut dict = HashMap::new();
  for i in 1..=n {
    let arr = prime_factorization(i);
    for v in arr {
      let entry = dict.entry(v).or_insert(0);
      *entry += 1;
    }
  }
  dict.remove(&1);
  
  let mut over_4 = 0isize;
  let mut over_2 = 0isize;
  for (_, v) in dict.iter() {
    let v = *v;
    if 4 <= v {
      over_4 += 1;
    } else if 2 <= v {
      over_2 += 1;
    }
  }
  let sm_5_5_3 = over_4 * over_4.saturating_sub(1)
   / 2 * (over_2 + over_4.saturating_sub(2));
  
  let mut over_14 = 0isize;
  let mut over_4 = 0isize;
  for (_, v) in dict.iter() {
    let v = *v;
    if 14 <= v {
      over_14 += 1;
    } else if 4 <= v {
      over_4 += 1;
    }
  }
  let sm_15_5 = over_14 * (over_4 + over_14.saturating_sub(1));
  
  let mut over_24 = 0isize;
  let mut over_2 = 0isize;
  for (_, v) in dict.iter() {
    let v = *v;
    if 24 <= v {
      over_24 += 1;
    } else if 2 <= v {
      over_2 += 1;
    }
  }
  let sm_25_3 = over_24 * (over_2 + over_24.saturating_sub(1));
  
  let mut sm_74 = 0isize;
  for (_, v) in dict.iter() {
    let v = *v;
    if 74 <= v {
      sm_74 += 1;
    }
  }
  
  println!(
    "{}",
    sm_5_5_3 + sm_15_5 + sm_25_3 + sm_74
  );
}