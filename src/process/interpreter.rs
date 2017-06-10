use process::process;
use std::error::Error;
use std;
use std::io::prelude::*;
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

	let valid_command = process::exec(parsed_command);	

    if !valid_command {
        use std::process::{Command, Stdio};
        let child = match Command::new("python").stdin(Stdio::piped()).spawn() {
            Err(why) => panic!("couldn't spawn python: {}", why.description()),
            Ok(process) => process,
        };
        child.stdin.unwrap().write_all(cmd.as_bytes());
    }
	/* Check built in commands */
	/* By checking what belongs where */
}	
