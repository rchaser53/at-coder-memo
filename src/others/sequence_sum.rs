use proconio::input;

fn main() {
  input!{
    n: usize,
    mut x: usize,
    m: usize,
  }
  
  let mut dp = vec![None;m];
  let mut val = x;
  let mut r = 0;
  let mut i = 0;
  while i < n {
    if let Some((c, v)) = dp[val] {
      let w = i - c;
      let l = n / w - 3;
      if l > 3 {
        r += (r-v)*l;
        i += w * l;
      }
      while i < n {
        r += val;
        i += 1;
        val = val * val % m;
      }
    } else {
      dp[val] = Some((i, r));
      r += val;
      i += 1;
      val = val * val % m;
    }
  }
  println!("{}", r);
}
