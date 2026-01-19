use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

pub fn run(shell: &str, install: bool) {
    match shell {
        "zsh" | "bash" => {
            if install {
                install_to_shell_config(shell);
            } else {
                print_sh_init();
            }
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
        if content.contains("stargate init") {
            eprintln!(
                "Stargate is already installed in {}",
                config_path.display()
            );
            eprintln!("To reinstall, remove the existing line first.");
            return;
        }
    }

    // Append the eval line
    let line = format!(
        "\n# Stargate - blockchain network switcher\neval \"$(stargate init {})\"\n",
        shell
    );

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&config_path)
        .expect("Failed to open shell config file");

    file.write_all(line.as_bytes())
        .expect("Failed to write to shell config file");

    eprintln!("Stargate installed to {}", config_path.display());
    eprintln!(
        "Restart your shell or run: source {}",
        config_path.display()
    );
}

fn print_sh_init() {
    print!(
        r#"# Stargate shell integration
# Add to your ~/.zshrc or ~/.bashrc:
#   eval "$(stargate init zsh)"

travel() {{
    eval "$(command stargate travel "$@")"
}}

root() {{
    eval "$(command stargate travel anvil)"
}}

current() {{
    command stargate current
}}

explorer() {{
    command stargate explorer "$@"
}}

# Initialize with anvil on shell startup
eval "$(command stargate travel anvil)"
"#
    );
}
