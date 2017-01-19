use std::io::{self, Write};
use std::process::Command;
mod environment;
fn main()
{
	
    let mut exit_status;
    let prompt = environment::prompt::Prompt::new();
    loop
    {
        print!("{}", prompt.get_shell_prompt());
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
