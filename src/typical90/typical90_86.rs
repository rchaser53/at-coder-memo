/* OUTPUT FILE */
use proconio::input;
use proconio::marker::*;
use std::collections::*;

const MOD:usize = 1_000_000_007;
pub fn main(
) {
  input! {
    n:usize,
    q:usize,
    vals:[(Usize1,Usize1,Usize1,usize);q]
  }

  let mut count = 1;
  let limit = 1 << n;
  for i in 0..60 {
    let bv = 1 << i;
    let mut temp = 0;
    for j in 0..limit {
      let mut flag = true;
      for &(x, y, z, v) in &vals {
        let xv = 1 << x;
        let yv = 1 << y;
        let zv = 1 << z;

        if v & bv == bv {
          if xv & j != xv && yv & j != yv && zv & j != zv {
            flag = false;
            break
          }
        } else {
          if xv & j == xv || yv & j == yv || zv & j == zv {
            flag = false;
            break
          }
        }
      }
      if flag {
        temp += 1;
      }
    }
    count *= temp;
    count %= MOD;
  }
  println!("{}", count);
}
