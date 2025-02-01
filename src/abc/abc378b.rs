/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    qr:[(usize,usize);n],
    q:usize,
    td:[(Usize1,usize);q]
  }

  for (t,d) in td {
    let (qa,ra) = qr[t]; 
    if d <= ra {
      println!("{ra}");
    } else {
      let mut v = (d-ra) / qa;
      if (d-ra) % qa != 0 {
        v += 1;
      }
      println!("{}", v * qa + ra);
    }
  }
}