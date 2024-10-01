use std::fmt;

pub enum TypeVal {
    text(String),
    number(f64),
    boolean(bool),
    null,
    object,
    array
}


pub struct Data {
    key: String,
    value: TypeVal
}

impl Data {

    pub fn new(k: String, val: TypeVal) -> Data {
        Data {
            key:    k,
            value:  val
        }
    }
}

impl fmt::Display for Data {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let val = match &self.value {
            TypeVal::text(s)    => format!("\"{s}\""),
            TypeVal::number(n)     => format!("{n}"),
            TypeVal::boolean(b)   => format!("{b}"),
            TypeVal::null                => format!("null"),
            TypeVal::object              => format!("object"),
            TypeVal::array               => format!("array"),
        };

        write!(f, "{{\"{}\": {}}}", self.key, val)
    }

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text() {
        let text = Data::new(String::from("text_test"), TypeVal::text(String::from("text")));
        assert_eq!("{\"text_test\": \"text\"}", text.to_string());
    }

    #[test]
    fn test_number() {
        let number = Data::new(String::from("number"), TypeVal::number(1.0));
        assert_eq!("{\"number\": 1}", number.to_string());
        let number = Data::new(String::from("number"), TypeVal::number(2.23));
        assert_eq!("{\"number\": 2.23}", number.to_string());
    }

    #[test]
    fn test_bool() {
        let b_true = Data::new(String::from("b_true"), TypeVal::boolean(true));
        assert_eq!("{\"b_true\": true}", b_true.to_string());
        let b_false = Data::new(String::from("b_false"), TypeVal::boolean(false));
        assert_eq!("{\"b_false\": false}", b_false.to_string());
    }

    #[test]
    fn test_null() {
        let null = Data::new(String::from("null"), TypeVal::null);
        assert_eq!("{\"null\": null}", null.to_string());
    }
    // let data4 = Data::new(String::from("null"), TypeVal::null);
    // println!("{}", data4.to_string());

}