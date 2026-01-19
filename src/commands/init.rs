pub fn run(shell: &str) {
    match shell {
        "zsh" | "bash" => print_sh_init(),
        _ => {
            eprintln!("Unsupported shell: {}", shell);
            eprintln!("Supported shells: zsh, bash");
            std::process::exit(1);
        }
    }
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
