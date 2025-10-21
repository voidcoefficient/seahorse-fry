use crate::help::Help;

pub struct Context {
    pub args: Vec<String>,
    help_text: String,
}

impl Context {
    pub fn new<S: Into<String>>(args: Vec<String>, help_text: S) -> Self {
        Self {
            args,
            help_text: help_text.into(),
        }
    }
}

impl Help for Context {
    fn help_text(&self) -> String {
        self.help_text.clone()
    }
}
