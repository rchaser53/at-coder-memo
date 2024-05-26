/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    ab:[(usize,usize);n]
  }

  let mut map = HashMap::new();
  for (a,b) in ab {
    let entry = map.entry(a).or_insert((0,0));
    entry.0 += 1;
    let entry = map.entry(b).or_insert((0,0));
    entry.1 += 1;
  }

  let mut arr = map.into_iter().collect::<Vec<(usize, (usize,usize))>>();
  arr.sort();

  let mut result = 0usize;
  let mut now = 0;
  for (_,(p,m)) in arr {
    if p > 0 {
      result += now * p;
      result += p * (p-1) / 2;
      now += p;
    }
    now -= m;
  }
  println!("{}", result);
}