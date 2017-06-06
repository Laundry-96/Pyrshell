use std::process::Command;

/// Executes the command given to it, along with arguments
pub fn exec(command: Vec<&str>) {


    //Create a new command process (essentially fork)
	let child = Command::new(command[0]).args(&command[1..]).spawn();
		
	if child.is_ok() {
		child.unwrap().wait();
	}
	else {
		println!("{}", child.err().unwrap());
	}
	//child.wait().expect("command wasn't running?");
}
