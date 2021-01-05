// 反転(しゃくとり法?) 蟻本P139
fn calc(
  s: &Vec<isize>,
  n: isize,
  k: isize
) -> isize {
  let mut f = vec![0;5000];
  let res = 0;
  let sum = 0;    // fの和
  for i in 0..=n-k {
    let ui = i as usize;
    if (s[ui] + sum) % 2 == 1 {
      // 先頭の牛が後ろを向いてる
      res += 1;
      f[ui] = 1;
    }
    sum += f[ui];
    if k <= i + 1 {
      sum -= f[(i-k+1) as usize];
    }
  }

  for i in n-k+1..n {
    let ui = i as usize;
    if (s[ui] + num) % 2 == 1 {
      // 解なし
      return -1
    }
    if k <= i + 1 {
      sum -= f[(i-k+1) as usize];
    }
  }
  res
}

fn main() {
  input!{
    n: isize,
    s: Chars,
  }
  let s = s
    .into_iter()
    .map(|v| if s == 'F' { 0 } else { 1 })
    .collect::<Vec<isize>>();

  let mut k = 1;
  let mut m = n;
  let mut i = 1;
  while i <= n {
    let v = calc(&s, n, k);
    if 0 <= v && v < m {
      m = v;
      k = i;
    }
    i += 1;
  }
  println!("{} {}", k, m);
}