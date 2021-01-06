fn main() {
  input! {
    n: usize,
    w: usize,
    vals: [(usize,usize);n] 
  }

  let n2 = n / 2;
  let mut ps = vec![(0,0);1<<n2];
  for i in 0..(1<<n2) {
    let mut sw = 0;
    let mut sw = 0;
    for ii in 0..n2 {
      if i >> ii & 1 == 1 {
        sw += vals[ii].0;
        sv += vals[ii].1;
      }
    }
    ps[i] = (sw, sv);
  }

  // 無駄な要素を取り除く
  ps.sort_by(|a, b| {
    a.0.cmp(&b.0)
  });
  let mut m = 1;
  for i in 1..(1<<n2) {
    if ps[m-1].1 < ps[i].1 {
      ps[m] = ps[i];
      m += 1;
    }
  }

  let mut result = 0;
  for i in 0..(1<<(n-n2)) {
    let mut sw = 0;
    let mut sv = 0;
    for ii in 0..(n-n2) {
      sw += vals[n2+ii].0;
      sv += vals[n2+ii].1;
    }
    if sw <= w {
      // tuppleのbinary searchで適切なvを取得する
      let tv= ps.binary_search(0, w-sw).1;
      result = std::cmp::max(result, sv + tv);
    }
  }
  println!("{}", result);
}