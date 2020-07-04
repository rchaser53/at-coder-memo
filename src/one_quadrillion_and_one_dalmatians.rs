use proconio::input;
use std::str::from_utf8;
 
fn main() {
    input! {
        n: i64
    }

  	let mut index = 0;
  	let mut sum = 0;
	while 0 < n - sum {
        index += 1;
		sum += 26i64.pow(index);
  	}
  	let mut left = n + 26i64.pow(index) - sum - 1;
  	index -= 1; 
  	
  	let mut result = vec![];
  	while 0 < index {
        let temp = 26i64.pow(index);
        let char_code = ((left  / temp) + 97) as u8;
      	result.push(char_code);
        left = left % temp;

 		index -= 1;   
    }
  
    let char_code = (left + 97) as u8;
    result.push(char_code);
  	
	println!("{}", from_utf8(&result).unwrap());
}