/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

fn main() {
  input! {
    n:usize,
    mut k:usize,
    queries:[(usize,isize);n]
  }

  let mut temp = 0isize;
  for i in 0..n {
    let (t, v) = queries[i];
    if t == 1 {
      temp = v;
    } else {
      temp += v;
    }
  }

  let mut tot = 0isize;
  let mut pheap = BinaryHeap::new();
  let mut result = temp;
  for i in (0..n).rev() {
    let (t, v) = queries[i];
    if t == 1 {

      result = std::cmp::max(result, v + tot);
      if k == 0 {
        println!("{}", result);
        return
      }

      k -= 1;

      if k < pheap.len() {
        if let Some(v) = pheap.pop() {
          tot += v;
        }
      }
      
      continue
    }

    if v < 0 && 0 < k {
      // 除外できる回数がまだある
      if pheap.len() < k {
        pheap.push(v);
        continue
      }

      // 除外している中で最大値と比較する
      if let Some(max_v) = pheap.pop() {
        if max_v < v {
          tot += v;
          pheap.push(max_v);
        } else {
          tot += max_v;
          pheap.push(v);
        }
      }
    } else {
      tot += v;
    }
  }
  result = std::cmp::max(result, tot);
  
  println!("{}", result);
}