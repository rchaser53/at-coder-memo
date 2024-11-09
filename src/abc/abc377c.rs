/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    ab:[(isize,isize);m]
  }

  let p = [
    (2,1),
    (1,2),
    (-1,2),
    (-2,1),
    (-2,-1),
    (-1,-2),
    (1,-2),
    (2,-1)
  ];

  let ni = n as isize;
  let mut set = HashSet::new();
  for (a,b) in ab {
    set.insert((a,b));
    for &(i,j) in &p {
      let na = a+i;
      let nb = b+j;
      if na <= 0 || ni < na { continue }
      if nb <= 0 || ni < nb { continue }
      set.insert((na,nb));
    }
  }
  
  println!("{}", n * n - set.len());
}