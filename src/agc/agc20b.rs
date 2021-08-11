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
    k:usize,
    mut vals:[usize;k]
  }

  vals.reverse();

  if 2 != vals[0] {
    println!("-1");
    return
  }

  let mut min = 2;
  let mut max = 3;

  // minは前のmin以上の最低の値 => Aiの倍数
  // maxは前のmax以下の最大の値 => Aiの倍数+Aiの最大の余り
  for i in 1..k {
    let v = vals[i];
    if max < v {
      println!("-1");
      return
    }

    if min <= v {
      min = v;
    } else {
      let temp = min / v;
      if min % v != 0 {
        min = (temp+1) * v;
      } else {
        min = temp * v;
      }
    }
    
    let temp = max / v;
    max = (temp+1) * v - 1;
  }

  if max < min {
    println!("-1")
  } else {
    println!("{} {}", min, max);
  }

}
