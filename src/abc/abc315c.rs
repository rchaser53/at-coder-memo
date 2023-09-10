/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    fs:[(usize,usize);n]
  }

  let mut map = HashMap::new();
  for (f, s) in fs {
    map.entry(f).or_insert(vec![]).push(s);
  }

  let mut arr = map.into_iter().map(|(_,v)| v).collect::<Vec<Vec<usize>>>();
  for i in 0..arr.len() {
    arr[i].sort();
  }
  arr.sort_by(|a,b| a[a.len()-1].cmp(&b[b.len()-1]));

  if arr.len() == 1 {
    let arr = &arr[arr.len()-1];
    println!("{}", arr[arr.len()-2] / 2 + arr[arr.len()-1]);
  } else {
    let arr1 = &arr[arr.len()-1];
    let arr2 = &arr[arr.len()-2];
    let mut result = arr1[arr1.len()-1] + arr2[arr2.len()-1];
    for i in 0..arr.len() {
      let temp = &arr[i];
      if temp.len() >= 2 {
        result = result.max(temp[temp.len()-1] + temp[temp.len()-2] / 2);
      }
    }
    println!("{}", result);
  }
}