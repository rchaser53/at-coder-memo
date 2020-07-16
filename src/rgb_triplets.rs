use proconio::input;

fn main() {
  input! {
    _N: usize,
    S: String
  }
  
  let mut rs: Vec<usize> = vec![];
  let mut gs: Vec<usize> = vec![];
  let mut bs: Vec<usize> = vec![];
  
  for (i, c) in S.chars().enumerate() {
    match c {
      'R' => rs.push(i+1),
      'G' => gs.push(i+1),
      'B' => bs.push(i+1),
      _ => unreachable!()
    }
  }
  rs.reverse();
  gs.reverse();
  bs.reverse();
  
  let mut result = 0;
  result += culc(&mut rs, &mut gs, &mut bs);
  result += culc(&mut rs, &mut bs, &mut gs);
  result += culc(&mut bs, &mut rs, &mut gs);
  result += culc(&mut bs, &mut gs, &mut rs);
  result += culc(&mut gs, &mut rs, &mut bs);
  result += culc(&mut gs, &mut bs, &mut rs);
  
  println!("{}", result);
}

fn culc(r: &mut Vec<usize>, g: &mut Vec<usize>, b: &mut Vec<usize>) -> usize {
  let mut result = 0;
  for i in r.iter() {
    for j in g.iter() {
      if j < i { break }
      for k in b.iter() {
        if k < j { break }
        if i + k != j * 2 {
          result += 1;
        }
      }
    }
  }
  result
}
