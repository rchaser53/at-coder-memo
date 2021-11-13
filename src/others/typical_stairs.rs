use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
  input! {
    n: usize,
    m: usize,
    breaks: [usize;m]
  }

  let breaks = breaks
                .into_iter()
                .collect::<std::collections::HashSet<usize>>();
  let mut dp: Vec<usize> = vec![0;n+1];
  dp[0] = 1;
  if !breaks.contains(&1) {
    dp[1] = 1;
  }

  for i in 2..=n {
    if !breaks.contains(&i) {
      dp[i] = (dp[i-1] + dp[i-2]) % MOD;
    }
  }
  println!("{}", dp[n]);
}
