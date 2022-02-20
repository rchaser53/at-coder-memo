/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering::*;

fn helper(memo: &mut Vec<Vec<bool>>, dict: &Vec<char>, bi:usize, x:usize ,y:usize) -> Option<Vec<(usize, usize)>> {
  if x + bi + 1 < 2 { return None }
  let nx = x + bi + 1 - 2;
  if 9 <= nx { return None }

  let mut result = vec![];
  for i in 0..dict.len() {
    if dict[i] == '.' { continue }
    if y + i + 1 < 2 { continue }
    let ny = y + i + 1 - 2;
    if 9 <= ny { continue }
    if !memo[nx][ny] {
      memo[nx][ny] = true;
      result.push((nx,ny));
    }
  }
  if result.is_empty() {
    None
  } else {
    Some(result)
  }
}

fn main() {
  input! {
    a:Usize1,
    b:Usize1,
    s1:Chars,
    s2:Chars,
    s3:Chars,
  }

  let mut memo = vec![vec![false;9];9];
  memo[a][b] = true;
  let mut stack = vec![(a,b)];
  
  while let Some((a,b)) = stack.pop() {
    if let Some(arr) = helper(&mut memo, &s1, 0, a, b) {
      for (x, y) in arr {
        stack.push((x,y));
      }
    }

    if let Some(arr) = helper(&mut memo, &s2, 1, a, b) {
      for (x, y) in arr {
        stack.push((x,y));
      }
    }

    if let Some(arr) = helper(&mut memo, &s3, 2, a, b) {
      for (x, y) in arr {
        stack.push((x,y));
      }
    }
  }

  let mut result = 0;
  for i in 0..9 {
    for j in 0..9 {
      if memo[i][j] {
        result += 1;
      }
    }
  }
  println!("{}", result);
}