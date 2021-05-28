/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn culc(
  a:usize,
  b:usize
) -> String {
  let mut an = a.to_string().chars().collect::<Vec<char>>();
  let mut bn = b.to_string().chars().collect::<Vec<char>>();

  an.reverse();
  bn.reverse();
  let anlen = an.len();
  let bnlen = bn.len();
  let mut result = vec![0;anlen+bnlen+1];

  for i in 0..bnlen {
    let c1 = bn[i] as usize - '0' as usize;
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
  rs
}

pub fn main(
) {
  input! {
    n:usize,
    mut vals:[usize;n]
  }
  vals.sort();
  
  let mut result = 1;
  for v in vals {
    if v == 0 {
      println!("0");
      return
    }
    let nv = culc(result, v);

    if 19 <= nv.len() && 10usize.pow(18).to_string() != nv {
      println!("-1");
      return
    }
    result = nv.parse::<usize>().unwrap();
  }
  println!("{}", result);
}
