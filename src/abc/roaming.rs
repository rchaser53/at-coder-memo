use proconio::input;

const MOD: usize = 1_000_000_007;
const MAX: usize = 500_010;

fn main() {
  input! {
    n: usize,
    k: usize
  }
  
  let mut fact:Vec<usize> = vec![0;MAX];
  let mut finv:Vec<usize> = vec![0;MAX];
  let mut inv:Vec<usize> = vec![0;MAX];
  
  init(&mut fact, &mut finv, &mut inv);
  
  let mut answer = 0;
  for i in 0..std::cmp::min(k+1, n) {
    answer += com(&fact, &finv, n, i) * com(&fact, &finv, n-1, n-i-1) % MOD;
    answer %= MOD;
  }
  println!("{}", answer);
}

fn init(
  fact: &mut Vec<usize>,
  finv: &mut Vec<usize>,
  inv: &mut Vec<usize>
) {
  fact[0] = 1;
  fact[1] = 1;
  finv[0] = 1;
  finv[1] = 1;
  inv[0] = 0;
  inv[1] = 1;
  
  let mut i = 2;
  while i < MAX {
    fact[i] = fact[i-1] * i % MOD;
    inv[i] = MOD - inv[MOD % i] * (MOD / i) % MOD;
    finv[i] = finv[i-1] * inv[i] % MOD;
    i += 1;
  }
}

fn com(
  fact: &Vec<usize>,
  finv: &Vec<usize>,
  n: usize,
  k: usize
) -> usize {
  if n < k {
    0
  } else {
    fact[n] * (finv[k] * finv[n-k] % MOD) % MOD
  }
}