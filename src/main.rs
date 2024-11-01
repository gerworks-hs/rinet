use clap::Parser;

const VERSION: &str = "1.0.0";

#[derive(Parser)]
struct Cli {
    #[clap(short, long, help = "Target IP address to connect to", required_unless_present = "listening")]
    target: String,
    #[clap(short, long, help = "Will rinet run as server?", required = false)]
    listening: bool,
    #[clap(short, long, help = "Increase verbosity", required = false)]
    verbose: bool,
    #[clap(short, long, help = "Show rinet info", required = false)]
    info: bool
}

fn main() {
    let cli_args = Cli::parse();
}