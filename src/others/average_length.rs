use proconio::input;

pub trait LexicalPermutation {
    /// Return `true` if the slice was permuted, `false` if it is already
    /// at the last ordered permutation.
    fn next_permutation(&mut self) -> bool;
    /// Return `true` if the slice was permuted, `false` if it is already
    /// at the first ordered permutation.
    fn prev_permutation(&mut self) -> bool;
}

impl<T> LexicalPermutation for [T] where T: Ord {
    /// Original author in Rust: Thomas Backman <serenity@exscape.org>
    fn next_permutation(&mut self) -> bool {
        // These cases only have 1 permutation each, so we can't do anything.
        if self.len() < 2 { return false; }

        // Step 1: Identify the longest, rightmost weakly decreasing part of the vector
        let mut i = self.len() - 1;
        while i > 0 && self[i-1] >= self[i] {
            i -= 1;
        }

        // If that is the entire vector, this is the last-ordered permutation.
        if i == 0 {
            return false;
        }

        // Step 2: Find the rightmost element larger than the pivot (i-1)
        let mut j = self.len() - 1;
        while j >= i && self[j] <= self[i-1]  {
            j -= 1;
        }

        // Step 3: Swap that element with the pivot
        self.swap(j, i-1);

        // Step 4: Reverse the (previously) weakly decreasing part
        self[i..].reverse();

        true
    }

    fn prev_permutation(&mut self) -> bool {
        // These cases only have 1 permutation each, so we can't do anything.
        if self.len() < 2 { return false; }

        // Step 1: Identify the longest, rightmost weakly increasing part of the vector
        let mut i = self.len() - 1;
        while i > 0 && self[i-1] <= self[i] {
            i -= 1;
        }

        // If that is the entire vector, this is the first-ordered permutation.
        if i == 0 {
            return false;
        }

        // Step 2: Reverse the weakly increasing part
        self[i..].reverse();

        // Step 3: Find the rightmost element equal to or bigger than the pivot (i-1)
        let mut j = self.len() - 1;
        while j >= i && self[j-1] < self[i-1]  {
            j -= 1;
        }

        // Step 4: Swap that element with the pivot
        self.swap(i-1, j);

        true
    }
}


fn main() {
  input! {
    n: usize,
    points: [(f64,f64);n],
  }
  
  let mut base: Vec<usize> = vec![];
  for v in 0..n {
    base.push(v);
  }
  
  let mut temps: Vec<Vec<(f64,f64)>> = vec![];
  loop {
    let temp_points: Vec<(f64,f64)> = base.iter().map(|i| points[*i]).collect();
    temps.push(temp_points);
    if !base.next_permutation() {
      break;
    }
  }
  
  let mut sum = 0f64;
  for temp in temps {
    let mut last_a = temp[0].0;
    let mut last_b = temp[0].1;
    let mut s_sum = 0f64;
    for i in 1..n {
      let (a,b) = temp[i];
      s_sum += ((last_a - a).powf(2f64) + (last_b - b).powf(2f64)).sqrt();
      last_a = a;
      last_b = b;
    }
    sum += s_sum;
  }
  
  let mut total = 1;
  for v in 1..=n {
    total *= v;
  }

  println!("{}", sum / total as f64);
}