/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;


type TargetType = usize;
const MOD:usize = 998244353;
// ===== MOD utils start =====
fn mod_pow(a: TargetType, n: TargetType) -> TargetType { 
  if n == 0 {
    1
  } else if n == 1 {
    a % MOD
  } else if n % 2 == 1 {
    let k = mod_pow(a, (n - 1) / 2);
    let ans = (a * k) % MOD;
    (ans * k) % MOD
  } else {
    let k = mod_pow(a, n / 2);
    (k * k) % MOD
  }
}

fn mod_inv(a:TargetType) -> TargetType {
  mod_pow(a, MOD-2)
}
// ===== MOD utils end =====

#[derive(Clone, Copy)]
struct ModInt {
  val: TargetType
}

impl From<TargetType> for ModInt {
  fn from(item: TargetType) -> Self {
      ModInt { val: item as TargetType % MOD }
  }
}
impl std::fmt::Debug for ModInt {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.val)
  }
}
impl std::fmt::Display for ModInt {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.val)
  }
}

impl std::ops::Add for ModInt {
  type Output = Self;
  fn add(self, other: Self) -> Self {
    Self {
      val: (self.val + other.val) % MOD,
    }
  }
}
impl std::ops::AddAssign for ModInt {
  fn add_assign(&mut self, other: Self) {
    *self = Self {
      val: (self.val + other.val) % MOD
    };
  }
}

impl std::ops::Sub for ModInt {
  type Output = Self;
  fn sub(self, other: Self) -> Self {
    Self {
      val: (MOD + self.val - other.val % MOD) % MOD,
    }
  }
}
impl std::ops::SubAssign for ModInt {
  fn sub_assign(&mut self, other: Self) {
    *self = Self {
      val: (MOD + self.val - other.val) % MOD
    };
  }
}

impl std::ops::Mul for ModInt {
  type Output = Self;
  fn mul(self, other: Self) -> Self {
    Self {
      val: self.val * other.val % MOD,
    }
  }
}
impl std::ops::MulAssign for ModInt {
  fn mul_assign(&mut self, other: Self) {
    *self = Self {
      val: self.val * other.val % MOD
    };
  }
}

impl std::ops::Div for ModInt {
  type Output = Self;
  fn div(self, other: Self) -> Self {
    Self {
      val: self.val * mod_inv(other.val) % MOD,
    }
  }
}
impl std::ops::DivAssign for ModInt {
  fn div_assign(&mut self, other: Self) {
    *self = Self {
      val: self.val * mod_inv(other.val) % MOD
    };
  }
}
/** ModInt End */

// ModInt 用の repeat_square
fn repeat_square(n:ModInt, p:usize) -> ModInt {
  if p == 0 {
    ModInt::from(1)
  } else if p % 2 == 0 {
    let v = repeat_square(n, p/2);
    v * v
  } else {
    n * repeat_square(n, p-1)
  }
}

// 10^6くらいまでのnCkが求められるやつ
const MAX: usize = 250000;
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

fn main() {
  input! {
    n:usize,
    m:usize,
    k:usize
  }
  
  let mut result = ModInt::from(0);
  let m = ModInt::from(m);

  let mut fact = vec![0;MAX];
  let mut finv = vec![0;MAX];
  let mut inv = vec![0;MAX];
  init(&mut fact, &mut finv, &mut inv);

  for i in 0..=k {
    let mut v = m * repeat_square(m-ModInt::from(1), n-i-1);
    v *= ModInt::from(com(&fact, &finv, n-1, i));
    result += v;
  }
  println!("{}", result);
}