/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
  }

  let mut memo = vec![];
  for _ in 0..n {
    input! {
      k:usize,
      a:[usize;k]
    }
    let mut map = HashMap::new();
    for i in 0..k {
      *map.entry(a[i]).or_insert(0) += 1;
    }
    memo.push((map, k));
  }

  let mut result = 0f64;
  for i in 0..n {
    let mut temp_map = HashMap::new();
    let k1 = memo[i].1 as f64;
    for (v, &num1) in &memo[i].0 {
      for j in 0..n {
        if i == j { continue }
        let k2 = memo[j].1 as f64;
        if let Some(&num2) = memo[j].0.get(&v) {
          let x = num1 as f64 / k1 * num2 as f64 / k2;
          // println!("v:{} x:{} i:{} j:{}", v, x, i, j);
          *temp_map.entry(j).or_insert(0f64) += x;
        }
      }
    }
    for (_, v) in temp_map {
      if result < v {
        result = v;
      }
    }
  }
  println!("{result}");
}