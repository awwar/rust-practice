#[derive(Clone, Debug)]
pub enum Value {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    Array(Vec<Value>),
}

impl Value {
    pub fn repr(&self) -> String {
        match self {
            Value::Integer(a) => a.to_string(),
            Value::Float(a) => a.to_string(),
            Value::Boolean(a) => a.to_string(),
            Value::String(a) => a.clone(),
            Value::Array(a) => format!("[{}]", a.iter().map(|v| v.repr()).collect::<Vec<_>>().join(",")),
            _ => panic!("unable to int({self:?})")
        }
    }
    pub fn to_integer(&self) -> Value {
        match self {
            Value::Integer(a) => Value::Integer(*a),
            Value::Float(a) => Value::Integer(*a as i64),
            Value::Boolean(a) => Value::Integer(i64::from(*a)),
            Value::String(a) => Value::Integer(a.parse::<i64>().unwrap()),
            _ => panic!("unable to int({self:?})")
        }
    }
    pub fn to_float(&self) -> Value {
        match self {
            Value::Integer(a) => Value::Float(*a as f64),
            Value::Float(a) => Value::Float(*a),
            Value::Boolean(a) => Value::Float(i64::from(*a) as f64),
            Value::String(a) => Value::Float(a.parse::<f64>().unwrap()),
            _ => panic!("unable to float({self:?})")
        }
    }
    pub fn to_bool(&self) -> Value {
        match self {
            Value::Integer(a) => Value::Boolean(a > &0),
            Value::Float(a) => Value::Boolean(a > &0.0),
            Value::Boolean(a) => Value::Boolean(*a),
            Value::String(a) => Value::Boolean(!a.is_empty()),
            _ => panic!("unable to bool({self:?})")
        }
    }
    pub fn to_string(&self) -> Value {
        match self {
            Value::Integer(a) => Value::String(a.to_string()),
            Value::Float(a) => Value::String(a.to_string()),
            Value::Boolean(a) => Value::String(a.to_string()),
            Value::String(a) => Value::String(a.clone()),
            _ => panic!("unable to string({self:?})")
        }
    }
    pub fn add(&self, r: &Self) -> Value {
        match (self, r) {
            (Value::Integer(a), Value::Integer(b)) => Value::Integer(a + b),
            (Value::Float(a), Value::Float(b)) => Value::Float(a + b),
            (Value::String(a), Value::String(b)) => {
                let mut combined = String::with_capacity(a.len() + b.len());

                combined.push_str(a);
                combined.push_str(b);

                Value::String(combined)
            },
            (Value::Boolean(a), Value::Boolean(b)) => Value::Boolean(*a || *b),
            (Value::Array(a), Value::Array(b)) => {
                let mut new_val = a.clone();
                new_val.extend(b.clone());
                return Value::Array(new_val);
            },
            _ => panic!("unable to {} + {}", self.repr(), r.repr())
        }
    }
    pub fn subtract(&self, r: &Self) -> Value {
        match (self, r) {
            (Value::Integer(a), Value::Integer(b)) => Value::Integer(a - b),
            (Value::Float(a), Value::Float(b)) => Value::Float(a - b),
            _ => panic!("unable to {:?} - {:?}", self.repr(), r.repr())
        }
    }
    pub fn multiply(&self, r: &Self) -> Value {
        match (self, r) {
            (Value::Integer(a), Value::Integer(b)) => Value::Integer(a * b),
            (Value::Float(a), Value::Float(b)) => Value::Float(a * b),
            (Value::Boolean(a), Value::Boolean(b)) => Value::Boolean(*a && *b),
            _ => panic!("unable to {:?} * {:?}", self.repr(), r.repr())
        }
    }
    pub fn divide(&self, r: &Self) -> Value {
        match (self, r) {
            (Value::Integer(a), Value::Integer(b)) => Value::Integer(a / b),
            (Value::Float(a), Value::Float(b)) => Value::Float(a / b),
            _ => panic!("unable to {:?} / {:?}", self.repr(), r.repr())
        }
    }
    pub fn power(&self, r: &Self) -> Value {
        match (self, r) {
            (Value::Integer(a), Value::Integer(b)) => Value::Integer(a ^ b),
            (Value::Float(a), Value::Float(b)) => Value::Float(a.powf(*b)),
            _ => panic!("unable to {:?} ^ {:?}", self.repr(), r.repr())
        }
    }
    pub fn more(&self, r: &Self) -> Value {
        match (self, r) {
            (Value::Integer(a), Value::Integer(b)) => Value::Boolean(a < b),
            (Value::Float(a), Value::Float(b)) => Value::Boolean(a < b),
            _ => panic!("unable to {:?} > {:?}", self.repr(), r.repr())
        }
    }
    pub fn less(&self, r: &Self) -> Value {
        match (self, r) {
            (Value::Integer(a), Value::Integer(b)) => Value::Boolean(a < b),
            (Value::Float(a), Value::Float(b)) => Value::Boolean(a < b),
            _ => panic!("unable to {:?} < {:?}", self.repr(), r.repr())
        }
    }
    pub fn eq(&self, r: &Self) -> Value {
        match (self, r) {
            (Value::Integer(a), Value::Integer(b)) => Value::Boolean(a == b),
            (Value::Float(a), Value::Float(b)) => Value::Boolean(a == b),
            (Value::Boolean(a), Value::Boolean(b)) => Value::Boolean(a == b),
            (Value::String(a), Value::String(b)) => Value::Boolean(a == b),
            _ => panic!("unable to {:?} == {:?}", self.repr(), r.repr())
        }
    }
}