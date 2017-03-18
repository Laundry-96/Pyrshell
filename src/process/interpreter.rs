use process::process;
pub fn interpret(cmd: String)
{
	let parsed_command: Vec<_> = cmd.split_whitespace().collect();
	process::exec(parsed_command);	
	/* Check built in commands */
	/* By checking what belongs where */
}	
