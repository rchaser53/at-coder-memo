use proconio::input;
 
fn main() {
    input! {
        n: i64,
        m: i64,
        k: i64,
        a: [i64; n],
        b: [i64; m],
    }
  	let a_table = create_table(&a);
    let b_table = create_table(&b);
    
    let mut a_index: i64 = 0;
    let mut b_index: i64 = 0;
    for i in 0..=m {
        if b_table[i as usize] <= k {
            b_index = i;
        } else {
            break;
        }
    }
  	let mut max = b_index;
 
    while 0 <= b_index {
       let mut last_a_index = a_index;
	   let b_val = b_table[b_index as usize];
       while a_index <= n {
          let a_val = a_table[a_index as usize];
          if a_val + b_val <= k {
            max = std::cmp::max(max, a_index + b_index);
            last_a_index = a_index;
            a_index += 1;
          } else {
            break
          }
       }
       
       if n < a_index {
          break;
       }
      
       a_index = last_a_index;
       b_index -= 1;
    }
    println!("{}", max);
}
 
fn create_table(input: &[i64]) -> Vec<i64> {
	let mut result = Vec::with_capacity(input.len() + 1);
    result.push(0);
    let mut sum = 0;
    for val in input {
      sum += val;
      result.push(sum);
    }
  	result
}
