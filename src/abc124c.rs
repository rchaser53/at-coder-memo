use proconio::input;
use proconio::marker::*;

fn main() {
  input! {
    s: String,
  }
  let s = s.chars().collect::<Vec<char>>();
  
  let mut b_count = 0;
  let mut c_count = 0;
  
  for (i, c) in s.iter().enumerate() {
    let c = *c;
    if i % 2 == 0 && c == '0' {
      b_count += 1;
    } else if i % 2 == 1 && c == '1' {
      b_count += 1;
    }
  }
  
  for (i, c) in s.iter().enumerate() {
    let c = *c;
    if i % 2 == 0 && c == '1' {
      c_count += 1;
    } else if i % 2 == 1 && c == '0' {
      c_count += 1;
    }
  }
  
  if c_count < b_count {
    println!("{}", c_count);
  } else {
    println!("{}", b_count);
  }
}

fn sol2() {
  input! {
    s:Chars
  }
  let n = s.len();
  let mut dp = vec![vec![0;2];n];
  
  if s[0] == '0' {
    dp[0][1] = 1;
  } else {
    dp[0][0] = 1;
  }

  let mut last = s[0];
  for i in 1..s.len() {
    if s[i] == '0' {
      dp[i][1] = dp[i-1][0]+1;
      dp[i][0] = dp[i-1][1];
    } else {
      dp[i][1] = dp[i-1][0];
      dp[i][0] = dp[i-1][1]+1;
    }
  }
  
  println!("{}", std::cmp::min(dp[n-1][0], dp[n-1][1]));
}
