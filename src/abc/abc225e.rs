/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;
use std::cmp::Ordering;

fn helper(a:&(usize,usize), b:&(usize,usize)) -> Ordering {
  let v = (a.0 * b.1).cmp(&(a.1 * b.0));
  if v == Ordering::Less {
    Ordering::Greater
  } else if v == Ordering::Greater {
    Ordering::Less
  } else {
    v
  }
}

fn main() {
  input!{
    n:usize,
    mut vals:[(usize,usize);n]
  }
  let mut points = vec![];
  for (x, y) in vals {
    points.push(((x, y-1), (x-1, y)));
  }
  points.sort_by(|a,b| helper(&a.1, &b.1));

  let mut result = 0;
  let mut now = (0, 0);
  for (from, to) in points {
    if helper(&now, &from) != Ordering::Greater {
      result += 1;
      now = to;
    }
  }
  println!("{}", result);
}