use crate::action::Action;

#[derive(Default)]
pub struct Command {
    pub name: String,
    pub description: Option<String>,
    pub usage: Option<String>,
    pub action: Option<Action>,
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
}
