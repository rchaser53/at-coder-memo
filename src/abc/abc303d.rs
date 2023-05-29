/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    x:usize,
    y:usize,
    z:usize,
    s:Chars
  }

  // (A,a)
  let mut cv = (z,0);
  let mut stack = vec![(s[0], 1)];
  for i in 1..s.len() {
    let c = s[i];
    let li = stack.len()-1;
    if stack[li].0 == c {
      stack[li].1 += 1;
    } else {
      stack.push((c,1));
    }
  }

  for (c, num) in stack {
    if c == 'A' {
      let nv1 = cv.0 + x * num;     // A => A
      let nv2 = cv.0 + z + y * num; // A => a
      let nv3 = cv.1 + y * num;     // a => a
      let nv4 = cv.1 + z + x * num; // a => A
      cv.0 = nv1.min(nv4);
      cv.1 = nv2.min(nv3);
    } else {
      let nv1 = cv.0 + y * num;     // A => A
      let nv2 = cv.0 + z + x * num; // A => a
      let nv3 = cv.1 + x * num;     // a => a
      let nv4 = cv.1 + z + y * num; // a => A
      cv.0 = nv1.min(nv4);
      cv.1 = nv2.min(nv3);
    }
  }

  println!("{}", cv.0.min(cv.1));
}
