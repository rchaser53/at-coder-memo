/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    mut tx:[(usize,usize);n]
  }

  let mut map = HashMap::new();
  let mut stack = vec![];
  let must_get = 1_000_000;
  let must_not_get = 2_000_000;
  for i in (0..n).rev() {
    let (t,x) = tx[i];
    if t == 1 {
      let entry = map.entry(x).or_insert(0);
      if *entry == 0 {
        stack.push(format!("0"));
        tx[i].0 = must_not_get;
      } else {
        *entry -= 1;
        stack.push(format!("1"));
        tx[i].0 = must_get;
      }
    } else {
      *map.entry(x).or_insert(0) += 1;
    }
  }

  for (_, num) in map {
    if num > 0 {
      println!("-1");
      return
    }
  }
  stack.reverse();
  
  let mut result = 0;
  let mut tot = 0;
  for (t, _) in tx {
    if t == must_get {
      tot += 1;
    } else if t == 2 {
      tot -= 1;
    }
    result = result.max(tot);
  }

  println!("{}", result);
  println!("{}", stack.join(" "));
}