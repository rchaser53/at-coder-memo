use proconio::input;

const MOD: isize = 1_000_000_007;
fn main() {
  input! {
    n: usize,
  }
  
  let mut dp = vec![
    vec![
      vec![
        vec![0;4];4
      ];4];
  n+1];
  dp[0][3][3][3] = 1;
  
  for len in 0..n {
    for c1 in 0..4 {
      for c2 in 0..4 {
        for c3 in 0..4 {
          for next in 0..4 {
            // c3_c2_c1_next
            if c2 == 0 && c1 == 1 && next == 2 { continue }
            if c2 == 1 && c1 == 0 && next == 2 { continue }
            if c2 == 0 && c1 == 2 && next == 1 { continue }
            if c3 == 0 && c2 == 1 && next == 2 { continue }
            if c3 == 0 && c1 == 1 && next == 2 { continue }
            
            dp[len+1][c2][c1][next] += dp[len][c3][c2][c1];
            dp[len+1][c2][c1][next] %= MOD;
          }
        }
      }
    }
  }
  
  let mut result = 0;
  for c1 in 0..4 {
    for c2 in 0..4 {
      for c3 in 0..4 {
        result += dp[n][c3][c2][c1];
        result %= MOD;
      }
    }
  }
  println!("{}", result);  
}