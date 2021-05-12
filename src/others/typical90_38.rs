/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn gcv(a:usize, b:usize) -> usize { 
  if b == 0 {
    a
  } else {
    gcv(b, a % b)
  }
}

pub fn main(
) {
    input! {
      mut a:usize,
      mut b:usize
    }

    if b < a {
      let temp = a;
      a = b;
      b = temp;
    }

    let v = b / gcv(a, b);

    let mut vn = v.to_string().chars().collect::<Vec<char>>();
    let mut an = a.to_string().chars().collect::<Vec<char>>();

    vn.reverse();
    an.reverse();
    let vnlen = vn.len();
    let anlen = an.len();
    let mut result = vec![0;vnlen+anlen+1];

    for i in 0..vnlen {
      let c1 = vn[i] as usize - '0' as usize;
      for j in 0..anlen {
        let c2 = an[j] as usize - '0' as usize;
        result[i+j] += c1 * c2;
        for k in i+j..result.len()-1 {
          let v = result[k];
          let next = v / 10;
          let reminder = v % 10;
          result[k] = reminder;
          result[k+1] += next;
        }
      }
    }
    result.reverse();
    let rs = result.into_iter().map(|v| v.to_string()).collect::<String>();
    let rs = rs.trim_start_matches('0').to_string();

    if 10usize.pow(18).to_string() == rs {
      println!("{}", rs);
    } else if 19 <= rs.len() {
      println!("Large");
    } else {
      println!("{}", rs);
    }
}
