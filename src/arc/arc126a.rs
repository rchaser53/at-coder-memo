use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input!{
    t:usize,
    vals:[(usize,usize,usize);t]
  }

  for (mut a, b, mut c) in vals {
    let mut result = 0usize;
    let mut bv = b / 2;

    if c < bv {
      result += c;
      bv -= c;
      
      let need_bv = bv * 2;
      if a < need_bv {
        result += a / 2;
      } else {
        result += bv;
        a -= need_bv;
        result += a / 5;
      }
    } else {
      result += bv;
      c -= bv;
      let cv = c / 2;
      let crv = c % 2;


      if a < cv {
        result += a;
      } else {
        result += cv;
        a -= cv;

        if crv == 1 && 3 <= a {
          result += 1;
          a -= 3;
        }
        result += a / 5;
      }
    }
    println!("{}", result);
  }
}