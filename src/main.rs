use std::io::{self, Write};
mod process;
mod environment;
fn main()
{
    //let mut exit_status;
    let prompt = environment::prompt::Prompt::new();
    loop
    {
        print!("{}", prompt.get_shell_prompt());
        io::stdout().flush().unwrap();

        let mut command = String::new();
        match io::stdin().read_line(&mut command) {
            Err(error) => { println!("Error reading line: {}", error); continue; },
            _ => process::interpreter::interpret(command),
        }

    }		
}
