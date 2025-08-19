/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    rc:[(isize,isize);n]
  }

  let xmin = rc.iter().map(|&(x,_)| x).min().unwrap();
  let xmax = rc.iter().map(|&(x,_)| x).max().unwrap();
  let ymin = rc.iter().map(|&(_,y)| y).min().unwrap();
  let ymax = rc.iter().map(|&(_,y)| y).max().unwrap();

  let x0 = (xmin+xmax)/2;
  let y0 = (ymin+ymax)/2;
  println!("{}", rc.iter()
  .map(|&(_,y)| y.abs_diff(y0))
  .chain(rc.iter().map(|&(x,_)| x.abs_diff(x0)))
  .max().unwrap()
  );
}