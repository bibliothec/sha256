extern crate crypto;

use crypto::sha2::Sha256;
use crypto::digest::Digest;

fn main() {

	println!("print sha256:  input letters");

	let mut input=String::new();
	std::io::stdin().read_line(&mut input).ok();
	let mut sha256=Sha256::new();
	sha256.input_str(&input);

	println!("{}",sha256.result_str());    
}
