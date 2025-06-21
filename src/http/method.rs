use std::str::FromStr;

pub enum Method {
    Get,
    Delete,
    Post,
    Put,
}
pub struct MethodError;

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::Get),
            "DELETE" => Ok(Self::Delete),
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::Put),
            _ => Err(MethodError),
        }
    }
}
