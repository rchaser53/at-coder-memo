use proconio::input;
use std::collections::HashMap;

fn helper(
  map: &mut HashMap<usize, usize>,
  val: usize,
  count: usize
) -> usize {
  if val == 1 {
    return 1
  }
  
  if let Some(memo_val) = map.get(&val) {
    *memo_val
  } else {
    let result = count + 2 * helper(map, val / 2, 1);
    map.insert(val, result);
    result
  }
}

fn main() {
  input! {
    h: usize,
  }

  let mut map: HashMap<usize, usize> = HashMap::new();
  println!("{}", helper(&mut map, h, 1));
}
