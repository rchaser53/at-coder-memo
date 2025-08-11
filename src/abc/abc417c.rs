/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    a:[isize;n]
  }

  let mut map1 = HashMap::new();
  let mut map2 = HashMap::new();
  for i in 0..n {
    let ai = i as isize + 1;

    *map1.entry((ai-a[i])).or_insert(0) += 1;
    *map2.entry((ai+a[i])).or_insert(0) += 1;
  }

  let mut result = 0usize;
  for (x, num) in map1.iter() {
    if let Some(&num2) = map2.get(x) {
      result += num * num2;
    }
  }

  println!("{}", result);
}