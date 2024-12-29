/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    _n:usize,
    m:usize,
    mut xyc:[(usize,usize,char);m]
  }

  xyc.sort();
  let mut must_black = false;
  let mut max_y = 0;
  for i in (0..m).rev() {
    let (_x,y,c) = xyc[i];
    if c == 'B' {
      max_y = max_y.max(y);
      must_black = true;
    } else if must_black {
      if y < max_y {
        println!("No");
        return
      }
    }
  }

  xyc.sort_by(|a,b| a.1.cmp(&b.1));
  let mut must_black = false;
  let mut max_x = 0;
  for i in (0..m).rev() {
    let (x,_y,c) = xyc[i];
    if c == 'B' {
      max_x = max_x.max(x);
      must_black = true;
    } else if must_black {
      if x < max_x {
        println!("No");
        return
      }
    }
  }

  println!("Yes");
}