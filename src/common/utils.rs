use colored::*;
use std::time::Duration;

pub struct Config {
    pub iteration_delay: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            iteration_delay: Duration::from_secs(1),
        }
    }
}

pub struct ColoredText;

impl ColoredText {
    pub fn blue(text: &str) -> String {
        text.blue().to_string()
    }

    pub fn green(text: &str) -> String {
        text.green().to_string()
    }

    pub fn yellow(text: &str) -> String {
        text.yellow().to_string()
    }

    pub fn red(text: &str) -> String {
        text.red().to_string()
    }

    pub fn cyan(text: &str) -> String {
        text.cyan().to_string()
    }

    pub fn bold_cyan(text: &str) -> String {
        text.cyan().bold().to_string()
    }
}
