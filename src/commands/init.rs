use crate::config::Config;

pub fn run(shell: &str) {
    match shell {
        "zsh" | "bash" => {
            let config = Config::load();
            let default_network = config.get_default_network();

            let shell_integration = format!(
                r#"sg() {{
    case "$1" in
        switch|sw|root)
            eval "$(command stargate "$@")"
            ;;
        *)
            command stargate "$@"
            ;;
    esac
}}

# Set default network
eval "$(command stargate switch {} --silent)"
"#,
                default_network
            );

            print!("{}", shell_integration);
        }
        _ => {
            eprintln!("Unsupported shell: {}", shell);
            eprintln!("Supported shells: zsh, bash");
            std::process::exit(1);
        }
    }
}
