/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

// a1 < b1
// b2 < a2
fn culc(
  a1:usize,
  a2:usize,
  b1:usize,
  b2:usize
) -> usize {
  let mut base = 1;
  let d1 = b1 - a1;
  let d2 = a2 - b2;

  if d2 < d1 { return 0 }
  
  base += (d1 / (d2 - d1)) * 2;
  if 0 < d1 % (d2 - d1) {
    base += 1;
  }
  base - 1
}

pub fn main(
) {
    input! {
      t1:usize,
      t2:usize,
      a1:usize,
      a2:usize,
      b1:usize,
      b2:usize
    }

    let a1v = t1 * a1;
    let a2v = t2 * a2;
    let b1v = t1 * b1;
    let b2v = t2 * b2;
    if a1v + a2v == b1v + b2v {
      println!("infinity");
      return
    }

    if a1v < b1v && a2v < b2v {
      println!("0");
      return
    } else if a1v > b1v && a2v > b2v {
      println!("0");
      return
    }

    let result = if a1v < b1v {
      culc(a1v, a2v, b1v, b2v)
    } else {
      culc(b1v, b2v, a1v, a2v)
    };
    
    println!("{}", result);
}
