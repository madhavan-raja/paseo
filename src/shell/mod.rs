use clap::ValueEnum;

mod bash;

pub trait Shell {
    /// Parses a raw PATH string into a Vec of individual paths based on shell rules
    fn parse_shell_path(&self, raw_path: &str) -> Vec<String>;
    
    /// Generates the shell-specific syntax to export the new PATH
    /// e.g., Bash: export PATH="a:b:c" | Fish: set -gx PATH "a" "b" "c"
    fn generate_shell_path(&self, paths: &[String]) -> String;
}

/// An enum representing the explicitly supported shells.
#[derive(ValueEnum, Clone, Debug, PartialEq)]
#[clap(rename_all = "lower")] // Ensures CLI inputs like "Bash" are parsed as "bash"
pub enum SupportedShell {
    Bash,
    Zsh,
    Fish,
    Nushell,
}

impl SupportedShell {
    /// Factory method to get the underlying trait implementation
    pub fn build(&self) -> Box<dyn Shell> {
        match self {
            SupportedShell::Bash => Box::new(bash::Bash),
            SupportedShell::Zsh => unimplemented!("Zsh not yet implemented"),
            SupportedShell::Fish => unimplemented!("Fish not yet implemented"),
            SupportedShell::Nushell => unimplemented!("Nushell not yet implemented"),
        }
    }
}