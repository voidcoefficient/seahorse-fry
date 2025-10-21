#[derive(Clone, Default, Debug)]
pub struct Flag {
    pub name: String,
    pub description: Option<String>,
    pub flag_type: FlagType,
}

#[derive(PartialEq, Clone, Debug, Default)]
pub enum FlagType {
    Int,
    Uint,
    Float,
    String,
    #[default]
    Bool,
}

impl Flag {
    pub fn new<S: Into<String>>(name: S, flag_type: FlagType) -> Self {
        let name: String = name.into();
        if name.starts_with("-") || name.contains("=") || name.contains(" ") {
            panic!(r#"invalid flag name: "{}"#, name);
        }

        Self {
            name,
            flag_type,
            ..Default::default()
        }
    }

    pub fn description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = Some(description.into());
        self
    }
}
