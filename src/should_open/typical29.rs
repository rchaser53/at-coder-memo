/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

// ---------- start lazy segment tree ----------
pub struct LazySegmentTree<T, E, F, G, H> {
  pub size: usize,
  height: usize,
  n: usize,
  data: Vec<T>,
  lazy: Vec<E>,
  ti: T,
  ei: E,
  f: F,
  g: G,
  h: H,
}
impl<T, E, F, G, H> LazySegmentTree<T, E, F, G, H>
where
  T: Copy,
  E: Copy + std::cmp::PartialEq,
  F: Fn(T, T) -> T,
  G: Fn(T, E) -> T,
  H: Fn(E, E) -> E,
{
  pub fn new(n: usize, f: F, g: G, h: H, ti: T, ei: E) -> Self {
      let mut size = 1;
      let mut height = 0;
      while size < n {
          size *= 2;
          height += 1;
      }
      LazySegmentTree {
          size: size,
          height: height,
          n: n,
          data: vec![ti; size * 2],
          lazy: vec![ei; size * 2],
          ti: ti,
          ei: ei,
          f: f,
          g: g,
          h: h,
      }
  }
  pub fn build(&mut self, v: Vec<T>) {
      assert_eq!(self.n, v.len());
      for i in 0..v.len() {
          self.data[self.size + i] = v[i];
      }
      for i in (1..self.size).rev() {
          self.update(i)
      }
  }
  fn f(&self, a: T, b: T) -> T {
      (self.f)(a, b)
  }
  fn g(&self, a: T, b: E) -> T {
      (self.g)(a, b)
  }
  fn h(&self, a: E, b: E) -> E {
      (self.h)(a, b)
  }
  fn update(&mut self, idx: usize) {
      self.data[idx] = self.f(self.data[idx * 2], self.data[idx * 2 + 1]);
  }
  fn update_parents(&mut self, idx: usize) {
      for i in 1..self.height + 1 {
          self.update(idx >> i);
      }
  }
  fn all_apply(&mut self, idx: usize, x: E) {
      self.data[idx] = self.g(self.data[idx], x);
      if idx < self.size {
          self.lazy[idx] = self.h(self.lazy[idx], x);
      }
  }
  fn propagate(&mut self, idx: usize) {
      if self.lazy[idx] != self.ei {
          self.all_apply(idx * 2, self.lazy[idx]);
          self.all_apply(idx * 2 + 1, self.lazy[idx]);
          self.lazy[idx] = self.ei;
      }
  }
  fn eval(&mut self, k: usize) {
      let idx = k + self.size;
      for i in (1..self.height + 1).rev() {
          self.propagate(idx >> i);
      }
  }
  fn eval_range(&mut self, mut l: usize, mut r: usize) {
      l += self.size;
      r += self.size;
      for i in (1..self.height + 1).rev() {
          if (l >> i) << i != l {
              self.propagate(l >> i);
          }
          if (r >> i) << i != r {
              self.propagate((r - 1) >> i);
          }
      }
  }
  pub fn set(&mut self, k: usize, x: T) {
      self.eval(k);
      let idx = k + self.size;
      self.data[idx] = x;
      self.update_parents(idx);
  }
  pub fn get(&mut self, k: usize) -> T {
      self.eval(k);
      self.data[k + self.size]
  }
  pub fn prood(&mut self, mut l: usize, mut r: usize) -> T {
      if l >= r {
          return self.ti;
      }
      self.eval_range(l, r);
      l += self.size;
      r += self.size;
      let mut l_val = self.ti;
      let mut r_val = self.ti;
      while l < r {
          if l % 2 == 1 {
              l_val = self.f(l_val, self.data[l]);
              l += 1;
          }
          if r % 2 == 1 {
              r_val = self.f(self.data[r - 1], r_val);
          }
          l /= 2;
          r /= 2;
      }
      self.f(l_val, r_val)
  }
  pub fn apply(&mut self, k: usize, x: E) {
      self.eval(k);
      let idx = k + self.size;
      self.data[idx] = self.g(self.data[idx], x);
      self.update_parents(idx);
  }
  pub fn apply_range(&mut self, l: usize, r: usize, x: E) {
      if l >= r {
          return;
      }
      self.eval_range(l, r);
      let mut l_idx = l + self.size;
      let mut r_idx = r + self.size;
      while l_idx < r_idx {
          if l_idx % 2 == 1 {
              self.all_apply(l_idx, x);
              l_idx += 1;
          }
          if r_idx % 2 == 1 {
              self.all_apply(r_idx - 1, x);
          }
          l_idx /= 2;
          r_idx /= 2;
      }
      l_idx = l + self.size;
      r_idx = r + self.size;
      for i in 1..self.height + 1 {
          if (l_idx >> i) << i != l_idx {
              self.update(l_idx >> i);
          }
          if (r_idx >> i) << i != r_idx {
              self.update((r_idx - 1) >> i);
          }
      }
  }
}
// ---------- end lazy segment tree ----------

fn main() {
  input!{
    w:usize,
    n:usize,
    vals:[(Usize1,Usize1);n]
  }
  let f = |x:usize, y:usize| if x > y { x } else { y };
  let g = |x:usize, y:usize| y;
  let h = |x:usize, y:usize| if x > y { x } else { y };
  let mut tree = LazySegmentTree::new(5*10usize.pow(5)+10, f, g, h, 0, 0);

  for (l, r) in vals {
    let bv = tree.prood(l, r+1);
    tree.apply_range(l, r+1, bv+1);
    println!("{}", bv+1);
  }
}