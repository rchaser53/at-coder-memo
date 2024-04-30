/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn dfs(memo:&mut Vec<Vec<(usize,usize)>>, g:&Vec<Vec<usize>>, c:&Vec<usize>, ci:usize, li:usize) -> usize {
  let mut cv = c[ci];
  for &ni in &g[ci] {
    if ni == li { continue }
    let nv = dfs(memo, g, c, ni, ci);
    memo[ci].push((nv, ni));
    cv += nv;
  }
  memo[ci].sort();
  cv
}

fn main() {
  input! {
    n:usize,
    ab:[(Usize1,Usize1);n-1],
    c:[usize;n]
  }

  let mut g = vec![vec![];n];
  let mut memo = vec![vec![];n];
  for (a, b) in ab {
    g[a].push(b);
    g[b].push(a);
  }
  let inf = 10usize.pow(18);
  dfs(&mut memo, &g, &c, 0, inf);

  let mut ci = 0;
  let mut map = vec![];
  let mut seen = vec![false;n];
  loop {
    let mut ov = c[ci];
    if memo[ci].is_empty() {
      break
    }
    let mi = memo[ci].len()-1;
    for i in 0..mi {
      ov += memo[ci][i].0;
    }
    let (nv, ni) = memo[ci][mi];

    if ov < nv && !seen[ni] {
      seen[ni] = true;
      map.push(ni);
      ci = ni;
    } else {
      break
    }
  }

  let mut result = inf;
  map.reverse();
  for i in 0..3.min(map.len()) {
    let mut stack = vec![(map[i], 0, inf)];
    let mut temp = 0;
    while !stack.is_empty() {
      let mut new_stack = vec![];
      while let Some((ci,count,li)) = stack.pop() {
        for &ni in &g[ci] {
          if ni == li { continue }
          let n_count = count+1;
          temp += n_count * c[ni];
          new_stack.push((ni,n_count,ci));
        }
      }
      stack = new_stack;
    }
    result = result.min(temp);
  }
  println!("{}", result);
}