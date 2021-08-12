/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    n: usize,
    rows: [Chars;n]
  }

  let mut ques = vec![VecDeque::new();n];
  let mut result = 0;
  for i in 0..n {
    ques[i] = rows[i].iter().map(|v| *v).collect::<VecDeque<char>>();
  }

  for _ in 0..n {
    let mut flag = true;
    for j in 0..n {
      for k in 0..n {
        if ques[j][k] != ques[k][j] {
          flag = false;
          break
        }
      }
      if !flag { break }
    }
    if flag {
      result += n;
    }

    for j in 0..n {
      let c = ques[j].pop_front().unwrap();
      ques[j].push_back(c);
    }
  }
  println!("{}", result);
}
