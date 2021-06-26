/* OUTPUT FILE */
use proconio::input;
use proconio::marker::*;
use std::collections::*;

const MOD:usize = 1_000_000_007;

pub fn main(
) {
  input! {
    n:usize,
    mut vals:[(usize,usize,usize);n]
  }

  let mut count = 0;
  for i in 0..n {
    let (t1, l1, r1) = vals[i];
    for j in i+1..n {
      let (t2, l2, r2) = vals[j];
      
      if l1 <= l2 {
        if l2 < r1 {
          count += 1;
        } else if l2 == r1 {
          if t1 % 2 == 1 && t2 <= 2 {
            count += 1;
          }
        }
      } else if r2 <= r1 {
        if l1 < r2 {
          count += 1;
        } else if l1 == r2 {
          if t1 <= 2 && t2 % 2 == 1 {
            count += 1;
          }
        }
      } else if l2 <= l1 {
        if l1 < r2 {
          count += 1;
        } else if l1 == r2 {
          if t1 <= 2 && t2 % 2 == 1 {
            count += 1;
          }
        }
      } else if r1 <= r2 {
        if l2 < r1 {
          count += 1;
        } else if l2 == r1 {
          if t1 % 2 == 1 && t2 <= 2 {
            count += 1;
          }
        }
      }
    }
  }
  println!("{}", count);
}

pub fn sol2(
) {
  input! {
    n:usize,
    mut vals:[(usize,usize,usize);n]
  }

  vals.sort_by(|a,b|a.1.cmp(&b.1));
  let mut count = 0;
  for i in 0..n {
    let (t1, l1, r1) = vals[i];
    for j in i+1..n {
      let (t2, l2, r2) = vals[j];
      
      if l1 <= l2 {
        if l2 < r1 {
          count += 1;
        } else if l2 == r1 {
          if t1 % 2 == 1 && t2 <= 2 {
            count += 1;
          }
        }
      } else if r2 <= r1 {
        if l1 < r2 {
          count += 1;
        } else if l1 == r2 {
          if t1 <= 2 && t2 % 2 == 1 {
            count += 1;
          }
        }
      }
    }
  }
  println!("{}", count);
}
