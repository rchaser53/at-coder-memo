/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::collections::*;
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;

fn main() {
  input! {
    x1:f64,
    y1:f64,
    x2:f64,
    y2:f64,
    x3:f64,
    y3:f64,
  }

  let ab = ((x2 - x1), (y2 - y1));
  let ac = ((x3 - x1), (y3 - y1));
  // 外積のベクトルの大きさは平行四辺形の面積になる。三角形なので1/2を求めれば良い
  // 二次元の場合、外積のベクトルは(0, 0, x1y2 - x2y1)になるが
  // z軸以外0なので単純に絶対値が面積になる
  println!("{}", (ab.0*ac.1 - ab.1*ac.0).abs() / 2.0);
}