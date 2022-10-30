/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

const MOD:usize = 998244353;
fn main() {
  input! {
    a:usize,
    b:usize,
    c:usize,
    d:usize,
    e:usize,
    f:usize,
  }

  let a = a % MOD;
  let b = b % MOD;
  let ab = a * b % MOD;
  let c = c % MOD;
  let abc = ab * c % MOD;

  let d = d % MOD;
  let e = e % MOD;
  let de = d * e % MOD;
  let f = f % MOD;
  let def = de * f % MOD;
  println!("{}", ((MOD + abc) - def) % MOD);
}