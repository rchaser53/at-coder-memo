use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn create_pascal() -> Vec<Vec<usize>> {
  let n = 100;
  let mut result: Vec<Vec<usize>> = vec![vec![0;n+1];n+1];
  result[0][0] = 1;
  for i in 0..n {
    for ii in 0..n {
      result[i+1][ii] = result[i+1][ii] + result[i][ii];
      result[i+1][ii+1] = result[i+1][ii+1] + result[i][ii];
    } 
  }
  result
}

pub fn main() {
  input! {
    n:usize,
    a:usize,
    b:usize,
    mut vals:[usize;n]
  }

  vals.sort();
  vals.reverse();

  let mut dict = HashMap::new();
  for &v in &vals {
    *dict.entry(v).or_insert(0) += 1;
  }

  let mut map = HashMap::new();
  let mut tot = 0;
  for i in 0..a {
    let v = vals[i];
    tot += v;
    *map.entry(v).or_insert(0) += 1;
  }
  let result_val = tot as f64 / a as f64;

  let mut memo = vec![];
  memo.push(map.clone());
  for i in a..b {
    let v = vals[i];
    let lv = tot * (i + 1);
    tot += v;
    let rv = tot * i;
    if lv != rv {
      break
    }
    
    *map.entry(v).or_insert(0) += 1;
    memo.push(map.clone());
  }

  let mut result_pattern = 0usize;
  let p_data = create_pascal();
  for m_map in memo {
    let mut base = 1;
    for (key, val) in m_map {
      let pv = *dict.get(&key).unwrap();
      base *= p_data[pv][val];
    }
    result_pattern += base;
  }

  println!("{}", result_val);
  println!("{}", result_pattern);
}