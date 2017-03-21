use std::process::{Command, Child};
use std::string;
use std::result;
pub fn exec(command: Vec<&str>) {
	// Move tbis to interpreter.rs
	if(command.len() == 0) {
		println!();
	}
	let mut child = Command::new(command[0]).args(&command[1..]).spawn();
		
	if(child.is_ok()){
		child.unwrap().wait();
	}
	else{
		println!("{}", child.err().unwrap());
	}
	//child.wait().expect("command wasn't running?");
}
