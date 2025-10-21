use crate::{action::Action, flag::Flag, help::Help};

#[derive(Default, Debug, Clone)]
pub struct Command {
    pub name: String,
    pub description: Option<String>,
    pub usage: Option<String>,
    pub action: Option<Action>,
    pub flags: Option<Vec<Flag>>,
    pub subcommands: Option<Vec<Command>>,
}

impl Command {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            ..Self::default()
        }
    }

    pub fn description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn usage<S: Into<String>>(mut self, usage: S) -> Self {
        self.usage = Some(usage.into());
        self
    }

    pub fn action(mut self, action: Action) -> Self {
        self.action = Some(action);
        self
    }

    pub fn subcommand(mut self, subcommand: Command) -> Self {
        if let Some(ref mut subcommands) = self.subcommands {
            if subcommands
                .iter()
                .any(|registered| registered.name == subcommand.name)
            {
                panic!(r#"subcommand {} already exists"#, subcommand.name);
            }
            (*subcommands).push(subcommand);
        } else {
            self.subcommands = Some(vec![subcommand]);
        }

        self
    }

    pub fn flag(mut self, flag: Flag) -> Self {
        match self.flags {
            Some(ref mut flags) => {
                (*flags).push(flag);
            }
            None => {
                self.flags = Some(vec![flag]);
            }
        };
        self
    }

    /// makes sure that we are as deep as we can to proceed with actions
    pub fn select_command(&self, args: Vec<String>) -> Self {
        if args.is_empty() {
            return self.clone();
        }

        match self.subcommands {
            Some(ref subcommands) => match args.first() {
                Some(subcommand_name) => {
                    match subcommands
                        .iter()
                        .find(|subcommand| &subcommand.name == subcommand_name)
                    {
                        Some(subcommand) => subcommand.clone().select_command(args[1..].to_vec()),
                        None => self.clone(),
                    }
                }
                None => self.clone(),
            },
            None => self.clone(),
        }
    }
}

impl Help for Command {
    fn help_text(&self) -> String {
        let mut result = String::new();

        match self.description {
            Some(ref description) => result += &format!("{}\n\n", description),
            None => {}
        };

        match self.usage {
            Some(ref usage) => result += &format!("usage\n\t{}\n\n", usage),
            None => {}
        };

        match self.subcommands {
            Some(ref subcommands) => {
                result += &format!("subcommands\n");
                subcommands
                    .iter()
                    .for_each(|subcommand| match &subcommand.description {
                        Some(description) => {
                            result += &format!("\t{}\t{}\n", subcommand.name, description)
                        }
                        None => result += &format!("\t{}\n", subcommand.name),
                    });
                result += "\n";
            }
            None => {}
        };

        match &self.flags {
            Some(flags) => {
                result += &format!("flags");
                flags.iter().for_each(|flag| match &flag.description {
                    Some(description) => result += &format!("\n\t{}\t{}", flag.name, description),
                    None => result += &format!("\n\t{}", flag.name),
                });
            }
            None => result += &format!("flags\n\t--help\tshows this help page"),
        };

        result
    }
}
