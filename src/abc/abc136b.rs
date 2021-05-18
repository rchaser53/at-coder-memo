/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

pub fn main(
) {
    input! {
      n:usize,
    }

    let mut count = 0;
    for i in 1..=n {
      if i.to_string().len() % 2 == 1 {
        count += 1;
      }
    }
    println!("{}", count);
}
