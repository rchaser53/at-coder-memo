#[allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::HashSet;

fn main() {
  input! {
    s: Chars,
    k: usize,
  }
  let base = [
    'a', 'b', 'c', 'd', 'e',
    'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't',
    'u', 'v', 'w', 'x', 'y', 'z'
  ];
  
  let n = s.len();
  let mut set: HashSet<String> = HashSet::new();
  for c in base.iter() {
    for (i, cc) in s.iter().enumerate() {
      if c == cc {
        let mut ii = i+1;
        let mut temp = c.to_string();
        set.insert(temp.clone());
        while ii < n && ii < i + 5 {
          temp = temp + &s[ii].to_string();
          set.insert(temp.clone());
          ii += 1;
        }
      }
    }
    if 5 < set.len() { break }
  }
  
  let mut result = set.into_iter().collect::<Vec<String>>();
  result.sort();
  println!("{}", result[k-1]);
}