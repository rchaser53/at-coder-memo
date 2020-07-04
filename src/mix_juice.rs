use proconio::input;
 
fn main() {
    input! {
        n: usize,
      	m: usize,
      	mut fruits: [u64;n],
    }

	fruits.sort();
  	let mut sum = 0;
  	for i in 0..m {
  		sum += fruits[i];
  	}
  	
	println!("{}", sum);
}