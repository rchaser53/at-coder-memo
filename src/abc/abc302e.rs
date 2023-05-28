/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input!{
    n:usize,
    q:usize,
  }

  let mut g = vec![HashSet::new();n];
  let mut count = n;

  for _ in 0..q {
    input! {
      t: usize,
      a: Usize1,
    }

    if t == 1 {
      input! {
        b: Usize1
      }

      if g[a].is_empty() {
        count -= 1;
      }
      g[a].insert(b);

      if g[b].is_empty() {
        count -= 1;
      }
      g[b].insert(a);
    } else {
      let mut new_set = HashSet::new();
      if !g[a].is_empty() {
        std::mem::swap(&mut new_set, &mut g[a]);
        count += 1;
      }

      for b in new_set {
        if !g[b].is_empty() {
          g[b].remove(&a);
          if g[b].is_empty() {
            count += 1;
          }
        }
      }
    }

    println!("{}", count);
  }
}