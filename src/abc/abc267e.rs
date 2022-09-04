/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::collections::*;
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;

impl PartialEq for Wrapper {
  fn eq(&self, other: &Wrapper) -> bool {
    self.0 == other.0
  }
}

impl PartialOrd for Wrapper {
  fn partial_cmp(&self, other: &Wrapper) -> Option<Ordering> {
    let v = self.0.cmp(&other.0);
    Some(if v == Ordering::Equal {
      if self.1.cmp(&other.1) == Ordering::Less {
        Ordering::Greater
      } else {
        Ordering::Less
      }
    } else {
      v
    })
  }
}

#[derive(Clone, Copy, Ord, Eq)] 
struct Wrapper(usize,usize);

fn main() {
  input! {
    n:usize,
    m:usize,
    vals:[usize;n],
    edges:[(Usize1,Usize1);m]
  }

  let mut map = HashMap::new();
  let mut memo = vec![0;n];
  for &(a, b) in &edges {
    memo[a] += vals[b];
    memo[b] += vals[a];

    map.entry(a).or_insert(HashSet::new()).insert(b);
    map.entry(b).or_insert(HashSet::new()).insert(a);
  }

  let mut btreemap = BTreeMap::new();
  for i in 0..n {
    btreemap.insert((Wrapper(memo[i], vals[i]), i), i);
  }

  let mut result = 0;
  for _ in 0..n {    
    let (&key, _) = btreemap.iter().next().unwrap();
    let (Wrapper(sum, val), bi) = key;
    btreemap.remove(&(Wrapper(sum, val), bi));
    result = std::cmp::max(result, sum);

    if let Some(set) = map.remove(&bi) {
      for i in set {
        let current_sum = memo[i];
        if let Some(tset) = map.get_mut(&i) {
          tset.remove(&bi);
          memo[i] -= val;
    
          btreemap.remove(&(Wrapper(current_sum, vals[i]), i));
          btreemap.insert((Wrapper(memo[i], vals[i]), i), i);
        }
      }
    }
  }
  println!("{}", result);
}