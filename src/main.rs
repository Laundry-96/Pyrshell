use std::io::{self, Write};
use std::process::Command;

fn main()
{
	loop
    {
        let mut command = String::new();
        match io::stdin().read_line(&mut command) {
            Err(error) => { println!("Error reading line"); continue; },
            _ => (),
        }
        let command = command.trim();

        let mut child = Command::new(command).spawn()
			.expect("Error");

		let ecode = child.wait()
			.expect("Failed to wait on child");

	}
		
}
