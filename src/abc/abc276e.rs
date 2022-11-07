/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn helper(rows:&Vec<Vec<char>>, seen:&mut Vec<Vec<bool>>, stack: &mut Vec<(usize,usize)>, i:usize, j:usize) {
  let n = rows.len();
  let m = rows[0].len();
  
  if 0 < i && rows[i-1][j] != '#' && !seen[i-1][j] {
    seen[i-1][j] = true;
    stack.push((i-1,j));
  }

  if i < n-1 && rows[i+1][j] != '#' && !seen[i+1][j] {
    seen[i+1][j] = true;
    stack.push((i+1,j));
  }

  if 0 < j && rows[i][j-1] != '#' && !seen[i][j-1] {
    seen[i][j-1] = true;
    stack.push((i,j-1));
  }

  if j < m-1 && rows[i][j+1] != '#' && !seen[i][j+1] {
    seen[i][j+1] = true;
    stack.push((i,j+1));
  }
}

fn init(rows:&Vec<Vec<char>>, stack: &mut Vec<(usize,usize)>, i:usize, j:usize) {
  let n = rows.len();
  let m = rows[0].len();
  
  if 0 < i && rows[i-1][j] != '#' {
    stack.push((i-1,j));
  }

  if i < n-1 && rows[i+1][j] != '#' {
    stack.push((i+1,j));
  }

  if 0 < j && rows[i][j-1] != '#' {
    stack.push((i,j-1));
  }

  if j < m-1 && rows[i][j+1] != '#' {
    stack.push((i,j+1));
  }
}

fn main() {
  input! {
    h:usize,
    w:usize,
    rows:[Chars;h]
  }

  let mut si = 0;
  let mut sj = 0;
  for i in 0..h {
    for j in 0..w {
      if rows[i][j] == 'S' {
        si = i;
        sj = j;
        break
      }
    }
  }

let mut starts = vec![];
init(&rows, &mut starts, si, sj);
// println!("{:?}", &starts);
for (x, y) in starts {
  let mut seen = vec![vec![false;w];h];
  let mut temp_stack = vec![];
  helper(&rows, &mut seen, &mut temp_stack, x, y);
  seen[si][sj] = false;
  seen[x][y] = true;

  let mut stack = vec![];
  for (nx,ny) in temp_stack {
    if si == nx && sj == ny {
      continue
    }
    stack.push((nx,ny));
  }

  // println!("{:?} x:{} y:{}", &stack, x, y);
  while !stack.is_empty() {
    let mut new_stack = vec![];
    while let Some((ci, cj)) = stack.pop() {
      helper(&rows, &mut seen, &mut new_stack, ci, cj);
    }
    stack = new_stack;
  }

  if seen[si][sj] {
    println!("Yes");
    return
  }
}
  println!("No");
}