// GCJ Bribe the Prisoners(2009 ROund 1C C)

const MAX_Q: usize = 100;

fn main() {
  input! {
    p: usize,
    q: usize,
    mut a: [usize;MAX_Q+1],
  }
  let mut vals = vec![0];
  vals.append(&mut a);
  vals.push(p+1);
  let mut dp = vec![vec![0;MAX_Q+1];MAX_Q+2];
  
  for w in 2..=q+1 {
    let mut i = 0;
    while i + w <= q + 1 {
      let j = i + w;
      let mut t = usize::max_value();

      for k in i+1..j {
        t = std::cmp::min(t, dp[i][k] + dp[k][j]);
      }

      dp[i][j] = t + vals[j] - vals[i] - 2;
      i += 1;
    }
  }

  println!("{}", dp[0][q+1]);
}