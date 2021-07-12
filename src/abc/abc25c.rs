/* OUTPUT FILE */
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn culc(
  memo: &mut Vec<Vec<Option<bool>>>,
  b: &Vec<Vec<usize>>,
  c: &Vec<Vec<usize>>,
  count: usize
) -> (usize, usize) {
  if count == 9 {
    let mut res1 = 0;
    let mut res2 = 0;
    for i in 0..2 {
      for j in 0..3 {
        if memo[i][j] == memo[i+1][j] {
          res1 += b[i][j];
        } else {
          res2 += b[i][j];
        }
      }
    }
    for i in 0..3 {
      for j in 0..2 {
        if memo[i][j] == memo[i][j+1] {
          res1 += c[i][j];
        } else {
          res2 += c[i][j];
        }
      }
    }

    return (res1, res2)
  }

  let mut res = (0, 0);
  for i in 0..9 {
    if let Some(_) = memo[i/3][i%3] { continue }
    memo[i/3][i%3] = Some(count % 2 == 0);
    let t = culc(memo, b, c, count + 1);
    memo[i/3][i%3] = None;

    if count % 2 == 0 && res.0 <= t.0 { res = t };
    if count % 2 == 1 && res.1 <= t.1 { res = t };
  }
  res
}

pub fn main(
) {
  input! {
    b: [[usize;3];2],
    c: [[usize;2];3]
  }
  
  let (bv, cv) = culc(&mut vec![vec![None;3];3], &b, &c, 0);
  println!("{}", bv);
  println!("{}", cv);
}
