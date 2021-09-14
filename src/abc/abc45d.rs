use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn helper(
  result: &mut Vec<usize>,
  set: &HashSet<(usize, usize)>,
  si: usize,
  sj: usize,
) {
  let mut count = 0;
  for i in 0..3 {
    for j in 0..3 {
      if set.contains(&(si+i, sj+j)) {
        count += 1;
      }
    }
  }
  result[count] += 1;
}

fn main() {
  input!{
    h:usize,
    w:usize,
    n:usize,
    edges:[(Usize1,Usize1);n]
  }

  let mut set = HashSet::new();
  for &(a, b) in &edges {
    set.insert((a,b));
  }

  let mut seen = HashSet::new();
  let mut result = vec![0;10];
  for &(a, b) in &edges {
    for i in 0..3 {
      for j in 0..3 {
        if a < i || b < j {
          continue
        }
        let bi = a-i;
        let bj = b-j;

        if h <= bi+2 || w <= bj+2 { continue }
        if seen.contains(&(bi, bj)) { continue }
        seen.insert((bi, bj));
        helper(&mut result, &set, bi, bj);
      }
    }

    for i in 0..3 {
      for j in 0..3 {
        if h <= a+1 || w < b+j {
          continue
        }
        let bi = a+i;
        let bj = b+j;

        if h <= bi+2 || w <= bj+2 { continue }
        if seen.contains(&(bi, bj)) { continue }
        seen.insert((bi, bj));
        helper(&mut result, &set, bi, bj);
      }
    }
  }

  let mut zero = (h-2) * (w-2);
  for i in 1..10 {
    zero -= result[i];
  }
  println!("{}", zero);

  for i in 1..10 {
    println!("{}", result[i]);
  }
}