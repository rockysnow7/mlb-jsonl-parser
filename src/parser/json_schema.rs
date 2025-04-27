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
        Self::boolean_with_options(vec![true, false])
    }

    pub fn boolean_with_options(options: Vec<bool>) -> Self {
        let regex = options.iter().map(|b| b.to_string()).collect::<Vec<_>>().join("|");
        Self::Boolean {
            regex: format!("({regex})"),
        }
    }

    pub fn integer_with_regex(regex: &str) -> Self {
        Self::Integer {
            regex: format!("({regex})"),
        }
    }

    pub fn integer() -> Self {
        Self::integer_with_regex(r"[1-9]\d+")
    }

    pub fn integer_max_digits(max_digits: usize) -> Self {
        Self::integer_with_regex(&format!(r"[1-9]\d{{0,{}}}", max_digits - 1))
    }

    pub fn integer_with_options(options: Vec<usize>) -> Self {
        Self::integer_with_regex(&options.iter().map(|i| i.to_string()).collect::<Vec<_>>().join("|"))
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
                "\\[{}(, {})*\\]",
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_array_to_regex() {
        let array = JsonType::array(JsonType::integer());
        assert_eq!(array.to_regex(), r"\[(([1-9]\d+)(, ([1-9]\d+))*)?\]");
    }
}
