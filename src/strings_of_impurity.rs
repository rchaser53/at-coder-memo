#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;

#[fastout]
fn main() {
  input! {
    s: Chars,
    t: Chars
  }
  
  let mut map: HashMap<char, Vec<usize>> = HashMap::new();
  for i in 0..s.len() {
    let c = s[i];
    let mut entry = map.entry(c).or_insert(vec![]);
    entry.push(i);
  }
  
  let mut count = 0;
  let mut index = 0;
  loop {
    let last_index = index;
    let mut s_index = 0;
    
    if let Some(vec) = map.get(&t[index]) {
      s_index = vec[0];
      index += 1;
    }
    
    let never_exsit_char = '0';
    while let Some(vec) = map.get(&t.get(index).unwrap_or(&never_exsit_char)) {
      match vec.binary_search(&s_index) {
        Ok(i) => {
          let target_index = i+1;
          if vec.len() <= target_index {
            break
          }
          s_index = vec[target_index];
          index += 1;
        },
        Err(i) => {
          if vec.len() <= i {
            break
          }
          s_index = vec[i];
          index += 1;
        }
      };
    }
    
    if index == last_index {
      println!("-1");
      return
    }
    
    if t.len() <= index {
      count += s_index + 1;
      break
    } else {
      count += s.len();
    }
  }

  println!("{}", count); 
}