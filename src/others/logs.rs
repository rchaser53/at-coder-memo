use proconio::input;

fn main() {
  input! {
    n: usize,
    k: usize,
    mut ais: [usize;n],
  }
  ais.sort();
  ais.reverse();

  let mut l = 0;
  let mut r = 1_000_000_000;
  let mut result = 0;
  let mut c = (l + r) / 2;
  while r - l > 1 {
    c = (l + r) / 2;
    let mut sum = 0;
    for v in ais.iter() {
      let v = *v;
      sum += (v - 1) / c;
    }
    
    if sum <= k {
      r = c;
    } else {
      l = c;
    }
  }

  println!("{}", r);
}