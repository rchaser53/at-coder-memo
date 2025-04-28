/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    q:usize,
  }
  let mut memo = vec![false;n];
  let mut map = HashMap::new();

  for i in 0..q {
    input! {
      t:usize
    }

    if t == 1 {
      input! {
        x:Usize1,
        y:Usize1
      }

      map.entry(x).or_insert(HashSet::new()).insert(y);

    } else if t == 2 {
      input! {
        x:Usize1
      }

      memo[x] = true;

    } else {
      input! {
        x:Usize1,
        y:Usize1
      }

      if memo[x] {
        println!("Yes");
      } else {
        if let Some(set) = map.get(&x) {
          if set.contains(&y) {
            println!("Yes");
          } else {
            println!("No");
          }
        } else {
          println!("No");
        }
      }
    }
  }
}