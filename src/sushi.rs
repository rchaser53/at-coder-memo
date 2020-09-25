use proconio::input;
use proconio::marker::*;

fn helper(
  dp: &mut Vec<Vec<Vec<f64>>>,
  n: usize,
  c1: usize,
  c2: usize,
  c3: usize
) -> f64 {
  if dp[c1][c2][c3] > 0f64 {
    dp[c1][c2][c3]
  } else if c1 == 0 && c2 == 0 && c3 == 0 {
    0f64
  } else {
    let mut a = n as f64;
    if c1 > 0 {
      let cf1 = c1 as f64;
      a += helper(dp, n, c1-1, c2, c3) * cf1;
    }

    if c2 > 0 {
      let cf2 = c2 as f64;
      a += helper(dp, n, c1+1, c2-1, c3) * cf2;
    }

    if c3 > 0 {
      let cf3 = c3 as f64;
      a += helper(dp, n, c1, c2+1, c3-1) * cf3;
    }

    dp[c1][c2][c3] = a / ((c1+c2+c3) as f64);
    dp[c1][c2][c3]
  }
}

fn main() {
  input!{
    n: usize,
    vals: [Usize1;n]
  }
  
  let mut dp: Vec<Vec<Vec<f64>>> = vec![vec![vec![-1f64;n+1];n+1];n+1];
  let mut counts: Vec<usize> = vec![0;3];
  for v in vals {
    counts[v] += 1;
  }
  
  let result = helper(&mut dp, n, counts[0], counts[1], counts[2]);
  println!("{}", result);
}