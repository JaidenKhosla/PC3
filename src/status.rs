use std::fmt;
use std::str::FromStr;

#[allow(non_camel_case_types)]
#[allow(unused)]
pub enum Status
{
    OK,
    ACCEPTED,
    COMPILATION_ERROR,
    RUNTIME_ERROR,
    WRONG_ANSWER,
    TIME_LIMIT_EXCEEDED,
    MEMORY_OVERFLOW,
}

impl Status
{
    pub fn to_string(&self) -> String
    {
        match self
        {
            Status::OK => "OK",
            Status::ACCEPTED => "ACCEPTED",
            Status::COMPILATION_ERROR => "COMPILATION_ERROR",
            Status::RUNTIME_ERROR => "RUNTIME_ERROR",
            Status::WRONG_ANSWER => "WRONG_ANSWER",
            Status::TIME_LIMIT_EXCEEDED => "TIME_LIMIT_EXCEEDED",
            Status::MEMORY_OVERFLOW => "MEMORY_OVERFLOW"
        }.to_string()
    }
}

impl FromStr for Status
{
    type Err = String;

    fn from_str(s: &str) -> Result<Status, Self::Err>
    {
        match s
        {
            "OK" => Ok(Status::OK),
            "ACCEPTED" => Ok(Status::ACCEPTED),
            "COMPILATION_ERROR" => Ok(Status::COMPILATION_ERROR),
            "RUNTIME_ERROR" => Ok(Status::RUNTIME_ERROR),
            "WRONG_ANSWER" => Ok(Status::RUNTIME_ERROR),
            "TIME_LIMIT_EXCEEDED" => Ok(Status::TIME_LIMIT_EXCEEDED),
            "MEMORY_OVERFLOW" => Ok(Status::MEMORY_OVERFLOW),
            _ => Err(format!("{} is an invalid status!", s))

        }
    }
}

impl fmt::Debug for Status
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        

        write!(f, "{}", self.to_string())
    }
}

impl fmt::Display for Status
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", self.to_string())
    }
}