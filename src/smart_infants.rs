use proconio::input;
use proconio::marker::Usize1;
use std::collections::BTreeSet;

fn main() {
  input! {
    n: usize,
    q: usize,
    mut initial: [(usize, Usize1);n],
    moves: [(Usize1, Usize1);q]
  };
  let mut memo = vec![BTreeSet::new(); 200_000];
  for (i, &(a, b)) in initial.iter().enumerate() {
    memo[b].insert((a, i));
  }
  
  let mut result = BTreeSet::new();
  for v in memo.iter().filter(|m| !m.is_empty()) {
    result.insert(*v.iter().rev().next().unwrap()); 
  }
  
  for &(child, next) in moves.iter() {
    let (rate, current) = initial[child];
    
    initial[child].1 = next;
    let &s = memo[current].iter().rev().next().unwrap();
    result.remove(&s);
    if !memo[next].is_empty() {
      let &t = memo[next].iter().rev().next().unwrap();
      result.remove(&t);
    }
    memo[current].remove(&(rate, child));
    memo[next].insert((rate, child));
    
    if !memo[current].is_empty() {
      let &s = memo[current].iter().rev().next().unwrap();
      result.insert(s);
    }
    let &t = memo[next].iter().rev().next().unwrap();
    result.insert(t);
    println!("{}", result.iter().next().unwrap().0);
  } 
}
