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

fn kruskal(
  edges: &Vec<(isize,isize,isize)>,
  n:usize
) -> (isize, Vec<isize>) {
  let mut sum = 0isize;
  let mut memo = vec![-1;n];

  for &(a,b,v) in edges {
    let av = find(&mut memo, a);
    let bv = find(&mut memo, b);
    if av != bv {
      connect(&mut memo, a, b);
      sum += v;
    }
  }
  (sum, memo)
}


// n個のnode
// connections [node1, node2, cost]
// 全域木を構成するコストを求めよ。全域木を構成できなかった場合は-1を返すこと
pub fn kruskal_problem(n: i32, connections: Vec<Vec<i32>>) -> i32 {
  if n == 1 { return 0 }

  let n = n as usize;
  let mut edges = vec![];
  for arr in connections {
    edges.push((arr[0] as isize - 1, arr[1] as isize - 1, arr[2] as isize));
    edges.push((arr[1] as isize - 1, arr[0] as isize - 1, arr[2] as isize));
  }

  edges.sort_by(|a,b| a.2.cmp(&b.2));
  let (cost, memo) = kruskal(&edges, n);
  for pi in memo {
    if pi < 0 {
      if pi * -1 == n as isize {
        break
      } else {
        return -1
      }
    }
  }
  cost as i32
}
