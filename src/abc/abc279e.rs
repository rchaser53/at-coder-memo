/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Reverse;
use std::cmp::Ordering;
use std::collections::*;

type Target = (usize,usize);
type UseValue = usize;
fn lower_bound(arr: &Vec<Target>, x: &UseValue) -> usize {
    let mut low = 0;
    let mut high = arr.len();
    while low != high {
        let mid = (low + high) / 2;
        // NEEDS TO EDIT
        match arr[mid].0.cmp(x) {
            std::cmp::Ordering::Less => {
                low = mid + 1;
            }
            std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {
                high = mid;
            }
        }
    }
    low
}

fn dfs(
  memo: &mut HashMap<(usize,usize),usize>,
  map: &HashMap<usize,Vec<(usize,usize)>>,
  ci: usize,
  ai: usize,
) -> usize {
  if let Some(val) = memo.get(&(ci, ai)) {
    return *val
  }

  if let Some(arr) = map.get(&ai) {
    let n = arr.len();
    let ti = lower_bound(&arr, &ci);

    if ti == n {
      memo.insert((ci,ai), ai);
      ai
    } else {
      let (ni, nai) = arr[ti];
      let result = dfs(memo, map, ni+1, nai);
      memo.insert((ci,ai), result);
      result
    }
  } else {
    memo.insert((ci,ai), ai);
    ai
  }
}

fn main() {
  input! {
    n:usize,
    m:usize,
    a:[Usize1;m]
  }
  let mut map = HashMap::new();
  for i in 0..m {
    map.entry(a[i]).or_insert(vec![]).push((i,a[i]+1));
    map.entry(a[i]+1).or_insert(vec![]).push((i,a[i]));
  }

  let mut result = vec![0;m];
  let mut ai = 0;
  let mut memo = HashMap::new();
  for i in 0..m {
    result[i] = dfs(&mut memo, &mut map, i+1, ai);
    let av = a[i];
    if av == ai {
      ai = av + 1;
    } else if av + 1 == ai {
      ai = av;
    }
  }

  for v in result {
    println!("{}", v+1);
  }
}