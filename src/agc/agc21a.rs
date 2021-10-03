use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn helper(a:&Vec<char>) -> usize {
  let mut result = 0usize;
  for &v in a {
    result += v.to_digit(10).unwrap() as usize;
  }
  result
}

fn main() {
  input!{
    n:usize
  }
  if n < 10 {
    println!("{}", n);
    return
  }
  let n = n.to_string().chars().into_iter().collect::<Vec<char>>();
  let mut max = helper(&n);
  
  let len = n.len();
  for i in (0..len-1).rev() {
    if n[i] != '0' {
      let mut temp = 0usize;
      for j in 0..i {
        temp += n[j].to_digit(10).unwrap() as usize;
      }
      temp += (n[i] as u8 - 1 - '0' as u8) as usize;
      for _ in i+1..len {
        temp += 9;
      }
      max = std::cmp::max(max, temp);
    }
  }
  println!("{}", max);
}