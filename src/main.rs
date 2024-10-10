use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("file"))]
    prefix: String,

    #[arg(short, long, default_value_t = 1)]
    amount: u128
}

fn main() {

    // Get command line arguments
    let args = Args::parse();

    // Print the arguments
    println!("Name prefix: {}", args.prefix);
    println!("Amount: {}", args.amount);
}
