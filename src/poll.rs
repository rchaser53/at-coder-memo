use proconio::input;
use std::collections::HashMap;

fn main() {
  input! {
    n: usize,
    sn: [String;n]
  }
  let mut map: HashMap<String, usize> = HashMap::new();
  let mut max = 0;
  for v in sn {
    let mut obj = map.entry(v).or_insert(0);
    *obj += 1;
    max = std::cmp::max(max, *obj);
  }
  
  let mut results: Vec<String> = vec![];
  let keys = map.keys();
  for key in keys {
    if *map.get(key).unwrap() == max {
      results.push(key.to_string());
    }
  }
  
  results.sort_by(|a,b| a.cmp(b));
  println!("{}", results.join("\n"));
}
