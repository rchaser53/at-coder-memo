use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;

// ---------- lazy segment tree 遅延セグ木 ----------
// 区間更新もできるやつ
// https://yukicoder.me/submissions/379658
pub trait TE {
  type T: Clone+Copy;
  type E: Clone+Copy;
  fn fold(l:Self::T, r:Self::T) -> Self::T;
  fn eval(p:Self::T, x:Self::E) -> Self::T;
  fn merge(f:Self::E, g:Self::E) -> Self::E;
  fn e() -> Self::T;
  fn id() -> Self::E;
}

pub struct LazySegmentTree<R:TE> {
  size: usize,
  bit: usize,
  a: Vec<(R::T, R::E)>,
}

impl <R:TE> LazySegmentTree<R> {
  pub fn new(n:usize) -> LazySegmentTree<R> {
    let mut bit = 0;
    while (1 << bit) < n {
      bit += 1;
    }
    LazySegmentTree {
      size: 1 << bit,
      bit,
      a: vec![(R::e(), R::id()); 2 << bit],
    }
  }

  pub fn build_by(z: &[R::T]) -> LazySegmentTree<R> {
    let n = z.len();
    let mut bit = 0;
    while (1 << bit) < n {
      bit += 1;
    }
    let mut a = vec![(R::e(), R::id()); 2 << bit];
    for (a, &z) in a[(1 << bit)..].iter_mut().zip(z.iter()) {
      a.0 = z;
    }
    for i in (1..(1 << bit)).rev() {
      let l = R::eval(a[2*i].0, a[2*i].1);
      let r = R::eval(a[2*i+1].0, a[2*i+1].1);
      a[i].0 = R::fold(l, r);
    }
    LazySegmentTree {
      size: 1 << bit,
      bit,
      a
    }
  }

  fn eval(&self, k:usize) -> R::T {
    R::eval(self.a[k].0, self.a[k].1)
  }

  fn propagate(&mut self, x:usize) {
    let x = x + self.size;
    for i in (1..(self.bit+1)).rev() {
      let k = x >> i;
      self.a[2*k].1 = R::merge(self.a[2*k].1, self.a[k].1);
      self.a[2*k+1].1 = R::merge(self.a[2*k+1].1, self.a[k].1);
      self.a[k].1 = R::id();
      self.a[k].0 = R::fold(self.eval(2*k), self.eval(2*k+1));
    }
  }

  fn save(&mut self, x:usize) {
    let x = x + self.size;
    for i in 1..self.bit+1 {
      let k = x >> i;
      self.a[k].0 = R::fold(self.eval(2*k), self.eval(2*k+1));
    }
  }

  pub fn update(&mut self, l:usize, r:usize, op: R::E) {
    self.propagate(l);
    self.propagate(r-1);
    let mut x = l + self.size;
    let mut y = r + self.size;
    while x < y {
      if x & 1 == 1 {
        self.a[x].1 = R::merge(self.a[x].1, op);
        x += 1;
      }
      if y & 1 == 1 {
        y -= 1;
        self.a[y].1 = R::merge(self.a[y].1, op);
      }
      x >>= 1;
      y >>= 1;
    }
    self.save(l);
    self.save(r-1);
  }

  pub fn find(&mut self, l:usize, r:usize) -> R::T {
    self.propagate(l);
    self.propagate(r-1);
    let mut x = l + self.size;
    let mut y = r + self.size;
    let mut p = R::e();
    let mut q = R::e();
    while x < y {
      if x & 1 == 1 {
        p = R::fold(p, self.eval(x));
        x += 1;
      }
      if y & 1 == 1 {
        y -= 1;
        q = R::fold(self.eval(y), q);
      }
      x >>= 1;
      y >>= 1;
    }
    R::fold(p, q)
  }
}

struct R;
// ここを状況に応じて書き換える
impl TE for R {
  type T = usize;
  type E = usize;
  fn fold(l:Self::T, r:Self::T) -> Self::T {
    std::cmp::max(l, r)
  }
  fn eval(p:Self::T, x:Self::E) -> Self::T {
    std::cmp::max(p, x)
  }
  fn merge(f:Self::E, g:Self::E) -> Self::E {
    std::cmp::max(f, g)
  }
  fn e() -> Self::T {
    0usize
  }
  fn id() -> Self::E {
    0
  }
}
// ---------- end lazy segment tree ----------

fn main() {
  input!{
    n:usize,
    m:usize,
    mut vals:[(usize,usize);m]
  }

  vals.sort_by(|a,b| (a.0,b.1).cmp(&(b.0,a.1)));
  let mut tree = LazySegmentTree::<R>::new(n+1);
  let mut count = 0;
  for (_,b) in vals {
    let v = tree.find(0, b) + 1;
    count = std::cmp::max(count, v);
    tree.update(b, b+1, v);
  }
  println!("{}", count);
}