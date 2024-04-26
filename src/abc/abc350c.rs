/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    mut a: [usize;n]
  }

  let mut result = vec![];
  let mut map = HashMap::new();

  for i in 0..n {
    map.insert(a[i], i);
  }

  for i in 0..n {
    let cv = i + 1;
    if a[i] == cv { continue }
    let av = a[i];
    let ti = *map.get(&cv).unwrap();

    result.push((i+1, ti+1));
    a.swap(i,ti);
    map.insert(av, ti);
    map.insert(cv, i);
  }

  println!("{}", result.len());
  for (a,b) in result {
    println!("{} {}", a, b);
  }
}