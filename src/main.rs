use clap::Parser;
mod analyzer;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();

    match analyzer::analyze_log_file(&args.file) {
        Ok(_) => {}
        Err(e) => eprintln!("Error: {}", e),
    }
}

