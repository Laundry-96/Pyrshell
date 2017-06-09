use process::process;
use std;
pub fn interpret(cmd: String)
{
	let trimmed_cmd = cmd.trim();

	// If the line is blank, just reprint shell
	if trimmed_cmd.len() == 0 {
		return ();
	}
	if trimmed_cmd == "exit"{
		use std::process::exit;
		std::process::exit(1);
	}
	let parsed_command: Vec<_> = cmd.split_whitespace().collect();

	// If there is nothing entered, just reprint shell
	if parsed_command.len() == 0 {
		println!();
	}

	process::exec(parsed_command);	
	/* Check built in commands */
	/* By checking what belongs where */
}	
