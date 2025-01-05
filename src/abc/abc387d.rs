/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn helper(rows:&Vec<Vec<char>>, c:char) -> (usize,usize) {
  let n = rows.len();
  let m = rows[0].len();
  for i in 0..n {
    for j in 0..m {
      if rows[i][j] == c {
        return (i,j)
      }
    }
  }

  unreachable!()
}

fn main() {
  input! {
    h:usize,
    w:usize,
    rows:[Chars;h]
  }

  let inf = 1_000_000_000;
  let mut memo = vec![vec![[inf,inf];w];h];

  let (si,sj) = helper(&rows, 'S');
  let (gi,gj) = helper(&rows, 'G');

  memo[si][sj] = [0usize,0usize];
  let mut stack = vec![(si,sj,0,true), (si,sj,0,false)];
  while !stack.is_empty() {
    let mut new_stack = vec![];
    while let Some((ci,cj,cv,p)) = stack.pop() {
      let nv = cv + 1;
      if p {
        if 0 < ci && rows[ci-1][cj] != '#' && nv < memo[ci-1][cj][1] {
          memo[ci-1][cj][1] = nv;
          new_stack.push((ci-1,cj,nv,false));
        }

        if ci < h-1 && rows[ci+1][cj] != '#' && nv < memo[ci+1][cj][1] {
          memo[ci+1][cj][1] = nv;
          new_stack.push((ci+1,cj,nv,false));
        }
      } else {
        if 0 < cj && rows[ci][cj-1] != '#' && nv < memo[ci][cj-1][0] {
          memo[ci][cj-1][0] = nv;
          new_stack.push((ci,cj-1,nv,true));
        }

        if cj < w-1 && rows[ci][cj+1] != '#' && nv < memo[ci][cj+1][0] {
          memo[ci][cj+1][0] = nv;
          new_stack.push((ci,cj+1,nv,true));
        }
      }
    }

    stack = new_stack;
  }
  
  let result = memo[gi][gj].iter().min().unwrap();
  if *result == inf {
    println!("-1");
  } else {
    println!("{result}");
  }
}