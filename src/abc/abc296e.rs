/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    a:[Usize1;n]
  }
  
  let mut seen = vec![false;n];
  let mut memo = vec![None;n];

  for i in 0..n {
    if seen[i] { continue }
    let mut map = HashMap::new();
    let mut ci = i;
    let mut count = 0;
    while count < 2*n {
      seen[ci] = true;
      count += 1;
      let entry = map.entry(ci).or_insert(0);
      *entry += 1;

      if 2 < *entry { break }
      ci = a[ci];
    }

    let mut temp = vec![];
    for (key, val) in map {
      if 2 <= val {
        temp.push(key);
      }
    }
    temp.sort();
    let max = temp[0];
    
    for key in temp {
      memo[key] = Some(max);
    }
  }

  let mut result = 0;
  for i in 0..n {
    if let Some(num) = memo[i] {
      if num <= i {
        result += 1;
      }
    }
  }

  println!("{}", result);
}