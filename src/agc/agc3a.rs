/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    s:Chars
  }

  let mut sn = vec![0,0];
  let mut we = vec![0,0];

  for c in s {
    match c {
      'W' => we[0] += 1,
      'E' => we[1] += 1,
      'S' => sn[0] += 1,
      _ => sn[1] += 1
    }
  }

  if (we[0] == 0 && we[1] != 0) ||
    (we[1] == 0 && we[0] != 0) {
    println!("No");
    return
  }
  
  if (sn[0] == 0 && sn[1] != 0) ||
    (sn[1] == 0 && sn[0] != 0) {
    println!("No");
    return
  }


  println!("Yes");
}
