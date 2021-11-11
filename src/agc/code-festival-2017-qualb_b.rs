/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;
use std::cmp::Ordering;

const MOD:isize = 1_000_000_007;
fn main() {
  input!{
    n:usize,
    ds:[usize;n],
    m:usize,
    ts:[usize;m]
  }

  let mut map = HashMap::new();
  for v in ds {
    *map.entry(v).or_insert(0) += 1;
  }

  for v in ts {
    if let Some(iv) = map.get_mut(&v) {
      if *iv == 0 {
        println!("NO");
        return
      }
      *iv -= 1;
    } else {
      println!("NO");
      return
    }
  }
  println!("YES");
}