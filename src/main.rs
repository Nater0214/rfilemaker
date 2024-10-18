use std::path::Path;

use clap::Parser;
use job::Job;
use tokio::{fs::{create_dir, try_exists}, task::{self, JoinHandle}};

mod job;

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

    #[arg(short, long, default_value_t = 1)]
    jobs: u16,

    #[arg(long, default_value_t = String::from("256M"))]
    write_size: String,
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
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Get command line arguments
    let args = Args::parse();

    // Parse size
    let size = parse_size(&args.size);

    // Parse the max memory
    let write_size = parse_size(&args.write_size);

    // Panic if string is larger than max memory
    if args.string.len() > write_size as usize {
        panic!("String size is larger than max memory");
    }

    // Validate directory
    let dir = Path::new(&args.directory);

    // Create directory if it doesn't exist
    if !try_exists(dir).await? {
        create_dir(dir).await?;
    }

    // Create handles vector
    let mut handles: Vec<JoinHandle<()>> = vec![];

    // Loop making files
    for i in 1..=args.amount {

        // Get the file path
        let file_path = dir.join(format!("{}{}.{}", args.name, i, args.extension));

        // Create a job
        let job = Job::new(
            file_path.clone(),
            args.string.clone(),
            size,
            write_size
        );

        // Run the job
        let handle = task::spawn(async move {
            let result = job.run().await;
            match result {
                Ok(_) => println!("Created file {}", file_path.display()),
                Err(e) => println!("Error creating file {}: {}", file_path.display(), e),
            }
        });

        // Add the handle to the handles vector
        handles.push(handle);
    }

    // Get a result from each handle
    for handle in handles {
        handle.await?;
    }

    Ok(())
}
