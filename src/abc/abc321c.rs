/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    k:Usize1
  }

  let mut set = HashSet::new();
  let mut stack = vec![];
  for i in 0..10 {
    stack.push(i);
    set.insert(i);
  }

  while !stack.is_empty() {
    let mut new_stack = vec![];
    while let Some(v) = stack.pop() {
      let s = v.to_string().chars().collect::<Vec<char>>();
      let one = s[0] as usize - '0' as usize;
      let len = s.len();
      for ni in one+1..10 {
        let av = ni * 10usize.pow(len as u32);
        let nv = av + v;
        if !set.contains(&nv) {
          set.insert(nv);
          new_stack.push(nv);
        }
      }
    }
    stack = new_stack;
  }
  set.remove(&0);

  let mut arr = set.into_iter().collect::<Vec<usize>>();
  arr.sort();
  println!("{}", arr[k]);
}