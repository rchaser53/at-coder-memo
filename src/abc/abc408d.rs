/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    t:usize,
  }

  for _ in 0..t {
    input! {
      n:usize,
      s:String,
    }

    let mut a = vec![0i64;n+1];
    let mut b = vec![0i64;n+1];
    let mut c = vec![0i64;n+1];
    
    for i in 0..n {
      let one = &s[i..i+1] == "1";
      a[i+1] = a[i];
      b[i+1] = b[i];
      if one {
        b[i+1] += 1;
      } else {
        a[i+1] += 1;
      }
      c[i+1] = a[i+1] - b[i+1];
    } 
    
    let mut result = 1i64 << 60;
    let mut temp = -1i64 << 60;
    for i in 0..n+1 {
      temp = std::cmp::max(temp, c[i]);
      result = std::cmp::min(result, c[i]-temp);
    }

    println!("{}", result+b[n]);
  }
}