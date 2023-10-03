/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    k:usize,
    p:usize,
    ca:[[usize;k+1];n]
  }

  let default = 10usize.pow(15);
  let mut map = HashMap::new();

  map.insert(vec![0;k], 0usize);
  for arr in ca {
    let add_cost = arr[0];
    
    let mut new_map: HashMap<Vec<usize>,usize> = HashMap::new();
    for (mut status, cost) in map {
      let entry = new_map.entry(status.clone()).or_insert(default);
      if cost < *entry {
        *entry = cost;
      }

      for i in 1..arr.len() {
        status[i-1] = p.min(status[i-1]+arr[i]);
      }
      let new_cost = add_cost + cost;

      let entry = new_map.entry(status).or_insert(default);
      if new_cost < *entry {
        *entry = new_cost;
      }
    }
    map = new_map;
  }

  if let Some(v) = map.get(&vec![p;k]) {
    println!("{}", v);
  } else {
    println!("-1");
  }
}