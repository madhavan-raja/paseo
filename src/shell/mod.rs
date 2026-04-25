use clap::ValueEnum;
use strum::Display;

mod bash;
mod zsh;

pub trait Shell {
    fn parse_shell_path(&self, raw_path: &str) -> Vec<String>;
    fn generate_shell_path(&self, paths: &[String]) -> String;
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Display)]
#[strum(serialize_all = "lowercase")]
#[clap(rename_all = "lower")]
pub enum SupportedShell {
    Bash,
    Zsh,
    Fish,
    Nu,
}

impl SupportedShell {
    pub fn build(&self) -> Box<dyn Shell> {
        match self {
            SupportedShell::Bash => Box::new(bash::Bash),
            SupportedShell::Zsh => Box::new(zsh::Zsh),
            SupportedShell::Fish => unimplemented!("Fish not yet implemented"),
            SupportedShell::Nu => unimplemented!("Nushell not yet implemented"),
        }
    }
}