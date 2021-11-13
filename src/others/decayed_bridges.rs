use proconio::input;
use proconio::marker::Isize1;

fn size(parents: &mut Vec<isize>, i: isize) -> isize {
  let ii = find(parents, i) as usize;
  -1 * parents[ii]
}

fn find(parents: &mut Vec<isize>, i: isize) -> isize {
  let ii = i as usize;
  if parents[ii] < 0 {
    i
  } else {
    parents[ii] = find(parents, parents[ii]);
    parents[ii]
  }
}

fn connect(
  parents: &mut Vec<isize>,
  a: isize,
  b: isize
) -> bool {
  let mut pa = find(parents, a);
  let mut pb = find(parents, b);
  
  if pa == pb { return false }
  
  if size(parents, pa) < size(parents, pb) {
    let temp = pa;
    pa = pb;
    pb = temp;
  }
  
  let paa = pa as usize;
  let pbb = pb as usize;
  parents[paa] += parents[pbb];
  parents[pbb] = pa;
  
  true
}

fn main() {
  input! {
    n: usize,
    m: usize,
    vals: [(Isize1, Isize1);m]
  }
  let mut parents: Vec<isize> = vec![-1;n];
  let mut result: Vec<isize> = vec![0;m];
  let mut index = m - 1;
  result[index] = (n * (n - 1) / 2) as isize;
  while 0 < index {
    let (from, to) = vals[index];
    result[index-1] = result[index];
    if find(&mut parents, from) != find(&mut parents, to) {
      let val = size(&mut parents, from) * size(&mut parents, to);
      result[index-1] -= val;
      connect(&mut parents, from, to);
    }
    index -= 1;
  }
  
  for v in result {
    println!("{}", v);
  }
}