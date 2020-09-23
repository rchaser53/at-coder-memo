use proconio::input;
use proconio::marker::*;

const MOD: isize = 1_000_000_007;

fn main() {
  input!{
    h: usize,
    w: usize,
    maze: [Chars;h]
  }
  
  let mut dp: Vec<Vec<isize>> = vec![vec![0;w];h];
  for c in 0..w {
    if maze[0][c] == '.' {
      dp[0][c] = 1;
    } else {
      break
    }
  }
  
  for r in 0..h {
    if maze[r][0] == '.' {
      dp[r][0] = 1;
    } else {
      break
    }
  }
  
  for r in 1..h {
    for c in 1..w {
      if maze[r][c] == '.' {
        dp[r][c] = (dp[r-1][c] + dp[r][c-1]) % MOD;
      } else {
        dp[r][c] = 0;
      }
    }
  }
  
  println!("{}", dp[h-1][w-1]);
}
