/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    h:[usize;n]
  }

  let mut btset = BTreeSet::new();
  let mut result = vec![0;n];
  for i in (0..n).rev() {
    result[i] = btset.len();
    let v = h[i];
    loop {
      if btset.is_empty() {
        btset.insert(v);
        break
      }

      let min = *btset.iter().next().unwrap();
      if v < min {
        btset.insert(v);
        break
      }
      btset.remove(&min);
    }
  }

  println!("{}", result.into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
}