use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

const SHELL_INTEGRATION: &str = r#"# Stargate - blockchain network switcher
sg() {
    case "$1" in
        travel|root)
            eval "$(command stargate "$@")"
            ;;
        *)
            command stargate "$@"
            ;;
    esac
}

#  Stargate - Set default anvil network
eval "$(command stargate travel anvil --silent)"
"#;

pub fn run(shell: &str) {
    match shell {
        "zsh" | "bash" => {
            install_to_shell_config(shell);
        }
        _ => {
            eprintln!("Unsupported shell: {}", shell);
            eprintln!("Supported shells: zsh, bash");
            std::process::exit(1);
        }
    }
}

fn get_shell_config_path(shell: &str) -> PathBuf {
    let home = dirs::home_dir().expect("Could not find home directory");
    match shell {
        "zsh" => home.join(".zshrc"),
        "bash" => home.join(".bashrc"),
        _ => unreachable!(),
    }
}

fn install_to_shell_config(shell: &str) {
    let config_path = get_shell_config_path(shell);

    // Check if already installed
    if let Ok(content) = fs::read_to_string(&config_path) {
        if content.contains("# Stargate - blockchain network switcher") {
            eprintln!(
                "Stargate is already installed in {}",
                config_path.display()
            );
            eprintln!("To reinstall, remove the existing Stargate section first.");
            return;
        }
    }

    // Append the shell integration
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&config_path)
        .expect("Failed to open shell config file");

    file.write_all(format!("\n{}", SHELL_INTEGRATION).as_bytes())
        .expect("Failed to write to shell config file");

    eprintln!("Stargate installed to {}", config_path.display());
    eprintln!(
        "Restart your shell or run: source {}",
        config_path.display()
    );
}
