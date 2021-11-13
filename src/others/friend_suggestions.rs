use proconio::input;
use std::collections::HashMap;
 
fn main() {
  input! {
    N: usize,
    M: usize,
    K: usize,
    A: [(usize, usize);M],
    B: [(usize, usize);K],
  }
  
  let mut friends: Vec<Vec<usize>> = vec![vec![];N+1];
  for a in A.iter() {
    friends[a.0].push(a.1);
    friends[a.1].push(a.0);
  }
  
  let mut group_num = 1;
  let mut map: HashMap<usize, usize> = HashMap::new();
  let mut connect_map: HashMap<usize, usize> = HashMap::new();
  for i in 1..=N {
    if map.get(&i).is_none() {
      let mut stack: Vec<usize> = vec![i];
      let mut count = 0;
      while 0 < stack.len() {
        let v = stack.pop().unwrap();
        for a in friends[v].iter() {
          if map.get(&a).is_none() {
            map.insert(*a, group_num);
            count += 1;
            stack.push(*a);
          }
        }
      }
      connect_map.insert(group_num, count);
      group_num += 1;
    }
  }
  
  let mut blocks: Vec<usize> = vec![0;N+1];
  for b in B.iter() {
    if map.get(&b.0) == map.get(&b.1) {
      blocks[b.0] += 1;
      blocks[b.1] += 1;
    }
  }

  let mut result: Vec<usize> = vec![0;N];
  for i in 1..=N {
    if let Some(v) = map.get(&i) {
      let cv = connect_map.get(&v).unwrap();
      result[i-1] = cv - friends[i].len() - blocks[i] - 1;
    } else {
      result[i-1] = 0;
    }
  }
  
  println!("{}", result
    .into_iter()
    .map(|v| v.to_string())
    .collect::<Vec<String>>()
    .join(" ")
  );
}