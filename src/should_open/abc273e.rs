/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

// 永続スタック、永続データ構造
// 親のindexを所持する配列とmapを作ることで
// 論理削除のように非破壊的なpushやpopができる配列が作れる
// https://atcoder.jp/contests/abc273/tasks/abc273_e
fn main() {
  input! {
    q:usize,
  }

  let mut result = vec![0;q];
  let mut a = vec![];
  let mut cur = None;
  let mut map = HashMap::new();
  for i in 0..q {
    input! { s: String }
    match s.as_str() {
      "ADD" => {
        input! { x:usize }
        let parent = cur;
        cur = Some(a.len());
        a.push((parent, x));
      }
      "DELETE" => {
        if let Some(i) = cur {
          cur = a[i].0;
        }
      }
      "SAVE" => {
        input! { y: usize } 
        map.insert(y, cur);
      }
      // LOAD
      _ => {
        input! { z: usize }
        cur = *map.get(&z).unwrap_or(&None);
      }
    }
    if let Some(j) = cur {
      result[i] = a[j].1 as i32;
    } else {
      result[i] = -1;
    } 
  }
  println!("{}", result.into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
}