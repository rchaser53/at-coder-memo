use proconio::input;

static MOD: i64 = 1000000007;
fn helper(n:i64, a: i64) -> i64 {
  let mut numerator = 1;
  let mut denominator = 1;
  for i in 0..a {
    numerator = numerator * (n-i) % MOD;
    denominator = denominator * (i + 1) % MOD;
  }
  return numerator * (mod_pow(denominator, MOD - 2) % MOD) % MOD;
}

fn mod_pow(mut a: i64, mut n: i64) -> i64 {
  let mut res = 1;
  while 0 < n {
    if n & 1 == 1 {
      res = res * a % MOD;
    }
    a = a * a % MOD;
    n >>= 1;
  }
  res
}

fn main() {
  input! {
    n: i64,
    a: i64,
    b: i64,
  }
  let mut result = mod_pow(2, n);
  result = (result - 1) % MOD;
  result = (result + MOD - helper(n, a)) % MOD;
  result = (result + MOD - helper(n, b)) % MOD;
  
  println!("{}", result);
}