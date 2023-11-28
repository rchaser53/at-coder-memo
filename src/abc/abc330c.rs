/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

type Target = isize;
type UseValue = isize;
fn lower_bound(arr: &Vec<Target>, x: &UseValue) -> usize {
  let mut low = 0;
  let mut high = arr.len();
  while low != high {
      let mid = (low + high) / 2;
      match arr[mid].cmp(x) {
          std::cmp::Ordering::Less => {
              low = mid + 1;
          }
          std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {
              high = mid;
          }
      }
  }
  low
}

fn main() {
  input! {
    d:isize
  }

  let mut arr = vec![];
  let mut i = 0;
  let limit = 3*10isize.pow(12);
  while i * i <= limit {
    arr.push(i*i);
    i += 1;
  }

  let mut result = limit;
  let n = arr.len();
  for &x in &arr {
    let diff = (x-d).abs();
    let ti = lower_bound(&arr, &diff);

    for i in ti.saturating_sub(3)..(ti+4).min(n) {
      let y = arr[i];
      result = result.min((x+y-d).abs());
    }
  }

  println!("{}", result);
}