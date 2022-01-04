/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;

// n^pを計算するやつ
fn repeat_square(n:usize, p:usize, m:usize) -> usize {
  if p == 0 {
    1
  } else if p % 2 == 0 {
    repeat_square(n, p/2, m).pow(2) % m 
  } else {
    n * repeat_square(n, p-1, m) % m
  }
}

fn main() {
  input! {
    a:usize,
    b:usize,
    c:usize
  }

  // 9以下の値が4周期になる
  let bc = repeat_square(b, c, 4);
  // HELP ME
  // 周期に入る前を検討しないといけないらしいが
  // ここで何故4を足すと考慮したことになるかは分からない
  println!("{}", repeat_square(a%10, bc+4, 10));
}