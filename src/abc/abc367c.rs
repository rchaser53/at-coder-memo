/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn helper(c: &mut Vec<String>, l: &Vec<String>) {
  let n = l.len();
  for i in 0..n {
    let temp = format!("{}{}{}", l[i].clone(), l[i].clone(), l[i].clone());
    c.push(temp);
  }
}

fn main() {
  input! {
    n:usize,
  }

  let mut memo = vec![];
  let zero = vec![format!("#")];
  memo.push(zero);
  for i in 1..=6 {
    let mut current = vec![];
    let last = memo[i-1].clone();

    helper(&mut current, &last);

    let num = 3usize.pow((i-1) as u32);
    let center = vec![format!(".");num].into_iter().collect::<String>();

    let m = last.len();
    for j in 0..m {
      let temp = format!("{}{}{}", last[j].clone(),  center, last[j].clone());
      current.push(temp);
    }

    helper(&mut current, &last);
    memo.push(current);
  }

  for s in &memo[n] {
    println!("{}", s)
  }
}