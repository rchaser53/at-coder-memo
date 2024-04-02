/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    a:usize,
    b:usize,
    mut d:[usize;n]
  }

  if n == 1 {
    println!("Yes");
    return
  }

  let ab = a + b;
  let mut d = d.into_iter().map(|v| v % ab).collect::<Vec<usize>>();
  d.sort();
  let mut dd = d.clone();

  for i in 0..n {
    dd.push(d[i] + ab);
  }

  for i in 0..n {
    let s = dd[i];
    let g = dd[i+n-1];
    if g - s < a {
      println!("Yes");
      return
    }
  }
  println!("No");
  
}