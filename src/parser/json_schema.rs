pub trait ToRegex {
    fn to_regex(&self) -> String;
}

pub struct KeyValueType {
    key: String,
    value: Box<JsonType>,
}

impl KeyValueType {
    pub fn new(key: String, value: JsonType) -> Self {
        Self {
            key,
            value: Box::new(value),
        }
    }
}

impl ToRegex for KeyValueType {
    fn to_regex(&self) -> String {
        format!("\"{}\": {}", self.key, self.value.to_regex())
    }
}

pub enum JsonType {
    Boolean { regex: String },
    Integer { regex: String },
    String { regex: String },
    Array(Box<JsonType>),
    Object(Vec<KeyValueType>),
    Union(Vec<JsonType>),
}

impl JsonType {
    pub fn boolean() -> Self {
        Self::Boolean {
            regex: r"(true|false)".to_string(),
        }
    }

    pub fn integer_with_regex(regex: &str) -> Self {
        Self::Integer {
            regex: format!("({regex})"),
        }
    }

    pub fn integer() -> Self {
        Self::integer_with_regex(r"\d+")
    }

    pub fn string_with_regex(regex: &str) -> Self {
        Self::String {
            regex: format!("\"({regex})\""),
        }
    }

    pub fn string() -> Self {
        Self::string_with_regex("[^\"]*")
    }

    pub fn array(item_type: JsonType) -> Self {
        Self::Array(Box::new(item_type))
    }

    pub fn key_value(key: &str, value: JsonType) -> KeyValueType {
        KeyValueType::new(key.to_string(), value)
    }

    pub fn object(items: Vec<KeyValueType>) -> Self {
        Self::Object(items)
    }

    pub fn union(items: Vec<JsonType>) -> Self {
        Self::Union(items)
    }
}

impl ToRegex for JsonType {
    fn to_regex(&self) -> String {
        match self {
            JsonType::Boolean { regex } | JsonType::Integer { regex } | JsonType::String { regex } => regex.to_string(),
            JsonType::Array(items) => format!(
                "\\[({}(, {})*)?\\]",
                items.to_regex(),
                items.to_regex(),
            ),
            JsonType::Object(items) => format!(
                "\\{{ {} \\}}",
                items.iter().map(|item| item.to_regex()).collect::<Vec<_>>().join(", "),
            ),
            JsonType::Union(items) => format!(
                "({})",
                items.iter().map(|item| format!("({})", item.to_regex())).collect::<Vec<_>>().join("|"),
            ),
        }
    }
}
