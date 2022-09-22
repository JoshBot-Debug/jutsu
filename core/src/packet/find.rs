use std::fmt;

pub enum Execute {
    Restart,
    Shutdown
}

impl fmt::Debug for Execute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Execute::Restart => write!(f, "Restart"),
            Execute::Shutdown => write!(f, "Shutdown"),
        }
    }
}

impl Execute
{    
    pub fn to_byte(&self) -> &u8
    {
        match self {
            Execute::Restart => &98,
            Execute::Shutdown => &99,
        }
    }

    pub fn from_byte(byte: &[u8]) -> Result<Execute, ExecuteError>
    {
        match byte[0] {
            98 => Ok(Execute::Restart),
            99 => Ok(Execute::Shutdown),
            _ => Err(ExecuteError::new(format!("Failed to convert"), 9)),
        }
    }

    pub fn from_str(execute: &String) -> Result<Execute, ExecuteError> {
        match execute.to_lowercase().as_str() {
            "restart"  => Ok(Execute::Restart),
            "shutdown"  => Ok(Execute::Shutdown),
            _ => Err(ExecuteError::new(format!("Invalid execute command \"{execute}\"."), 8)),
        }
    }
}


pub struct ExecuteError
{
    message: String,
    pub code: i32
}

impl fmt::Display for ExecuteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ExecuteError\nError: {}\nCode: {}", self.message, self.code)
    }
}

impl ExecuteError {
    
    fn new(message: String, code: i32) -> Self
    {
        Self{message, code}
    }
}