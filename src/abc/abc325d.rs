/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Reverse;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    td:[(isize,isize);n]
  }

  let mut bteemap = BTreeMap::new();
  for (s, v) in td {
    let entry = bteemap.entry(s).or_insert(vec![]);
    entry.push(s+v);
  }

  let mut result = 0;
  let mut time = *bteemap.iter().next().unwrap().0;
  let mut heap = BinaryHeap::new();

  loop {
    if let Some(arr) = bteemap.remove(&time) {
      for v in arr {
        heap.push(Reverse(v));
      }
    } else {
      if heap.is_empty() {
        if let Some((next_time, _)) = bteemap.iter().next() {
          time = *next_time;
          continue
        } else {
          break
        }
      }
    }

    while !heap.is_empty() {
      let v = heap.pop().unwrap().0;
      if time <= v {
        result += 1;
        break
      }
    }
    time += 1;
  }

  println!("{}", result);
}