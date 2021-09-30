use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn check(vals: &Vec<usize>) -> bool {
  vals[0] % 2 == 0 && vals[1] % 2 == 0 && vals[2] % 2 == 0
}

fn main() {
  input!{
    mut vals:[usize;3]
  }
  if vals[0] % 2 == 0 && vals[0] == vals[1] && vals[1] == vals[2] {
    println!("-1");
    return
  }

  vals.sort();
  let mut set = HashSet::new();
  loop {
    if set.contains(&(vals[0], vals[1], vals[2])) {
      println!("-1");
      return
    }
    if !check(&vals) {
      break
    }
    set.insert((vals[0], vals[1], vals[2]));
    let vh1 = vals[0] / 2;
    let vh2 = vals[1] / 2;
    let vh3 = vals[2] / 2;
    vals = vec![vh1+vh2, vh2+vh3, vh1+vh3];
    vals.sort();
  }

  println!("{}", set.len());
}