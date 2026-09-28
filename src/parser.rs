use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum Edition {
    Java,
    Bedrock
}

impl Edition {
    pub fn as_str(&self) -> &'static str {
        match self {
            Edition::Java => "java",
            Edition::Bedrock => "bedrock"
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    #[arg(short, long, value_enum, ignore_case = true, default_value_t = Edition::Java)]
    pub edition: Edition,

    #[arg(short, long, default_value = "test.mioclient.me")]
    pub ip: String
}

pub fn parse() -> Args {
    Args::parse()
}