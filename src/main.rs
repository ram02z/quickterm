use clap::Parser;
use quickterm::cli::Args;
use quickterm::config::load_config;

fn main() {
    let args = Args::parse();

    let config = match load_config() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    };

    if let Some(shell) = args.shell.as_deref() {
        if !config.shells.contains_key(shell) {
            eprintln!("unknown shell: {shell}");
            std::process::exit(1);
        }
    }

    if let Err(err) = quickterm::app::run(config, args.shell, args.in_place) {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
