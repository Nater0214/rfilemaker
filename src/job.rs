use std::path::PathBuf;

use tokio::{fs::File, io::{self, AsyncWriteExt}};

/// A job that describes how to make a file
pub struct Job {
    pub path: PathBuf,
    pub string: String,
    pub size: u128,
    pub write_size: u128
}

impl Job {

    /// Construct a new Job
    #[inline]
    pub fn new(path: PathBuf, string: String, size: u128, write_size: u128) -> Job {
        Job {
            size,
            write_size,
            path,
            string
        }
    }

    /// Run the job
    pub async fn run(&self) -> io::Result<()> {

        // Create the file and open it
        File::create(&self.path).await?;
        let mut file = File::options().write(true).open(&self.path).await?;

        // Create the write string
        let mut write_string = self.string.clone();
        write_string = write_string.repeat((self.write_size / self.string.len() as u128) as usize);
        write_string.truncate(self.write_size as usize);
        let write_string = write_string;

        // Loop writing to file
        for _ in 0..(self.size / self.write_size) {
            file.write_all(write_string.as_bytes()).await?;
        }

        // Write the remaining amount
        let remaining = self.size % self.write_size;
        file.write_all(&write_string[..remaining as usize].as_bytes()).await?;

        // Return
        Ok(())
    }
}

impl Clone for Job {
    #[inline]
    fn clone(&self) -> Self {
        Job {
            size: self.size,
            write_size: self.write_size,
            path: self.path.clone(),
            string: self.string.clone()
        }
    }
}
