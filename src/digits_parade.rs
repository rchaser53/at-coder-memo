use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
  input! {
    s: String,
  }
  let n = 13;
  let mut chars = s.chars().collect::<Vec<char>>();
  let mut dp = vec![0;n];
  dp[0] = 1;

  let mut mul = 1;
  let mut index = chars.len() - 1;
  loop {
    let c = chars[index];
    let mut next_dp = vec![0;n];
    if c == '?' {
      for i in 0..10 {
        for ii in 0..n {
          next_dp[(i * mul + ii) % n] += dp[ii];
          next_dp[(i * mul + ii) % n] %= MOD;
        }
      }
    } else {
      let i = c.to_digit(10).unwrap() as usize;
      for ii in 0..n {
        next_dp[(i * mul + ii) % n] += dp[ii];
        next_dp[(i * mul + ii) % n] %= MOD;
      }
    }
    mul *= 10;
    mul %= n;
    dp = next_dp;
    
    if index == 0 {
      break
    }
    index -= 1;
  }
  
  println!("{}", dp[5]);
}