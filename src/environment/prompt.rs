use std::env;

pub struct Prompt {
    shell_prompt: String,
}

impl Prompt {
    pub fn new() -> Prompt
    {
        let mut new_prompt = Prompt
        {
            shell_prompt: "SHELLY$ ".to_owned(),
        };

        new_prompt
    }

    pub fn get_shell_prompt(&self) -> &str
    {
        &self.shell_prompt
    }

    pub fn set_shell_prompt(&mut self, new_prompt: String) 
    {
        self.shell_prompt = new_prompt;
    }
}
