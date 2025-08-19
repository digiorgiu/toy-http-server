use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'a> {
    data: HashMap<&'a str, Value<'a>>,
}

#[derive(Debug)]
pub enum Value<'a> {
    Single(&'a str),
    Multiple(Vec<&'a str>),
}

impl<'a> QueryString<'a> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

// a=1&b=2&c&d=&e===&d=7&d=abc

impl<'a> From<&'a str> for QueryString<'a> {
    fn from(s: &'a str) -> Self {
        println!("{}", s);
        let mut data = HashMap::new();

        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }

            data.entry(key)
                .and_modify(|existing| match existing {
                    Value::Single(prev_val) => {
                        let new_vec: Vec<&str> = vec![prev_val, val];
                        *existing = Value::Multiple(new_vec);
                    }
                    Value::Multiple(vec) => {
                        vec.push(val);
                    }
                })
                .or_insert(Value::Single(val));
        }

        QueryString { data }
    }
}
