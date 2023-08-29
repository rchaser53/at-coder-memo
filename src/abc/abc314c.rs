/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    s:Chars,
    c:[Usize1;n]
  }
  
  let mut map = HashMap::new();
  for i in 0..n {
    map.entry(c[i]).or_insert(vec![]).push(i);
  }

  let mut result = s.clone();
  for i in 0..m {
    if let Some(arr) = map.get(&i) {
      let len = arr.len();
      for j in 0..len {
        let ci = arr[j];
        let ni = arr[(j+1)%len];
        result[ni] = s[ci];
      }
    }
  }
  println!("{}", result.into_iter().map(|v| v.to_string()).collect::<String>());
}