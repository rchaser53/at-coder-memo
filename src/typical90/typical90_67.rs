/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use proconio::{input};
use proconio::marker::*;
use std::collections::*;

const MOD:usize = 1_000_000_007;

pub fn main(
) {
  input! {
    mut n: Chars,
    k: usize
  }

  if n.len() == 1 && n[0] == '0' {
    println!("{}", 0);
    return
  }

  for _ in 0..k {
    let mut ten = 0;
    n.reverse();
    // 8 => 10
    for i in 0..n.len() {
      ten += n[i].to_string().parse::<usize>().unwrap() * 8usize.pow(i as u32);
    }

    let mut temp = String::from("");
    while 0 < ten {
      temp = format!("{}{}", temp, ((ten % 9).to_string()));
      ten /= 9;
    }

    let mut temp = temp.chars().collect::<Vec<char>>();

    for i in 0..temp.len() {
      if temp[i] == '8' {
        temp[i] = '5';
      }
    }
    temp.reverse();
    n = temp;
  }

  println!("{}", n.iter().map(|v| v.to_string()).collect::<String>());
}