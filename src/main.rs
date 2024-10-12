use std::net::IpAddr;

mod tokenizer;
mod test;

fn main() {
	println!("Hello, world!");
	//let a = 4 / i32::from_str_radix("0", 10).unwrap();

	let home: IpAddr = "127.0.0.1".parse().unwrap();
	dbg!(&home);
	match home {
		IpAddr::V4(v4) => println!("v4 {}", v4),
		IpAddr::V6(v6) => println!("v6 {}", v6),
	}
}


