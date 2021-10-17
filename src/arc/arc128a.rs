use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    vals:[usize;n]
  }
  let mut stack = vec![];
  stack.push((vals[0], 0));
  let mut f = true;


  for i in 1..n {
    let v = vals[i];
    let li = stack.len() - 1;
    let (lv, _) = stack[li];

    if f {
      if lv <= v {
        stack[li] = (v, i);
      } else {
        stack.push((v, i));
        f = !f;
      }
    } else {
      if lv >= v {
        stack[li] = (v, i);
      } else {
        stack.push((v, i));
        f = !f;
      }
    }
  }

  if stack.len() % 2 == 1 {
    stack.pop();
  }

  let mut result = vec![String::from("0");n];
  for (_, i) in stack {
    result[i] = String::from("1");
  }

  println!("{}", result.join(" "));
}