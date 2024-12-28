/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    a:isize,
    b:isize,
    c:isize,
    d:isize,
    e:isize,
    f:isize,

    g:isize,
    h:isize,
    i:isize,
    j:isize,
    k:isize,
    l:isize,
  }

  if j <= a || d <= g { 
    println!("No");
  } else if k <= b || e <= h {
    println!("No");
  } else if l <= c || f <= i {
    println!("No");
  } else {
    println!("Yes");
  }
}