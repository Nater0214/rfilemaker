use std::{fs::{create_dir_all, File}, io::Write, path::Path};

use clap::Parser;

/// The args of the program
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    amount: u64,

    #[arg(short, long, default_value_t = String::from("1K"))]
    size: String,

    #[arg(short, long, default_value_t = String::from("."))]
    directory: String,

    #[arg(short, long, default_value_t = String::from("file"))]
    name: String,

    #[arg(short, long, default_value_t = String::from("txt"))]
    extension: String,

    #[arg(short = 'S', long, default_value_t = String::from('e'))]
    string: String,

    #[arg(long, default_value_t = String::from("256M"))]
    max_memory: String,
}

/// Constant containing the size of a kilobyte
const KILOBYTE_SIZE: u128 = 1024;
/// Constant containing the size of a megabyte
const MEGABYTE_SIZE: u128 = 1024 * KILOBYTE_SIZE;
/// Constant containing the size of a gigabyte
const GIGABYTE_SIZE: u128 = 1024 * MEGABYTE_SIZE;
/// Constant containing the size of a terabyte
const TERABYTE_SIZE: u128 = 1024 * GIGABYTE_SIZE;

/// Parse a size string
fn parse_size(size: &String) -> u128 {
    let upper_size = size.to_uppercase();
    if upper_size.ends_with("K") || upper_size.ends_with("KB") {
        return upper_size.trim_end_matches('K').trim_end_matches('B').parse::<u128>().unwrap() * KILOBYTE_SIZE;
    } else if upper_size.ends_with("M") || upper_size.ends_with("MB") {
        return upper_size.trim_end_matches('M').trim_end_matches('B').parse::<u128>().unwrap() * MEGABYTE_SIZE;
    } else if upper_size.ends_with("G") || upper_size.ends_with("GB") {
        return upper_size.trim_end_matches('G').trim_end_matches('B').parse::<u128>().unwrap() * GIGABYTE_SIZE;
    } else if upper_size.ends_with("T") || upper_size.ends_with("TB") {
        return upper_size.trim_end_matches('T').trim_end_matches('B').parse::<u128>().unwrap() * TERABYTE_SIZE;
    } else {
        return upper_size.parse::<u128>().unwrap();
    }
}

/// The main function
fn main() {

    // Get command line arguments
    let args = Args::parse();

    // Parse size
    let size = parse_size(&args.size);

    // Parse the max memory
    let max_memory = parse_size(&args.max_memory);

    // Panic if string is larger than max memory
    if args.string.len() > max_memory as usize {
        panic!("String size is larger than max memory");
    }

    // Create the write string
    let mut write_string = args.string.repeat((max_memory / args.string.len() as u128) as usize);
    write_string.truncate(max_memory as usize);

    // Determine the amount of times to write the string
    let write_amount = (size / max_memory) as u64;

    // Create the remainder string
    let mut remainder_string = args.string.repeat((size % max_memory) as usize);
    remainder_string.truncate(size as usize);

    // Validate directory
    let dir = Path::new(&args.directory);

    // Create directory if it doesn't exist
    if !dir.exists() {
        create_dir_all(dir).unwrap();
    }

    // Loop creating files
    for i in 1..=args.amount {

        // Get the file path
        let file_path = dir.join(format!("{}{}.{}", args.name, i, args.extension));

        // Create the file
        File::create(&file_path).unwrap();

        // Write the string to the file
        let mut file = File::options().write(true).open(&file_path).unwrap();
        for _ in 0..write_amount {
            file.write_all(write_string.as_bytes()).unwrap();
        }
        file.write_all(remainder_string.as_bytes()).unwrap();
    }
}
