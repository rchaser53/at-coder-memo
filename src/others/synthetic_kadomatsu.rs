use proconio::input;

fn main() {
  input! {
    n: usize,
    bases: [isize;3],
    bs: [isize;n]
  }
  
  let mut min = isize::max_value();
  let max = 2usize.pow(2*n as u32);
  for i in 0..max {
    let mut bmps: Vec<Vec<isize>> = vec![vec![];3];
    for ii in 0..n {
      let powv = (2 * ii) as u32;
      let mut val = 0;      
      if (i >> powv) & 1 == 1 {
        val += 1;
      }
      
      if (i >> (powv+1u32)) & 1 == 1 {
        val += 2;
      }
      
      if val != 3 {
        bmps[val].push(bs[ii]);
      }
    }
    
    let mut sr = 0;
    let mut succeeded = true;
    for (i, bmp) in bmps.into_iter().enumerate() {
      if bmp.len() == 0 { 
        succeeded = false;
        break
      }
      sr += ((bmp.len() - 1) * 10) as isize;
      sr += (bases[i] - bmp.into_iter().sum::<isize>()).abs();
    }
    
    if succeeded {
      min = std::cmp::min(sr, min);
    }
  }
  println!("{}", min);
}