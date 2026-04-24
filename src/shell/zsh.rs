use crate::shell::Shell;

pub struct Zsh;

impl Shell for Zsh {
    fn parse_shell_path(&self, raw_path: &str) -> Vec<String> {
        raw_path
            .split(':')
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.to_string())
            .collect()
    }
    
    fn generate_shell_path(&self, paths: &[String]) -> String {
        paths.join(":")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_shell_path() {
        let zsh = Zsh;
        let raw = "/usr/local/bin:/usr/bin:/bin::/opt/homebrew/bin:";
        let parsed = zsh.parse_shell_path(raw);
        
        assert_eq!(
            parsed,
            vec![
                "/usr/local/bin",
                "/usr/bin",
                "/bin",
                "/opt/homebrew/bin"
            ]
        );
    }

    #[test]
    fn test_generate_shell_path() {
        let zsh = Zsh;
        let paths = vec![
            "/usr/local/bin".to_string(),
            "/usr/bin".to_string(),
            "/opt/custom dir/bin".to_string(),
        ];
        
        let path_string = zsh.generate_shell_path(&paths);
        
        assert_eq!(
            path_string, 
            "/usr/local/bin:/usr/bin:/opt/custom dir/bin"
        );
    }
}
