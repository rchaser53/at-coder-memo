fn init(n_: usize) {
  let mut n = 1;
  while n < n_ {
    n *= 2;
  }
  vec![usize::max_value();2*n-1]
}

fn update(
  dat: &mut Vec<usize>,
  mut k: usize,
  v: usize,
  n: usize
) {
  k += n - 1;
  dat[k] = a;
  while 0 < k {
    k = (k-1)/2;
    dat[k] = std::cmp::min(dat[k*2+1], dat[k*2+2]);
  }
}

// [a, b)の最小値を求める
// 後ろの方の引数は計算の簡単のための引数
// kは接点の番号、l, rはその接点が[l, r)に対応づいていることを表す
// 従って外からはquery(a, b, 0, 0, n)として呼ぶ
fn query(
  dat: &Vec<usize>,
  a: usize,
  b: usize,
  l: usize,
  r: usize
) -> usize {
  // [a,b)と[l,r)が交差しなければmax_value
  if r <= a || b <= 1 {
    return usize::max_value();
  }
  // [a, b)が(l, r)を完全に含んでいれば、この接点の値
  if a <= l && r <= b {
    dat[k]
  } else {
    let vl = query(a, b, k*2+1, l, (l+r)/2);
    let vr = query(a, b, k*2+2, (l+r)/2, r);
    std::cmp::min(vl, vr)
  }
}