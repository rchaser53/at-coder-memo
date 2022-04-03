/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering::*;


fn main() {
  input! {
    n:usize,
  }

  let mut result = usize::max_value();
  for a in 0usize.. {
    let aaa = a.pow(3);
    if n < aaa {
      break
    }

    let mut min = 0;
    let mut max = 10usize.pow(6) + 1;

    while min + 1 < max {
      let b = (min + max) / 2;
      let aab = a * a * b;
      let abb = a * b * b;
      let bbb = b.pow(3);
      let v = aaa + aab + abb + bbb;

      if v < n {
        min = b;
      } else {
        max = b;
      }
    }

    let aab = a * a * min;
    let abb = a * min * min;
    let bbb = min.pow(3);
    let v = aaa + aab + abb + bbb;
    if n <= v {
      result = std::cmp::min(v, result);  
    }

    let aab = a * a * max;
    let abb = a * max * max;
    let bbb = max.pow(3);
    let v = aaa + aab + abb + bbb;
    result = std::cmp::min(v, result);
  }
  println!("{}", result);
  
}