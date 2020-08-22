use proconio::input;

const MOD: usize = 1_000_000_007;
fn create_pascal() -> Vec<Vec<usize>> {
  let n = 2000;
  let mut result: Vec<Vec<usize>> = vec![vec![0;n+1];n+1];
  result[0][0] = 1;
  for i in 0..n {
    for ii in 0..n {
      result[i+1][ii] = (result[i+1][ii] + result[i][ii]) % MOD;
      result[i+1][ii+1] = (result[i+1][ii+1] + result[i][ii]) % MOD;
    } 
  }
  result
}

fn main() {
  input! {
    n: usize,
    blue: usize,
  }

  let pascal = create_pascal();
  
  let culc = |n: usize, k: usize| {
    if n < k { return 0 }
    if n == 0 && k == 0 { return 1 }
    if k < 1 { return 0 }
    pascal[n - 1][k - 1]
  };
  
  let red = n - blue;
  for i in 1..=blue {
    let blue_p = culc(blue, i) % MOD;
    let mut red_p = culc(red, i-1) % MOD;
    red_p = (red_p + culc(red, i)) % MOD;
    red_p = (red_p + culc(red, i)) % MOD;
    red_p = (red_p + culc(red, i+1)) % MOD;
    println!("{}", blue_p * red_p % MOD);
  }
}