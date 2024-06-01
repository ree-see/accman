use clap::{Command, Arg, ArgAction};

fn build_accman() -> Command {
    Command::new("accman")
        .version("0.1.0")
        .about("An account manager from the terminal")
            .arg(Arg::new("create")
                .action(ArgAction::SetTrue))
}
fn main() {
    let cli = build_accman().get_matches();
}

