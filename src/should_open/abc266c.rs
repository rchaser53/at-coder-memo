/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;

 
fn main() { 
  input! { 
    ax:i64, 
    ay:i64, 
    bx:i64, 
    by:i64,
    cx:i64,
    cy:i64,
    dx:i64,
    dy:i64,
  }

  // counter clock wise (ccw) 反時計回り
  //         clock wise (cw)  時計回り
  // 2次元の外積
  // x1y2 - x2y1 
  // 外積のベクトルがZ軸に正か負かで座標の位置関係が時計回りか半時計周りかが分かる
  // >0のときccw、 <0の時cw
  // 具体例は https://atcoder.jp/contests/abc266/tasks/abc266_c に図があるので見ると良い
  let v1 = (bx-ax) * (dy-ay) - (by-ay) * (dx-ax) > 0; // ab ad
  let v2 = (cx-bx) * (ay-by) - (ax-bx) * (cy-by) > 0; // bc ba 
  let v3 = (dx-cx) * (by-cy) - (bx-cx) * (dy-cy) > 0; // cd cb 
  let v4 = (ax-dx) * (cy-dy) - (cx-dx) * (ay-dy) > 0; // da dc  
  if v1 && v2 && v3 && v4 {
    println!("Yes");
  } else if !v1 && !v2 && !v3 && !v4 {
    println!("Yes");
  } else {
    println!("No");
  }
}