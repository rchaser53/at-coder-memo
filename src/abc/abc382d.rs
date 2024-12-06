/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn helper(result: &mut Vec<String>, a:&mut Vec<usize>, n:usize, m:usize) {
  let len = a.len();
  if len == n {
    result.push(a.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
    return
  }

  let last_val = a[len-1];

  for nv in last_val+10.. {
    let left_num = n - len - 1;
    let need = nv + left_num * 10;
    if m < need {
      break
    }

    a.push(nv);
    helper(result, a, n, m);
    a.pop();
  }
}

fn main() {
  input! {
    n:usize,
    m:usize,
  }

  let mut result = vec![];
  let mut arr = vec![];
  for i in 1.. {
    let left_num = i + (n-1) * 10;
    if m < left_num {
      break
    }
    arr.push(i);
    helper(&mut result, &mut arr, n, m);
    arr.pop();
  }

  println!("{}", result.len());
  for v in result {
    println!("{}", v);
  }
}