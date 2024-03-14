/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    h:usize,
    w:usize,
    rows1:[Chars;h],
    rows2:[Chars;h],
  }

  for i in 0..h {
    for j in 0..w {
      let mut success = true;
      for y in 0..h {
        for x in 0..w {
          if rows1[(i+y)%h][(j+x)%w] != rows2[y][x] {
            success = false;
            break
          }
        } 
      }

      if success {
        println!("Yes");
        return
      }
    }
  }

  println!("No");
}