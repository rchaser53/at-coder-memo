use proconio::input;
use std::collections::HashMap;

const MOD:usize = 1_000_000_007;

// a^n mod を計算する
fn mod_pow(mut a:usize, mut n:usize) -> usize {
  let mut res = 1;
  while n > 0 {
    if n & 1 == 1 {
      res *= a;
      res %= MOD;
    }
    a *= a;
    a %= MOD;
    n >>= 1;
  }
  res
}

// a^{-1} mod を計算する
fn mod_inv(a:usize) -> usize {
  mod_pow(a, MOD-2)
}

fn prime_factorization(mut a:usize, primes:&[usize]) -> HashMap<usize, usize> {
  let mut factors = HashMap::new();
  for &prime in primes {
    if a % prime != 0 { continue }
    let mut exp = 1;
    while 0 < a {
      a /= prime;
      if a % prime != 0 { break }
      exp += 1;
    }
    factors.insert(prime, exp);
  }
  if 1 < a {
    factors.insert(a, 1);
  }
  factors
}

fn sieve(a: usize) -> Vec<usize> {
  let mut result:Vec<usize> = vec![];
  let mut is_prime = vec![true;a+1];
  for i in 2..a+1 {
    if is_prime[i] {
      result.push(i);
      for j in 1..a / i + 1 {
        is_prime[i * j] = false;
      }
    }
  }
  result
}

fn main() {
  input! {
    n: usize,
    vals: [usize;n]
  }
  
  let val_max = *vals.iter().max().unwrap();
  let primes = sieve((val_max as f64).sqrt().ceil() as usize);
  let mut values = vec![];
  
  for v in vals.iter() {
    let factors = prime_factorization(*v, &primes);
    values.push(factors);
  }
  let mut map:HashMap<usize,usize> = HashMap::new();
  
  for factors in values {
    for (k, v) in factors {
      let e = map.entry(k).or_insert(v);
      *e = std::cmp::max(*e, v);
    }
  }
  
  let mut lcm = 1;
  for (k, v) in map {
    lcm *= mod_pow(k, v);
    lcm %= MOD;
  }
  
  let mut ans = 0;
  for v in vals {
    ans += lcm * mod_inv(v) as usize;
    ans %= MOD;
  }
  println!("{}", ans);
}