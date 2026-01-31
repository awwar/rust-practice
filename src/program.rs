const INTEGER_TYPE: &str = "integer";
const FLOAT_TYPE: &str = "float";
const BOOLEAN_TYPE: &str = "boolean";
const STRING_TYPE: &str = "string";

struct Value<T>(T);

// may be  union Val { f: f64, i: i64, b: bool } or enum Val { Int(u64), Float(f64), Boolean(bool), String(String) }

type IntegerValue = Value<i64>;
type FloatValue = Value<f64>;
type BoolValue = Value<bool>;
type StringValue = Value<String>;

trait ValueConverter {
    fn repr(&self) -> String;
    fn type_name(&self) -> &str;
    fn to_integer(self) -> Result<IntegerValue, String>;
    fn to_float(self) -> Result<FloatValue, String>;
    fn to_bool(self) -> Result<BoolValue, String>;
    fn to_string(self) -> Result<StringValue, String>;
    fn add(self, r: Self) -> Result<Self, String> where Self: Sized;
    fn subtract(self, r: Self) -> Result<Self, String> where Self: Sized;
    fn multiply(self, r: Self) -> Result<Self, String> where Self: Sized;
    fn divide(self, r: Self) -> Result<Self, String> where Self: Sized;
    fn power(self, r: Self) -> Result<Self, String> where Self: Sized;
    fn more(self, r: Self) -> Result<BoolValue, String>;
    fn less(self, r: Self) -> Result<BoolValue, String>;
    fn eq(self, r: Self) -> Result<BoolValue, String>;
}

#[rustfmt::skip]
impl ValueConverter for IntegerValue {
    fn repr(&self) -> String { self.0.to_string() }
    fn type_name(&self) -> &str { INTEGER_TYPE }
    fn to_integer(self) -> Result<IntegerValue, String> { Ok(self) }
    fn to_float(self) -> Result<FloatValue, String> { Ok(Value(self.0 as f64)) }
    fn to_bool(self) -> Result<BoolValue, String> { Ok(Value(self.0 != 0)) }
    fn to_string(self) -> Result<StringValue, String> { Ok(Value(self.0.to_string())) }
    fn add(self, r: Self) -> Result<Self, String> { Ok(Value(self.0 + r.0)) }
    fn subtract(self, r: Self) -> Result<Self, String> { Ok(Value(self.0 - r.0)) }
    fn multiply(self, r: Self) -> Result<Self, String> { Ok(Value(self.0 * r.0)) }
    fn divide(self, r: Self) -> Result<Self, String> { Ok(Value(self.0 / r.0)) }
    fn power(self, r: Self) -> Result<Self, String>{ Ok(Value(self.0 ^ r.0)) }
    fn more(self, r: Self) -> Result<BoolValue, String> { Ok(Value(self.0 > r.0)) }
    fn less(self, r: Self) -> Result<BoolValue, String> { Ok(Value(self.0 < r.0)) }
    fn eq(self, r: Self) -> Result<BoolValue, String> {Ok(Value(self.0 == r.0))}
}

#[rustfmt::skip]
impl ValueConverter for FloatValue {
    fn repr(&self) -> String { self.0.to_string() }
    fn type_name(&self) -> &str { FLOAT_TYPE }
    fn to_integer(self) -> Result<IntegerValue, String> { Ok(Value(self.0 as i64)) }
    fn to_float(self) -> Result<FloatValue, String> { Ok(Value(self.0)) }
    fn to_bool(self) -> Result<BoolValue, String> { Ok(Value(self.0 != 0.0)) }
    fn to_string(self) -> Result<StringValue, String> { Ok(Value(self.0.to_string())) }
    fn add(self, r: Self) -> Result<Self, String> { Ok(Value(self.0 + r.0)) }
    fn subtract(self, r: Self) -> Result<Self, String> { Ok(Value(self.0 - r.0)) }
    fn multiply(self, r: Self) -> Result<Self, String> { Ok(Value(self.0 * r.0)) }
    fn divide(self, r: Self) -> Result<Self, String> { Ok(Value(self.0 / r.0)) }
    fn power(self, r: Self) -> Result<Self, String>{ Ok(Value(self.0.powf(r.0))) }
    fn more(self, r: Self) -> Result<BoolValue, String> { Ok(Value(self.0 > r.0)) }
    fn less(self, r: Self) -> Result<BoolValue, String> { Ok(Value(self.0 < r.0)) }
    fn eq(self, r: Self) -> Result<BoolValue, String> {Ok(Value(self.0 == r.0))}
}

