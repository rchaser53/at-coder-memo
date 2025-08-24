/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn rotate_chars_clockwise(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
  let n = matrix.len();
  if n == 0 {
    return vec![];
  }
  let m = matrix[0].len();  
  let mut rotated = vec![vec![' '; n]; m];
  
  for i in 0..n {
    for j in 0..m {
      rotated[j][n - 1 - i] = matrix[i][j];
    }
  }
  
  rotated
}

fn main() {
  input! {
    n:usize,
    mut s:[Chars;n],
    t:[Chars;n]
  }

  let mut result = 100000;
  for base in 0..5 {
    let mut temp = base;
    for i in 0..n {
      for j in 0..n {
        if s[i][j] != t[i][j] {
          temp += 1;
        }
      }
    }
    result = min(result, temp);
    s = rotate_chars_clockwise(&s);
  }

  println!("{}", result);
}