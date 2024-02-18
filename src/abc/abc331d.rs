/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    q:usize,
    rows:[Chars;n],
    abcd:[(usize,usize,usize,usize);q]
  }

  let mut memo = vec![vec![0;n+1];n+1];
  for i in 0..n {
    for j in 0..n {
      let v = if rows[i][j] == 'B' {
        1
      } else {
        0
      };
      memo[i+1][j+1] = memo[i+1][j] + memo[i][j+1] + v - memo[i][j];
    }
  }

  for (a,b,c,d) in abcd {
    let x1 = a / n;
    let xr1 = a % n;
    let x2 = c / n;
    let xr2 = c % n;

    let y1 = b / n;
    let yr1 = b % n;
    let y2 = d / n;
    let yr2 = d % n;

    let dx = x2 - x1;
    let dy = y2 - y1;

    if dx == 0 {
      if dy == 0 {
        println!("{}", memo[xr2+1][yr2+1] - memo[xr2+1][yr1] - memo[xr1][yr2+1] + memo[xr1][yr1]);
      } else {
        let left = memo[xr2+1][n] - memo[xr2+1][yr1] - memo[xr1][n] + memo[xr1][yr1];
        let center = memo[xr2+1][n] - memo[xr1][n];
        let right = memo[xr2+1][yr2+1] - memo[xr1][yr2+1];
        let tot = left + right + center * (dy - 1); 
        println!("{}", tot);
      }
    } else {
      if dy == 0 {
        let top = memo[n][yr2+1] - memo[n][yr1] - memo[xr1][yr2+1] + memo[xr1][yr1];
        let center = memo[n][yr2+1] - memo[n][yr1];
        let bottom = memo[xr2+1][yr2+1] - memo[xr2+1][yr1];
        println!("{}", top + bottom + center * (dx - 1));
      } else {
        let lt = memo[n][n] - memo[n][yr1] - memo[xr1][n] + memo[xr1][yr1];
        let lc = (memo[n][n] - memo[n][yr1]) * (dx - 1);
        let lb = memo[xr2+1][n] - memo[xr2+1][yr1];

        let ct = (memo[n][n] - memo[xr1][n]) * (dy-1);
        let cc = memo[n][n] * (dx-1) * (dy-1);
        let cb = memo[xr2+1][n] * (dy-1);

        let rt = memo[n][yr2+1] - memo[xr1][yr2+1];
        let rc = memo[n][yr2+1] * (dx-1);
        let rb = memo[xr2+1][yr2+1];

        println!("{}", lt+lc+lb + ct+cc+cb + rt+rc+rb);
      }
    }
  }
}