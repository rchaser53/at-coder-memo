/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::collections::*;
use std::cmp::Reverse;
use std::cmp::*;

fn readln<T: std::str::FromStr>() -> T {
  let mut tmp = String::new();
  std::io::stdin().read_line(&mut tmp).ok();
  tmp.trim().parse().ok().unwrap()
}
fn readvec<T: std::str::FromStr>() -> Vec<T> {
  readln::<String>()
      .split_whitespace()
      .map(|x| x.parse().ok().unwrap())
      .collect()
}
fn readchars() -> Vec<char> {
  let mut tmp = String::new();
  std::io::stdin().read_line(&mut tmp).ok();
  let tmp:String = tmp.trim().parse().ok().unwrap();
  tmp.chars().into_iter().collect::<Vec<char>>()
}

fn main() {
  let a:Vec<usize> = readvec();
  let (h,w) = (a[0],a[1]);
  let a:Vec<usize> = readvec();
  let (x0,y0,x1,y1) = (a[0],a[1],a[2],a[3]);
  let mut arr = vec![];
  for _ in 0..h {
    arr.push(readchars());
  }

  let inf = 1_000_000;
  let mut memo = vec![vec![inf;w];h];
  memo[x0][y0] = 0;
  let mut stack = vec![(x0,y0,0)];
  while !stack.is_empty() {
    let mut new_stack = vec![];
    while let Some((x,y,cv)) = stack.pop() {
      let nv = cv + 1;
      if 0 < x && nv < memo[x-1][y] && arr[x-1][y] == 'W' {
        memo[x-1][y] = nv;
        new_stack.push((x-1,y,nv));
      }

      if x < h-1 && nv < memo[x+1][y] && arr[x+1][y] == 'W' {
        memo[x+1][y] = nv;
        new_stack.push((x+1,y,nv));
      }

      if 0 < y && nv < memo[x][y-1] && arr[x][y-1] == 'W' {
        memo[x][y-1] = nv;
        new_stack.push((x,y-1,nv));
      }

      if y < w-1 && nv < memo[x][y+1] && arr[x][y+1] == 'W' {
        memo[x][y+1] = nv;
        new_stack.push((x,y+1,nv));
      }
    }
    stack = new_stack;
  }
  println!("{}", memo[x1][y1]);
}