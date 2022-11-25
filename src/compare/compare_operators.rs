use crate::record::{LogRecord, LogRecordTypes};
use regex::Regex;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Comparitors {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterOrEqualThan,
    LessOrEqualThan,
}

impl FromStr for Comparitors {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "==" => Ok(Comparitors::Equal),
            "!=" => Ok(Comparitors::NotEqual),
            ">" => Ok(Comparitors::GreaterThan),
            "<" => Ok(Comparitors::LessThan),
            ">=" => Ok(Comparitors::GreaterOrEqualThan),
            "<=" => Ok(Comparitors::LessOrEqualThan),
            _ => Err(anyhow::anyhow!(format!("Invalid comparitor: '{}'", s))),
        }
    }
}

impl Comparitors {
    pub fn get_regex() -> regex::Regex {
        Regex::new(r"\s?(==|!=|>=|<=|<|>)\s?").unwrap()
    }
    pub fn compare<T: PartialEq + PartialOrd>(&self, operand1: T, operand2: T) -> bool {
        match *self {
            Comparitors::Equal => operand1 == operand2,
            Comparitors::NotEqual => operand1 != operand2,
            Comparitors::GreaterThan => operand1 > operand2,
            Comparitors::LessThan => operand1 < operand2,
            Comparitors::GreaterOrEqualThan => operand1 >= operand2,
            Comparitors::LessOrEqualThan => operand1 <= operand2,
        }
    }
}

pub struct CompiledComparison {
    pub key: String,
    pub operator: Comparitors,
    pub value: String,
}

impl CompiledComparison {
    pub fn new(compare: &str) -> CompiledComparison {
        let re = Comparitors::get_regex();
        let mat = re
            .find(compare)
            .unwrap_or_else(|| panic!("Invalid comparison syntax of:{}", compare));
        let key: String = compare[0..mat.start()].to_string();
        let operator = Comparitors::from_str(compare[mat.start()..mat.end()].as_ref()).unwrap();
        let value: String = compare[mat.end()..].to_string();
        CompiledComparison {
            key,
            operator,
            value,
        }
    }
}

pub fn do_compare(r: &LogRecord, cc: &CompiledComparison) -> bool {
    match r.field_by_name(&cc.key) {
        Some(x) => match x {
            LogRecordTypes::U8(y) => {
                let u8_value: u8 = cc.value.parse().expect("getting u8 from value");
                cc.operator.compare(y, u8_value)
            }
            LogRecordTypes::Num(y) => {
                let f64_value: f64 = cc.value.parse().expect("getting u32 from value");
                cc.operator.compare(y, f64_value)
            }
            LogRecordTypes::Str(y) => cc.operator.compare(&y, &cc.value),
        },
        _ => false,
    }
}
