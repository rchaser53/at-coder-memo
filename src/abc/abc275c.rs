/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    rows:[Chars;9]
  }

  let mut arr = vec![];
  for i in 0..9 {
    for j in 0..9 {
      if rows[i][j] == '#' {
        arr.push((i as i32, j as i32));
      }
    }
  }

  let mut result = HashSet::new();
  let n = arr.len();
  for i in 0..n {
    let (x1, y1) = arr[i];
    for j in i+1..n {
      let (x2, y2) = arr[j];
      let v12 = (x1-x2).pow(2) + (y1-y2).pow(2);
      for k in j+1..n {
        let (x3, y3) = arr[k];
        let v13 = (x1-x3).pow(2) + (y1-y3).pow(2);
        let v23 = (x2-x3).pow(2) + (y2-y3).pow(2);
        for l in k+1..n {
          let (x4, y4) = arr[l];
          let v14 = (x1-x4).pow(2) + (y1-y4).pow(2);
          let v24 = (x2-x4).pow(2) + (y2-y4).pow(2);
          let v34 = (x3-x4).pow(2) + (y3-y4).pow(2);
          
          let mut a = vec![v12,v13,v14,v23,v24,v34];
          a.sort();
          if a[0] == a[1] && a[1] == a[2] && a[2] == a[3]
            && a[3] * 2 == a[4] && a[4] == a[5] {
              result.insert((i,j,k,l));
          }
        }
      }
    }
  }

  println!("{}", result.len());
}