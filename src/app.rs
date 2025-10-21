use crate::{action::Action, command::Command, help::Help};

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
        if args.len() == 0usize {
            self.help();
            std::process::exit(1);
        }

        self.help();
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
            Some(usage) => result += &format!("Usage:\n\t{}\n\n", usage),
            None => result += &format!("Usage:\n\t{} --help\n\n", self.name),
        }

        if let Some(commands) = &self.commands {
            result += "Commands:\n";
            commands.iter().for_each(|c| match &c.description {
                Some(description) => result += &format!("\t{}\t{}\n", c.name, description),
                None => result += &format!("\t{}\n", c.name),
            });
        }

        result
    }
}
