use crate::{
    flag::{Flag, FlagType, FlagValue},
    help::Help,
    utils::normalize_args,
};

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
            .filter(|arg| !arg.starts_with("-") && !self.flags().contains(arg))
            .map(|arg| arg.to_owned())
            .collect()
    }

    pub fn value(&self) -> Option<String> {
        self.values().first().map(|value| value.to_owned())
    }

    pub fn get_flag_value<S: Into<String>>(
        &self,
        flag_name: S,
        flag_type: FlagType,
    ) -> Option<FlagValue> {
        let flag_name = flag_name.into();

        match self
            .args
            .iter()
            .position(|arg| arg == &format!("-{}", flag_name))
        {
            Some(pos) => Flag::extract_value(self.args.get(pos + 1).cloned(), flag_type),
            None => None,
        }
    }

    pub fn get_flag_values<S: Into<String>>(&self, flag_name: S) -> Vec<String> {
        let flag_name = flag_name.into();

        self.args
            .iter()
            .enumerate()
            .filter(|(_i, arg)| *arg == &format!("-{}", flag_name))
            .map(|(i, _arg)| self.args.get(i + 1).cloned().unwrap())
            .collect::<Vec<String>>()
    }

    pub fn get_boolean_flag<S: Into<String>>(&self, flag_name: S) -> bool {
        let flag_name = flag_name.into();
        self.flags()
            .iter()
            .any(|flag| flag == &format!("-{}", flag_name))
    }
}

impl Help for Context {
    fn help_text(&self) -> String {
        self.help_text.clone()
    }
}
