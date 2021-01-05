// 平均最大化
// ref: https://github.com/rchaser53/at-corder-memo/blob/master/src/past201912m.rs

fn check(
  vals: &Vec<f64>,
  x: f64,
  k: usize
) -> bool {
  let mut copied = vec![0f64;vals.len()];
  for i in 0..vals.len() {
    let (w, v) = vals[i];
    copied[i] = v - x * w;
  }
  copied.sort();
  copied.reverse();
  let mut sum = 0f64;
  for i in 0..k {
    sum += copied[i];
  }

  0 <= sum
}

// 要確認
fn main() {
  input!{
    n: usize,
    k: usize,
    vals: [(f64,f64);n]
  }

  let mut min = 0f64;
  let mut max = 1_000_000_000f64;
  for i in 0..100 {
    let mid = (min + max) / 2f64;
    if check(&vals, mid, k) {
      min = mid;
    } else {
      max = mid;
    }
  }
  println!("{}", max);
}
