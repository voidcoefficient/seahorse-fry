use crate::{action::Action, command::Command, context::Context, help::Help};

#[derive(Default)]
pub struct App {
    pub name: String,
    pub description: Option<String>,
    pub usage: Option<String>,
    pub commands: Option<Vec<Command>>,
    pub action: Option<Action>,
}

impl App {
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

    pub fn command(mut self, command: Command) -> Self {
        if let Some(ref mut commands) = self.commands {
            if commands
                .iter()
                .any(|registered| registered.name == command.name)
            {
                panic!(r#"command {} already exists"#, command.name);
            }
            (*commands).push(command);
        } else {
            self.commands = Some(vec![command]);
        }

        self
    }

    pub fn action(mut self, action: Action) -> Self {
        self.action = Some(action);
        self
    }

    pub fn run(&self, args: Vec<String>) {
        if args.len() == 1usize {
            self.help();
            std::process::exit(1);
        }

        let (_bin_path, args) = args.split_first().unwrap();
        let (command_name, rest) = match args.to_vec().len() {
            1 => args.split_at(1),
            _ => args.split_at(1),
        };
        let command_name = match command_name.first() {
            Some(command_name) => command_name,
            None => {
                self.help();
                std::process::exit(1);
            }
        };

        match self.commands {
            Some(ref commands) => {
                match commands.iter().find(|c| &c.name == command_name) {
                    Some(command) => match command.action {
                        Some(action) => {
                            action(&Context::new(args.to_vec(), self.help_text()));
                        }
                        None => {
                            self.help();
                            std::process::exit(1);
                        }
                    },
                    None => {}
                };
            }
            None => {
                self.help();
                std::process::exit(1);
            }
        }

        std::process::exit(0);
    }
}

impl Help for App {
    fn help_text(&self) -> String {
        let mut result = String::new();

        if let Some(description) = &self.description {
            result += &format!("{}\n\n", description);
        }

        match &self.usage {
            Some(usage) => result += &format!("usage\n\t{}\n\n", usage),
            None => result += &format!("usage\n\t{} --help\n\n", self.name),
        }

        if let Some(commands) = &self.commands {
            result += "commands\n";
            commands.iter().for_each(|c| match &c.description {
                Some(description) => result += &format!("\t{}\t{}\n", c.name, description),
                None => result += &format!("\t{}\n", c.name),
            });
        }

        result
    }
}