#[rustfmt::skip]
impl ValueConverter for BoolValue {
    fn repr(&self) -> String { self.0.to_string() }
    fn type_name(&self) -> &str { BOOLEAN_TYPE }
    fn to_integer(self) -> Result<IntegerValue, String> { Ok(Value(self.0 as i64)) }
    fn to_float(self) -> Result<FloatValue, String> { Ok(Value(self.0 as i64 as f64)) }
    fn to_bool(self) -> Result<BoolValue, String> { Ok(self) }
    fn to_string(self) -> Result<StringValue, String> { Ok(Value(self.0.to_string())) }
    fn add(self, r: Self) -> Result<Self, String> { Ok(Value(self.0 || r.0)) }
    fn subtract(self, _: Self) -> Result<Self, String> { Err("unable to subtract bool from bool".to_owned()) }
    fn multiply(self, r: Self) -> Result<Self, String> { Ok(Value(self.0 && r.0)) }
    fn divide(self, _: Self) -> Result<Self, String> { Err("unable to devide bool to bool".to_owned()) }
    fn power(self, r: Self) -> Result<Self, String>{ Ok(Value(self.0 ^ r.0)) }
    fn more(self, r: Self) -> Result<BoolValue, String> { Ok(Value(self.0 > r.0)) }
    fn less(self, r: Self) -> Result<BoolValue, String> { Ok(Value(self.0 < r.0)) }
    fn eq(self, r: Self) -> Result<BoolValue, String> {Ok(Value(self.0 == r.0))}
}

#[rustfmt::skip]
impl ValueConverter for StringValue {
    fn repr(&self) -> String { self.0.to_string() }
    fn type_name(&self) -> &str { STRING_TYPE }
    fn to_integer(self) -> Result<IntegerValue, String> {
        self.0.parse::<i64>().map(|t| Value(t)).map_err(|e| e.to_string())
    }
    fn to_float(self) -> Result<FloatValue, String> {
        self.0.parse::<f64>().map(|t| Value(t)).map_err(|e| e.to_string())
    }
    fn to_bool(self) -> Result<BoolValue, String> { Ok(Value(self.0.len() > 0)) }
    fn to_string(self) -> Result<StringValue, String> { Ok(self) }
    fn add(self, r: Self) -> Result<Self, String> { Ok(Value(self.0 + &*r.0)) }
    fn subtract(self, _: Self) -> Result<Self, String> { Err("unable to subtract string from string".to_owned()) }
    fn multiply(self, _: Self) -> Result<Self, String> { Err("unable to multiply string to string".to_owned()) }
    fn divide(self, _: Self) -> Result<Self, String> { Err("unable to divide string on string".to_owned()) }
    fn power(self, _: Self) -> Result<Self, String>{ Err("unable to power string to string".to_owned()) }
    fn more(self, r: Self) -> Result<BoolValue, String> { Ok(Value(self.0 > r.0)) }
    fn less(self, r: Self) -> Result<BoolValue, String> { Ok(Value(self.0 < r.0)) }
    fn eq(self, r: Self) -> Result<BoolValue, String> {Ok(Value(self.0 == r.0))}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_string_to_integer_when_ok() {
        let str_value = Value(String::from("999"));
        let integer_value = str_value.to_integer();

        assert!(integer_value.is_ok());

        let intval = integer_value.unwrap();

        assert_eq!(intval.0, 999);

        assert_eq!(intval.add(Value(1)).unwrap().0, 1000);
    }

    #[test]
    fn test_convert_string_to_integer_when_error() {
        let str_value = Value(String::from("asdasd"));
        let integer_value = str_value.to_integer();

        assert!(integer_value.is_err());
    }
}
