/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    a:[usize;n]
  }
  
  let mut lmap = HashMap::new();
  let mut rmap = HashMap::new();  
  for ci in 1..n {
    *rmap.entry(a[ci]).or_insert(0) += 1;
  }

  let mut result = 0;
  let mut temp = 0;
  *lmap.entry(a[0]).or_insert(0) += 1isize;
  if let Some(num) = rmap.get(&a[0]) {
    temp += num;
  }

  for i in 1..n-1 {
    let v = a[i];
    let l_entry = lmap.entry(v).or_insert(0);
    let r_entry = rmap.entry(v).or_insert(0);
    
    let now_val = *l_entry * *r_entry;
    result += temp - now_val;

    *l_entry += 1;
    *r_entry -= 1;
    let next_val = *l_entry * *r_entry;

    temp -= now_val;
    temp += next_val;
  }

  println!("{}", result);
}