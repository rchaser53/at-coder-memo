#![allow(unused_imports)]
 
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::HashMap;
 
#[fastout]
fn main() {
  input! {
    s: String,
    t: String,
  }
  let mut s: Vec<char> = s.chars().collect();
  let mut t: Vec<char> = t.chars().collect();
  let mut amap: HashMap<char, char> = HashMap::new();
  let mut bmap: HashMap<char, char> = HashMap::new();
  for i in 0..s.len() {
    let sc = s[i];
    let tc = t[i]; 
    if let Some(v) = amap.get(&sc) {
      if v != &tc {
        println!("No");
        return
      }
    }
    if let Some(v) = bmap.get(&tc) {
      if v != &sc {
        println!("No");
        return
      }
    }
    amap.insert(sc, tc);
    bmap.insert(tc, sc);
  }
  println!("Yes");
}