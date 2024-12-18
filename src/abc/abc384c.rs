/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    abcde: [usize;5]
  }

  let dict = vec!['A','B','C','D','E'];
  let limit = 1 << 5;
  let mut arr = vec![];
  for i in 1..limit {
    let mut temp = 0;
    let mut name = format!("");
    for j in 0..5 {
      if i >> j & 1 == 1 {
        temp += abcde[j];
        name = format!("{}{}", name, dict[j]);
      }
    }
    arr.push((temp, name));
  }

  arr.sort_by(|a,b| {
    let v = a.0.cmp(&b.0);
    if v == Ordering::Equal {
      a.1.cmp(&b.1)
    } else if v == Ordering::Less {
      Ordering::Greater
    } else {
      Ordering::Less
    }
  });

  for v in arr {
    println!("{}", v.1);
  }
}