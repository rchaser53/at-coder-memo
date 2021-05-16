/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
    input! {
      n:usize,
      vals:[Chars;n]
    }
    let mut set = HashSet::new();
    set.insert(vals[0].clone());
    let mut last = vals[0][vals[0].len()-1];
    for i in 1..n {
      if last != vals[i][0] || set.contains(&vals[i]) {
        println!("No");
        return
      }
      last = vals[i][vals[i].len()-1];
      set.insert(vals[i].clone());
    }
    println!("Yes");
}
