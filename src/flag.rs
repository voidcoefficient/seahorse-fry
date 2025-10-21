#[derive(Clone, Default, Debug)]
pub struct Flag {
    pub name: String,
    pub description: Option<String>,
    pub flag_type: FlagType,
}

#[derive(PartialEq, Copy, Clone, Debug, Default)]
pub enum FlagType {
    Int,
    Uint,
    Float,
    String,
    #[default]
    Bool,
}

#[derive(PartialEq, Clone, Debug)]
pub enum FlagValue {
    Int(isize),
    Uint(usize),
    Float(f64),
    String(String),
    Bool(bool),
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

    pub fn extract_value(value: Option<String>, flag_type: FlagType) -> Option<FlagValue> {
        match value {
            Some(value) => match flag_type {
                FlagType::Int => Some(FlagValue::Int(value.parse::<isize>().expect("invalid"))),
                FlagType::Uint => Some(FlagValue::Uint(value.parse::<usize>().expect("invalid"))),
                FlagType::Float => Some(FlagValue::Float(value.parse::<f64>().expect("invalid"))),
                FlagType::String => Some(FlagValue::String(value)),
                FlagType::Bool => match value.as_str() {
                    "false" => Some(FlagValue::Bool(false)),
                    _ => Some(FlagValue::Bool(true)),
                },
            },
            None => match flag_type {
                FlagType::Bool => Some(FlagValue::Bool(true)),
                _ => None,
            },
        }
    }

    pub fn get_value(&self, args: Vec<String>) -> Option<FlagValue> {
        match args
            .iter()
            .position(|arg| arg == &format!("-{}", self.name))
        {
            Some(pos) => Flag::extract_value(args.get(pos + 1).cloned(), self.flag_type),
            None => None,
        }
    }
}
