/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn helper(n:usize, ci:usize, ti:usize, block:usize) -> usize {
  if ti < ci {
    if ti < block && block < ci {
      n - ci + ti + 1
    } else {
      ci - ti
    }
  } else {
    if ci < block && block < ti {
      ci + 1 + n - ti
    } else {
      ti - ci
    }
  } 
}

fn main() {
  input! {
    n:Usize1,
    q:usize,
    ht:[(char,Usize1);q]
  }

  let mut result = 0;
  let mut l = 0;
  let mut r = 1;
  for (h, t) in ht {
    if h == 'L' {
      result += helper(n,l,t,r);
      l = t;
    } else {
      result += helper(n,r,t,l);
      r = t;
    }
  }
  println!("{result}");
}