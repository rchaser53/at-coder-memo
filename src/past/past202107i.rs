/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

// https://atcoder.jp/contests/past202107-open/tasks/past202107_i
fn main() {
  input! {
    n:usize,
    x1:f64,
    y1:f64,
    x2:f64,
    y2:f64,
    ab:[(f64,f64);n]
  }

  // 回転前の右目を点A、左目を点Bとする

  // 問題文より点Aは(-e,0)、点Bは(e,0)。よって|AB|は2e
  // 回転や並行移動でベクトルの大きさは変わらないので
  // hypotを使って大きさを求めて2で割ることでeが求められる
  let e = (x1-x2).hypot(y1-y2) / 2.0;

  // ベクトルABから回転したradianをatan2で求める
  // 便利なことに座標から求められる
  // 回転した分を逆回転させてやりたいので-1をかける
  let radian = -(y2 - y1).atan2(x2 - x1);

  // ベクトルABを逆回転させてやることで
  // 並行移動しただけの座標を求める
  let rx = radian.cos() * (x2-x1) - radian.sin() * (y2-y1);
  let ry = radian.sin() * (x2-x1) + radian.cos() * (y2-y1);

  // 移動した分を戻す
  let rx = rx + x1;
  let ry = ry + y1;

  // 並行移動分を求める
  // e + dx = rx;
  // 0 + dy = ry;
  let dx = rx - e;
  let dy = ry;

  // それぞれのほくろに対して処理をしていく
  for (x, y) in ab {
    // 回転中心である点Aを原点Oに移動
    let x = x - x1;
    let y = y - y1;

    // radian分回転
    let rx = radian.cos() * x - radian.sin() * y;
    let ry = radian.sin() * x + radian.cos() * y;

    // 移動した分を戻す
    let rx = rx + x1;
    let ry = ry + y1;

    // 並行移動した分移動させてやる
    let rx = rx - dx;
    let ry = ry - dy;

    println!("{} {}", rx, ry);
  }
}