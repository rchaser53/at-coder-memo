use proconio::input;

fn main() {
    input! {
        x: i32,
      	n: usize,
      	mut arr: [usize;n]
    };

  	let mut index = 0;
  	let mut min = x;
  	let mut temp_vec: Vec<i32> = Vec::with_capacity(101);
  
  	for i in 0..=101 {
  		temp_vec.push((i - x).abs());
  	}
  
	for i in 0..n {
      	temp_vec[arr[i]] = 201;
  	}
  	
    for i in 0..=101 {
    	if temp_vec[i] < min {
      		index = i;
          	min = temp_vec[i];
      	}
  	}
 	
    println!("{}", index);
}