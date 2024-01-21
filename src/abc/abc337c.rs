/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    a:[i32;n]
  }
  
  let mut map = HashMap::new();
  let mut stack = vec![];

  let mut now = 0;
  for i in 0..n {
    let ti = (i+1) as i32;
    let v = a[i];
    if v == -1 {
      stack.push(ti);
      now = ti;
    } else {
      map.insert(v, ti);
    }
  }

  while let Some(&ni) = map.get(&now) {
    stack.push(ni);
    now = ni;
  }
  
  println!("{}", stack.into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
}