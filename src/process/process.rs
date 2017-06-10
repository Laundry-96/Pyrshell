use std::process::Command;

/// Executes the command given to it, along with arguments
pub fn exec(command: Vec<&str>) -> bool {


    //Create a new command process (essentially fork)
	let child = Command::new(command[0]).args(&command[1..]).spawn();
		
	if child.is_ok() {
		child.unwrap().wait();
        return true;
	}
	else {
        return false;
		println!("{}", child.err().unwrap());
	}
}
