use std::collections::HashMap;
 
#[warn(non_snake_case)]
fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
  	s = s.trim().to_string();
  
    let mut cache: HashMap<&str, bool> = HashMap::new();
    
    if s.is_empty() {
      println!("NO");
      return
    }
  
    if helper(&s, &mut cache) {
      println!("YES");
    } else {
      println!("NO");
    }
}
 
const DICT: [(&str, usize); 4] = [
	("dream", 5), ("dreamer", 7), ("erase", 5), ("eraser", 6)
];
 
fn helper<'a>(input: &'a str, cache: &mut HashMap<&'a str, bool>) -> bool {
  	if let Some(flag) = cache.get(input) {
      return *flag
    }
  
  	if input.len() == 0 {
  		return true
    }
  
  	for (word, size) in DICT.iter() {
    	if input.starts_with(word) && helper(&input[*size..], cache) {
			return true
      	}
    }
  
  	cache.insert(input, false);
	false
}


#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;

const DREAM:[char;5] = ['d','r','e','a','m'];
const DREAMER:[char;7] = ['d','r','e','a','m','e','r'];
const ERASE:[char;5] = ['e','r','a','s','e'];
const ERASER:[char;6] = ['e','r','a','s','e','r'];

fn helper(
  input: &Vec<char>,
  len: usize,
  index: usize,
) -> bool {
  if len <= index {
    true
  } else {
    if input[index] == DREAM[0] {
      if len <= index+4 { return false }
      for i in 0..5 {
        if input[index+i] != DREAM[i] { return false }
      }
      if helper(input, len, index+5) {
        true
      } else if index+6 < len
        && input[index+5] == DREAMER[5]
        && input[index+6] == DREAMER[6]
      {
        helper(input, len, index+7)
      } else {
        false
      }
    } else if input[index] == ERASE[0] {
      if len <= index+4 { return false }
      for i in 0..5 {
        if input[index+i] != ERASE[i] { return false }
      }
      if helper(input, len, index+5) {
        true
      } else if index+5 < len
        && input[index+5] == ERASER[5]
      {
        helper(input, len, index+6)
      } else {
        false
      }
    } else {
      false
    }
  }
}

#[fastout]
fn main() {
  input!{
    s: Chars
  }
  if helper(&s, s.len(), 0) {
    println!("YES");
  } else {
    println!("NO");
  }
}