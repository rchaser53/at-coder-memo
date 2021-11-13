use proconio::input;

fn main() {
  input! {
    a: usize,
    b: usize,
    q: usize,
    mut aps: [isize;a],
    mut bps: [isize;b],
    mut qs: [isize;q]
  }
  aps.sort();
  bps.sort();
  
  for qv in qs {
    let mut ais = vec![];
    let mut bis = vec![];
    let av = aps.binary_search(&qv).unwrap_err();
    let bv = bps.binary_search(&qv).unwrap_err();
    
    if av < aps.len() {
      if 0 < av {
        ais.push(av-1);
      }
      if av < aps.len() - 1 {
        ais.push(av+1);
      }
      ais.push(av);
    } else {
      ais.push(av-1);
    }
    
    if bv < bps.len() {
      if 0 < bv {
        bis.push(bv-1);
      }
      if bv < bps.len() - 1 {
        bis.push(bv+1);
      }
      bis.push(bv);
    } else {
      bis.push(bv-1);
    }

    let mut min = isize::max_value();
    for ai in ais {
      let av = aps[ai];
      for bi in bis.iter() {
        let bv = bps[*bi];

        let v = if (av - qv).abs() < (bv - qv).abs() {
          (av - qv).abs() + (bv - av).abs()
        } else {
          (bv - qv).abs() + (bv - av).abs()
        };
        min = std::cmp::min(min, v);
      }
    }
    println!("{}", min);
  }
}