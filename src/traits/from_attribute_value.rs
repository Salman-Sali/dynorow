use std::collections::{HashMap, HashSet};

use aws_sdk_dynamodb::types::AttributeValue;

use crate::{error::Error, traits::dyno_map_trait::DynoMapTrait};

pub trait FromAttributeValue<T> {
    fn from_attribute_value(value: AttributeValue) -> Result<T, Error>;
}

impl<T> FromAttributeValue<T> for T
where
    T: TryFrom<HashMap<String, AttributeValue>, Error = Error> + DynoMapTrait,
{
    fn from_attribute_value(value: AttributeValue) -> Result<T, Error> {
        if let AttributeValue::M(hash_map) = value {
            return hash_map.try_into();
        }
        return Err(Error::parse_error(value, "T", String::new()));
    }
}

impl<T> FromAttributeValue<Option<T>> for Option<T>
where
    T: FromAttributeValue<T>,
{
    fn from_attribute_value(value: AttributeValue) -> Result<Option<T>, Error> {
        if value.is_null() {
            return Ok(None);
        } else {
            return T::from_attribute_value(value).map(|x| Some(x));
        }
    }
}

impl<T> FromAttributeValue<Vec<T>> for Vec<T>
where
    T: FromAttributeValue<T>,
{
    fn from_attribute_value(value: AttributeValue) -> Result<Vec<T>, Error> {
        if let AttributeValue::L(list) = value {
            let mut result: Vec<T> = vec![];
            for item in list {
                result.push(<T as FromAttributeValue<T>>::from_attribute_value(item)?);
            }
            return Ok(result);
        }
        return Err(Error::parse_error(value, "Vec<T>", String::new()));
    }
}

impl FromAttributeValue<String> for String {
    fn from_attribute_value(value: AttributeValue) -> Result<String, Error> {
        if let Ok(value) = value.as_s() {
            return Ok(value.clone());
        }
        return Err(Error::parse_error(value, "String", String::new()));
    }
}

impl FromAttributeValue<i32> for i32 {
    fn from_attribute_value(value: AttributeValue) -> Result<i32, Error> {
        if let Ok(number) = value.as_n() {
            if let Ok(parsed_value) = number.parse::<i32>() {
                return Ok(parsed_value);
            }
        }
        return Err(Error::parse_error(value, "i32", String::new()));
    }
}

impl FromAttributeValue<u32> for u32 {
    fn from_attribute_value(value: AttributeValue) -> Result<u32, Error> {
        if let Ok(number) = value.as_n() {
            if let Ok(parsed_value) = number.parse::<u32>() {
                return Ok(parsed_value);
            }
        }
        return Err(Error::parse_error(value, "u32", String::new()));
    }
}

impl FromAttributeValue<f32> for f32 {
    fn from_attribute_value(value: AttributeValue) -> Result<f32, Error> {
        if let Ok(number) = value.as_n() {
            if let Ok(parsed_value) = number.parse::<f32>() {
                return Ok(parsed_value);
            }
        }
        return Err(Error::parse_error(value, "f32", String::new()));
    }
}

impl FromAttributeValue<bool> for bool {
    fn from_attribute_value(value: AttributeValue) -> Result<bool, Error> {
        if let Ok(boolean) = value.as_bool() {
            return Ok(*boolean);
        }
        return Err(Error::parse_error(value, "bool", String::new()));
    }
}

impl FromAttributeValue<HashSet<String>> for HashSet<String> {
    fn from_attribute_value(value: AttributeValue) -> Result<HashSet<String>, Error> {
        if value.is_null() {
            return Ok(HashSet::new());
        }

        if let AttributeValue::Ss(set) = value {
            return Ok(set.into_iter().collect());
        }
        return Err(Error::parse_error(value, "HashSet<String>", String::new()));
    }
}

impl FromAttributeValue<HashSet<i32>> for HashSet<i32> {
    fn from_attribute_value(value: AttributeValue) -> Result<HashSet<i32>, Error> {
        if value.is_null() {
            return Ok(HashSet::new());
        }

        if let Ok(numeric_set) = value.as_ns() {
            let mut result: HashSet<i32> = HashSet::new();
            for item in numeric_set {
                let number = item.parse::<i32>().map_err(|e| {
                    Error::parse_error(value.clone(), "HashSet<i32>", format!("{:?}", e))
                })?;
                result.insert(number);
            }
            return Ok(result);
        }
        return Err(Error::parse_error(value, "HashSet<i32>", String::new()));
    }
}

impl FromAttributeValue<HashSet<u32>> for HashSet<u32> {
    fn from_attribute_value(value: AttributeValue) -> Result<HashSet<u32>, Error> {
        if value.is_null() {
            return Ok(HashSet::new());
        }

        if let Ok(numeric_set) = value.as_ns() {
            let mut result: HashSet<u32> = HashSet::new();
            for item in numeric_set {
                let number = item.parse::<u32>().map_err(|e| {
                    Error::parse_error(value.clone(), "Result<HashSet<u32>", format!("{:?}", e))
                })?;
                result.insert(number);
            }
            return Ok(result);
        }
        return Err(Error::parse_error(
            value,
            "Result<HashSet<u32>",
            String::new(),
        ));
    }
}
