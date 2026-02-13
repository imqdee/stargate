const SHELL_INTEGRATION: &str = r#"sg() {
    case "$1" in
        switch|sw|root)
            eval "$(command stargate "$@")"
            ;;
        *)
            command stargate "$@"
            ;;
    esac
}

# Set default anvil network
eval "$(command stargate switch anvil --silent)"
"#;

pub fn run(shell: &str) {
    match shell {
        "zsh" | "bash" => {
            print!("{}", SHELL_INTEGRATION);
        }
        _ => {
            eprintln!("Unsupported shell: {}", shell);
            eprintln!("Supported shells: zsh, bash");
            std::process::exit(1);
        }
    }
}
