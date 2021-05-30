/* OUTPUT FILE */
#![allow(unused_imports)]
use permutohedron::heap_recursive;
use proconio::input;
use proconio::marker::*;
use std::collections::*;

const MOD:usize = 1_000_000_007;
pub fn main(
) {
    input! {
      h:usize,
      w:usize,
      rs:Usize1,
      cs:Usize1,
      rg:Usize1,
      cg:Usize1,
      rows:[Chars;h]
    }
    
    let inf = 1_000_000_000_000usize;
    let limit = 1010;
    let dx = vec![1,0,!0,0];
    let dy = vec![0,1,0,!0];
    
    let mut dist = vec![vec![vec![inf;4];limit];limit];
    let mut que = VecDeque::new();
    for i in 0..4 {
      dist[rs][cs][i] = 0;
      que.push_back((rs, cs, i));
    }

    while let Some((y,x,d)) = que.pop_front() {
      for i in 0..4 {
        let tx = x + dx[i];
        let ty = y + dy[i];
        let mut cost = dist[y][x][d];
        if d != i {
          cost += 1;
        }
        if tx < w && ty < h &&
          rows[ty][tx] == '.' &&
          cost < dist[ty][tx][i] {
            dist[ty][tx][i] = cost;
            if d != i {
              que.push_back((ty,tx,i));
            } else {
              que.push_front((ty, tx, i));
            }
        }
      }
    }
    let mut result = inf;
    for i in 0..4 {
      result = std::cmp::min(dist[rg][cg][i], result);
    }
    println!("{}", result);
}
