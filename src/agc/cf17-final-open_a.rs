/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

const AKIHABARA:[char;9] = ['A','K','I','H','A','B','A','R','A'];
struct Helper {
  set: HashSet<Vec<char>>,
  flag: bool
}

impl Helper {
  fn helper(&mut self, s:Vec<char>) {

    if self.set.contains(&s) {
      return
    }

    if s.len() == 9 {
      for i in 0..AKIHABARA.len() {
        if s[i] != AKIHABARA[i] {
          self.set.insert(s);
          return
        }
      }
      self.flag = true;
      return
    }
  
    for i in 0..=s.len() {
      let mut new_s = s.clone();
      new_s.insert(i, 'A');
      self.helper(new_s);
    }

    self.set.insert(s);
  }
}



fn main() {
  input!{
    s:Chars
  }

  if 9 < s.len() {
    println!("NO");
    return
  }

  let mut helper = Helper { set:HashSet::new(), flag:false };
  helper.helper(s);
  if helper.flag {
    println!("YES");
  } else {
    println!("NO");
  }
  
}