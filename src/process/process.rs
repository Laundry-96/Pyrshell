use std::process::{Command, Child};
use std::string;
pub fn exec(command: Vec<&str>) {
	println!("{:?}", command);
	let mut child = Command::new(command[0]).spawn();
	let act_child = match child {
		Ok(a) => a,
		Err(b)  => println!("{}",b),
	};
	//child.wait().expect("command wasn't running?");
}
