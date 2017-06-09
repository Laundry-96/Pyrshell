/// Creates a struct for a Prompt
///
/// A prompt consists of a string that is printed out to a user
/// to infer input is requested
pub struct Prompt {
    /// String that is printed out, assigned to "PYRSH$ " upon new instance
    shell_prompt: String,
}

impl Prompt {

    /// Returns a prompt with a default prompt
    ///
    /// # Example
    ///
    /// ```
    /// use prompt::Prompt;
    /// let p = Prompt::new();
    /// ```
    pub fn new() -> Prompt
    {
        let new_prompt = Prompt
        {
            shell_prompt: "PYRSH$ ".to_owned(),
        };

        new_prompt
    }
    
    /// Returns the prompt string
    pub fn get_shell_prompt(&self) -> &str
    {
        &self.shell_prompt
    }

    /// Sets the prompt to something that the user would like
    pub fn set_shell_prompt(&mut self, new_prompt: String) 
    {
        self.shell_prompt = new_prompt;
    }
}
