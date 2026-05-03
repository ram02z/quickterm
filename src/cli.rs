use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "quickterm")]
#[command(version)]
#[command(about = "A small drop-down terminal for sway/i3-compatible IPC")]
pub struct Args {
    #[arg(short = 'i', long = "in-place")]
    pub in_place: bool,

    #[arg(value_name = "SHELL")]
    pub shell: Option<String>,
}
