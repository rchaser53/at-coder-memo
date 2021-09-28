use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input!{
    n:usize
  }

  let nc = n.to_string().chars().into_iter().collect::<Vec<char>>();
  let mut memo = vec![0;16];
  let mut tot = 0usize;
  for i in 0..16 {
    tot += 10usize.pow(i);
    memo[i as usize] = tot;
  }

  let nlen = nc.len()-1;
  let mut result = 0usize;
  for i in 0..nlen {
    result += memo[i];
  }
  
  for i in 0..nc.len() {
    if nc[i] == '1' {
      let v = nc[i+1..].iter().map(|v| v.to_string()).collect::<String>().parse::<usize>().unwrap_or(0);
      result += v + 1;
    } else {
      if nc[i] != '0' {
        result += memo[nc.len()-i-1];
      }
      break
    }
  }
  println!("{}", result);
}