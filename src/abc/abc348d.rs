/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    h:usize,
    w:usize,
    a:[Chars;h],
    n:usize,
    rce:[(Usize1,Usize1,isize);n]
  }

  let mut memo: Vec<Vec<isize>> = vec![vec![-1;w];h];
  for &(r,c,e) in &rce {
    memo[r][c] = e;
  }
  let mut s = (0,0);
  let mut t = (0,0);
  for i in 0..h {
    for j in 0..w {
      if a[i][j] == 'S' {
        s = (i,j);
      } else if a[i][j] == 'T' {
        t = (i,j);
      }
    }
  }

  let mut seen = vec![vec![false;w];h];
  seen[s.0][s.1] = true;
  let mut stack: Vec<(usize,usize,isize)> = vec![(s.0, s.1, memo[s.0][s.1])];
  while !stack.is_empty() {
    let mut new_stack = vec![];
    while let Some((cx,cy,cv)) = stack.pop() {
      if cv <= 0 { continue }
      let nv = cv - 1;
      if 0 < cx && a[cx-1][cy] != '#' {
        if memo[cx-1][cy] < nv {
          new_stack.push((cx-1,cy,nv));
          memo[cx-1][cy] = nv;
        } else if !seen[cx-1][cy] {
          new_stack.push((cx-1,cy,memo[cx-1][cy]));
        }
        seen[cx-1][cy] = true;
      }

      if cx < h-1 && a[cx+1][cy] != '#' {
        if memo[cx+1][cy] < nv {
          new_stack.push((cx+1,cy,nv));
          memo[cx+1][cy] = nv;
        } else if !seen[cx+1][cy] {
          new_stack.push((cx+1,cy,memo[cx+1][cy]));
        }
        seen[cx+1][cy] = true;
      }

      if 0 < cy && a[cx][cy-1] != '#' {
        if memo[cx][cy-1] < nv {
          new_stack.push((cx,cy-1,nv));
          memo[cx][cy-1] = nv;
        } else if !seen[cx][cy-1] {
          new_stack.push((cx,cy-1,memo[cx][cy-1]));
        }
        seen[cx][cy-1] = true;
      }

      if cy < w-1 && a[cx][cy+1] != '#' {
        if memo[cx][cy+1] < nv {
          new_stack.push((cx,cy+1,nv));
          memo[cx][cy+1] = nv;
        } else if !seen[cx][cy+1] {
          new_stack.push((cx,cy+1,memo[cx][cy+1]));
        }
        seen[cx][cy+1] = true;
      }
    }
    stack = new_stack;
  }


  if memo[t.0][t.1] >= 0 && seen[t.0][t.1] {
    println!("Yes");
  } else {
    println!("No");
  }
}