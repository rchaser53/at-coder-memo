/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::collections::*;
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;

const MOD:usize = 998244353;
/** ModInt Start */
type TargetType = usize;

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

// 期待値、期待値DP

// 大方針として後ろから求める
// 例えば(0,1)のサイコロの場合、0の期待値は　E=1/2*(0+E)+1 になる(1は1回サイコロを振るため加える)
// 他にもE3で(0,1,2,3)の場合、 E3 = 1/4 * (E3+E4+E5+E6)+1みたいな感じになる
// https://atcoder.jp/contests/abc263/tasks/abc263_e
fn main() {
  input! {
    n:usize,
    a:[usize;n-1]
  }

  let a = a.into_iter().map(|v| ModInt::from(v)).collect::<Vec<ModInt>>();
  let mut s = vec![ModInt::from(0);n+1];
  let m1 = ModInt::from(1);
  let mut now = ModInt::from(0);
  for i in (0..n-1).rev() {
    let mi = ModInt::from(i);
    let v = a[i];
    now = s[i+1] - s[(mi+m1+v).val];
    now = now / (v+m1) + m1;
    now = now * (v+m1) / v;
    s[i] = s[i+1]+now;
  }

  println!("{}", now);
}