/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    d:isize,
    a:[isize;n]
  }

  let mut map = BTreeMap::new();
  for v in a {
    *map.entry(v).or_insert(0) += 1;
  }
  
  let mut memo = vec![];
  while let Some((k, v)) = map.pop_first() {
    let mut arr = vec![(k, v)];
    for i in 1.. {
      let nd = d * i;
      if let Some((nk, nv)) = map.get_key_value(&(k + nd)) {
        arr.push((*nk, *nv));
        map.remove(&(k + nd));
      } else {
        break;
      }
    }
    memo.push(arr);
  }

  let mut result = 0;
  for i in 0..memo.len() {
    let m = memo[i].len();
    let mut ov = 0;
    let mut ev = 0;

    for j in 0..m {
      if j % 2 == 0 {
        ev += memo[i][j].1;
      } else {
        ov += memo[i][j].1;
      }
    }

    result += std::cmp::min(ov, ev);
  }
  println!("{result}");
}