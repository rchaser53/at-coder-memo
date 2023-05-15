/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input!{
    s:Chars,
    t:Chars,
  }

  let mut s_memo = vec![0;27];
  let mut t_memo = vec![0;27];

  let n = s.len();
  let at_index = 26;
  for i in 0..n {
    if s[i] == '@' {
      s_memo[at_index] += 1;
    } else {
      s_memo[s[i] as usize - 'a' as usize] += 1;
    }
    
    if t[i] == '@' {
      t_memo[at_index] += 1;
    } else {
      t_memo[t[i] as usize - 'a' as usize] += 1;
    }
  }
  
  let dict = "bfghijklmnpqsuvwxyz".chars().collect::<Vec<char>>();
  let dict2 = "acdeort".chars().collect::<Vec<char>>();
  for c in dict {
    let i = c as usize - 'a' as usize;
    if s_memo[i] != t_memo[i] {
      println!("No");
      return
    }
  }

  for c in dict2 {
    let i = c as usize - 'a' as usize;
    if s_memo[i] > t_memo[i] {
      let diff = s_memo[i] - t_memo[i];
      if t_memo[at_index] < diff {
        println!("No");
        return
      }
      t_memo[at_index] -= diff;
    } else if t_memo[i] > s_memo[i] {
      let diff = t_memo[i] - s_memo[i];
      if s_memo[at_index] < diff {
        println!("No");
        return
      }
      s_memo[at_index] -= diff;
    }
  }
  
  println!("Yes");
}