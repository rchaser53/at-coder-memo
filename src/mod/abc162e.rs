/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

/** ModInt Start */
type TargetType = usize;
const MOD: TargetType = 1_000_000_007;

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

fn main() {
  input! {
    n:usize,
    k:usize
  }
  
  let mut result = ModInt::from(0);
  let mut memo = vec![ModInt::from(0);k+1];
  for i in (1..=k).rev() {
    let num = k / i;

    let mut temp = ModInt::from(0);
    for j in (2..=num).rev() {
      temp += memo[j*i];
    }
    let pattern = repeat_square(ModInt::from(num), n) - temp;
    memo[i] = pattern;
    result += ModInt::from(i) * pattern;
  }
  println!("{}", result);
}