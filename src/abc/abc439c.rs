/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize
  }

  let mut memo = vec![];

  for i in 1..10usize.pow(4) {
    let v = i*i;
    if n < v {
      break;
    }
    memo.push(v);
  }

  let mut map = HashMap::new();
  let m = memo.len();
  for i in 0..m {
    for j in i+1..m {
      let v = memo[i] + memo[j];
      if n < v {
        break;
      }
      *map.entry(v).or_insert(0) += 1;
    }
  }

  let mut result = vec![];
  for (k, v) in map.iter() {
    if *v == 1 {
      result.push(*k);
    }
  }
  result.sort();

  println!("{}", result.len());
  println!("{}", result.into_iter().map(|v| format!("{}", v)).collect::<Vec<String>>().join(" "));  
}