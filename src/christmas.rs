use proconio::{input, fastout};

struct Helper {
  memo: Vec<(usize, usize)>
}

impl Helper {
  fn traverse(&self,
    index: usize,
    mut val: usize,
    mut total: usize
  ) -> usize {
    if val == 1 {
      return total
    }
    val -= 1;
    
    let (b_p, b_b) = self.memo[index-1];
    let last = b_p + b_b;
    if val < last {
      return self.traverse(index-1, val, total);
    } else if val == last {
      return total + b_b;
    }
    val -= last;
    total += b_b;
    
    if val == 1 {
      return total + 1;
    }
    val -=1;
    total +=1;
    
    if val < last {
      return self.traverse(index-1, val, total);
    } else if val == last {
      return total + b_b;
    }
    val -= last;
    total += b_b;
    
    if val == 1 {
      return total
    }
    
    panic!("total: {}, val: {}, index: {}", total, val, index);
  }
}

#[fastout]
fn main() {
  input! {
    n: usize,
    x: usize,
  }
  
  let mut memo: Vec<(usize, usize)> = vec![];
  memo.push((0, 1));
  
  for i in 1..=n {
    let a = memo[i-1].0 * 2usize + 2usize;
    let b = memo[i-1].1 * 2usize + 1usize;
    memo.push((a, b));
  }  
  let helper = Helper { memo };
  let result = helper.traverse(n, x, 0);  

  println!("{}", result);
}