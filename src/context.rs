use crate::{help::Help, utils::normalize_args};

#[derive(Debug)]
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

    pub fn flags(&self) -> Vec<String> {
        normalize_args(self.args.clone())
            .iter()
            .filter(|arg| arg.starts_with("-"))
            .map(|arg| arg.to_owned())
            .collect()
    }

    pub fn values(&self) -> Vec<String> {
        normalize_args(self.args.clone())
            .iter()
            .filter(|arg| !arg.starts_with("-"))
            .map(|arg| arg.to_owned())
            .collect()
    }

    pub fn value(&self) -> Option<String> {
        self.values().first().map(|value| value.to_owned())
    }
}

impl Help for Context {
    fn help_text(&self) -> String {
        self.help_text.clone()
    }
}
