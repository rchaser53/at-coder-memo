/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn create_primes(n:usize) -> Vec<usize> {
  let mut memo = vec![true;n+1];
  memo[0] = false;
  memo[1] = false;
  for i in 2..=n {
    if !memo[i] { continue }
    for j in 2.. {
      let ni = i * j;
      if n < ni { break }
      memo[ni] = false;
    }
  }

  let mut primes = vec![];
  for i in 0..memo.len() {
    let v = memo[i];
    if v {
      primes.push(i);
    }
  }
  primes
}

fn main() {
  input! {
    n:usize
  }

  let mut primes = create_primes(n);
  primes.push(1);
  // println!("primes: {:?}", primes);
  primes.sort();
  let m = primes.len();
  let mut set = HashSet::new();
  for i in 0..m {
    for j in i+1..m {
      let v = primes[i].pow(2) + primes[j].pow(2);
      if n < v {
        break;
      }
      set.insert(v);
    }
  }

  let mut result = set.into_iter().collect::<Vec<usize>>();
  result.sort();

  println!("{}", result.len());
  println!("{}", result.into_iter().map(|v| format!("{}", v)).collect::<Vec<String>>().join(" "));  
}