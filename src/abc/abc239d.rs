/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

// エラトステネスのふるい (変形でない)
fn sieve(a:usize) -> HashSet<usize> {
  let mut is_prime = vec![true;a+1];
  let mut set = HashSet::new();
  for i in 2..=a {
    if is_prime[i] {
      is_prime[i] = false;
      set.insert(i);
      for j in 2..=a/i {
        is_prime[i * j] = false;
      }
    }
  }
  set
}

fn main() {
  input! {
    a:usize,
    b:usize,
    c:usize,
    d:usize
  }

  let set = sieve(210);
  for i in a..=b {
    let mut temp = true;
    for j in c..=d {
      let v = i + j;
      if set.contains(&v) {
        temp = false;
        break
      }
    }

    if temp {
      println!("Takahashi");
      return
    }
  }
  println!("Aoki");
}