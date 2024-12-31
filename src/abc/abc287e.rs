/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

#[derive(Clone, Debug)]
struct Tri {
  length:usize,
  children: Vec<Option<Tri>>,
  num: usize,
}

impl Tri {
  fn new(length:usize) -> Self {
    Tri {
      length,
      children:vec![None;26],
      num: 1,
    }
  }
}

fn main() {
  input! {
    n:usize,
    s:[Chars;n]
  }

  let mut result = vec![0;n];
  let mut root = Tri::new(0);
  for i in 0..n {
    let ss = &s[i];
    let mut tri_node = &mut root;
    for j in 0..ss.len() {
      let tj = ss[j] as usize - 'a' as usize;
      let next_length = j + 1;
      if tri_node.children[tj].is_none() {
        tri_node.children[tj] = Some(Tri::new(next_length));
      } else {
        tri_node.children[tj].as_mut().unwrap().num += 1;
      }
      tri_node = tri_node.children[tj].as_mut().unwrap();
    }
  }

  for i in 0..n {
    let ss = &s[i];
    let mut tri_node = &mut root;
    let mut max = 0;
    for j in 0..ss.len() {
      let tj = ss[j] as usize - 'a' as usize;
      if tri_node.children[tj].as_ref().unwrap().num > 1 {
        max = tri_node.children[tj].as_ref().unwrap().length;
      }
      tri_node = tri_node.children[tj].as_mut().unwrap();
    }
    result[i] = max;
  }
  
  for v in result {
    println!("{}", v);
  }
}