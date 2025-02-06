/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    q:usize,
  }

  let mut memo = vec![1;n];
  let mut map = HashMap::new();
  for i in 0..n {
    map.insert(i,i);
  }

  let mut over_two = 0;
  for _ in 0..q {
    input! {
      t:usize
    }

    if t == 1 {
      input! {
        p:Usize1,
        h:Usize1
      }

      let ci = *map.get(&p).unwrap();
      map.insert(p, h);

      memo[ci] -= 1;
      if memo[ci] == 1 {
        over_two -= 1;
      }
      memo[h] += 1;
      if memo[h] == 2 {
        over_two += 1;
      }
    } else {
      println!("{over_two}");
    }
  }
}