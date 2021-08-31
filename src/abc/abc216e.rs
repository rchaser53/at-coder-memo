use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input!{
    n:usize,
    mut k:usize,
    mut vals:[usize;n]
  }

  vals.sort();
  vals.reverse();
  let sum = vals.iter().sum::<usize>();
  if sum < k {
    let mut result = 0;
    for v in vals {
      result += (v + 1) * v / 2;
    }
    println!("{}", result);
    return
  }

  vals.push(0);
  let mut result = 0;
  for i in 1..=n {
    let tv = vals[i-1];
    let v = (tv - vals[i]) * i;
    if k <= v {
      let dv = k / i;
      let rv = k % i;
      let min = tv - dv + 1;
      result += (min + tv) * dv * i / 2;
      result += (min - 1) * rv;
      println!("{}", result);
      return
    } else {
      k -= v;
      let dv = v / i;
      let rv = v % i;
      let min = tv - dv + 1;
      result += (min + tv) * dv / 2 * i;
      result += (min - 1) * rv;
    }
  }
}