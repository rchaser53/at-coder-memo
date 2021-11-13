use proconio::input;

fn main() {
  input! {
    n: usize,
    q: usize,
    s: String,
    lrs: [(usize,usize);q],
  }
  let s = String::from(" ") + &s;
  let s = s.chars().collect::<Vec<char>>();
  let mut dp = vec![0;n+1];
  for i in 1..=n {
    if s[i-1] == 'A' && s[i] == 'C' {
      dp[i] = dp[i-1] + 1;
    } else {
      dp[i] = dp[i-1];
    }
  }

  for (l, r) in lrs {
    println!("{}", dp[r] - dp[l]);  
  }
}