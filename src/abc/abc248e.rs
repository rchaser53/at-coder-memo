/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

fn main() {
  input! {
    n:usize,
    k:usize,
    mut xy:[(isize,isize);n]
  }

  if k == 1 {
    println!("Infinity");
    return
  }

  let mut flag = vec![vec![false;n];n];
  let mut result = 0;

  for i in 0..n {
    for j in i+1..n {
      flag[i][j] = true;
    }
  }

  let colinear = |a:usize, b:usize, c:usize| {
    let v1 = (xy[b].1 - xy[a].1) * (xy[c].0 - xy[a].0);
    let v2 = (xy[b].0 - xy[a].0) * (xy[c].1 - xy[a].1);
    v1 == v2
  };

  for i in 0..n {
    for j in i+1..n {
      if flag[i][j] {
        let mut count = 2;
        let mut list = vec![i,j];
        for ii in j+1..n {
          if colinear(i, j, ii) {
            count += 1;
            list.push(ii);
          }
        }
        for ii in 0..count {
          for jj in ii+1..count {
            flag[list[ii]][list[jj]] = false;
          }
        }
        if k <= count {
          result += 1;
        }
      }
    }
  }  

  println!("{}", result);
}