/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

/** ModInt Start */
type TargetType = usize;
const MOD: TargetType = 998244353;

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

fn main() {
  input! {
    n:usize,
  }

  let mut memo = vec![ModInt::from(0);19];
  for i in 0..19 {
    let ui = i as u32;
    let bottom = ModInt::from(10usize.pow(ui));
    let top = ModInt::from(10usize.pow(ui+1) - 1);
    let num = top - bottom + ModInt::from(1);
    memo[i] = (ModInt::from(1) + num) * num / ModInt::from(2);
  }

  let mut result = ModInt::from(0);
  for i in 0.. {
    let base = 10usize.pow(i as u32);
    if base.to_string().len() == n.to_string().len() {
      let bottom = ModInt::from(base);
      let top = ModInt::from(n);
      let num = top - bottom + ModInt::from(1);
      result += (ModInt::from(1) + num) * num / ModInt::from(2);

      println!("{}", result);
      return
    } else {
      result += memo[i];
    }
  }
}