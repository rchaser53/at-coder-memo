/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

pub fn main(
) {
    input! {
      n:usize,
    }

    let mut memo = vec![0;n+1];
    memo[0] = 2usize;
    memo[1] = 1;
    for i in 2..=n {
      memo[i] = memo[i-1] + memo[i-2];
    }

    println!("{}", memo[n]);
}
