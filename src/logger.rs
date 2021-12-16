use std::{cmp::Ordering, fmt::Display};

pub struct Logger {
    log_level: LogLevel,
}
impl Logger {
    pub fn new(log_level: LogLevel) -> Self {
        Self { log_level }
    }
    pub fn verb<T>(&self, msg: T)
    where
        T: Display,
    {
        if self.log_level <= LogLevel::Verbose {
            println!("{}", msg);
        }
    }
    pub fn log<T>(&self, msg: T)
    where
        T: Display,
    {
        if self.log_level <= LogLevel::Log {
            println!("{}", msg);
        }
    }
    pub fn warn<T>(&self, msg: T)
    where
        T: Display,
    {
        if self.log_level <= LogLevel::Warn {
            println!("{}", msg);
        }
    }
    pub fn error<T>(&self, msg: T)
    where
        T: Display,
    {
        if self.log_level <= LogLevel::Error {
            println!("{}", msg);
        }
    }
}
pub enum LogLevel {
    Verbose,
    Log,
    Warn,
    Error,
}

impl PartialEq for LogLevel {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}
impl PartialOrd<LogLevel> for LogLevel {
    fn partial_cmp(&self, other: &LogLevel) -> Option<std::cmp::Ordering> {
        Some(match self {
            LogLevel::Verbose => match other {
                LogLevel::Verbose => Ordering::Equal,
                LogLevel::Log => Ordering::Less,
                LogLevel::Warn => Ordering::Less,
                LogLevel::Error => Ordering::Less,
            },
            LogLevel::Log => match other {
                LogLevel::Verbose => Ordering::Greater,
                LogLevel::Log => Ordering::Equal,
                LogLevel::Warn => Ordering::Less,
                LogLevel::Error => Ordering::Less,
            },
            LogLevel::Warn => match other {
                LogLevel::Verbose => Ordering::Greater,
                LogLevel::Log => Ordering::Greater,
                LogLevel::Warn => Ordering::Equal,
                LogLevel::Error => Ordering::Less,
            },

            LogLevel::Error => match other {
                LogLevel::Verbose => Ordering::Greater,
                LogLevel::Log => Ordering::Greater,
                LogLevel::Warn => Ordering::Greater,
                LogLevel::Error => Ordering::Equal,
            },
        })
    }
}
