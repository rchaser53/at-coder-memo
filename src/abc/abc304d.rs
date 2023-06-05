/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

type Target = usize;
type UseValue = usize;
fn lower_bound(arr: &Vec<Target>, x: &UseValue) -> usize {
    let mut low = 0;
    let mut high = arr.len();
    while low != high {
        let mid = (low + high) / 2;
        // NEEDS TO EDIT
        match arr[mid].cmp(x) {
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

fn main() {
  input! {
    w:usize,
    h:usize,
    n:usize,
    pq:[(usize,usize);n],
    an:usize,
    mut a:[usize;an],
    bn:usize,
    mut b:[usize;bn],
  }

  a.push(w);
  b.push(h);

  let mut map = HashMap::new();

  for (p, q) in pq {
    let xi = lower_bound(&a, &p);
    let yi = lower_bound(&b, &q);

    *map.entry((xi,yi)).or_insert(0) += 1;
  }

  let mut memo = map.into_iter().map(|v| v.1).collect::<Vec<usize>>();
  memo.sort();

  let num = a.len() * b.len();
  if memo.len() < num {
    println!("0 {}", memo[memo.len()-1]);
  } else {
    println!("{} {}", memo[0], memo[memo.len()-1]);
  }
}