use std::io::{self, Write};
use std::process::Command;
use process::Prompt;
fn main()
{
	let mut exit_status;
    let prompt = Prompt.new();
    loop
    {
        print!("Shelly$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        
        match io::stdin().read_line(&mut command) {
            Err(error) => { println!("Error reading line"); continue; },
            _ => (),
        }
        let command = command.trim();

        match Command::new(command).spawn() {
            Err(error) => { println!("Error executing, {}", error); continue;},
            Ok(mut child) => { exit_status = child.wait(); },
        }

	}
		
}
