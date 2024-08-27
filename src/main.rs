use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
};

use clap::Parser;
use rand::{thread_rng, Rng};
use rand_distr::{Distribution, WeightedAliasIndex};

#[derive(Parser)]
struct Args {
    /// the target of corruption
    file: String,
}

#[derive(Debug, Clone, Copy)]
enum Change {
    Delete,
    Add,
    Alter,
    Nothing,
}

fn file_reader<P: AsRef<Path>>(path: P) -> Result<BufReader<File>, std::io::Error> {
    let file = File::open(path)?;
    let f = BufReader::new(file);
    Ok(f)
}

fn file_writer<P: AsRef<Path>>(path: P) -> Result<BufWriter<File>, std::io::Error> {
    let file = File::create(path)?;
    let f = BufWriter::new(file);
    Ok(f)
}

fn traverse(reader: BufReader<File>, writer: &mut BufWriter<File>) -> Result<(), std::io::Error> {
    let mut rng = thread_rng();
    let dist = WeightedAliasIndex::new(vec![1, 1, 1, 10]).unwrap();
    let choices = vec![Change::Delete, Change::Add, Change::Alter, Change::Nothing];
    for byte in reader.bytes() {
        let byte = byte?;
        let operation = choices[dist.sample(&mut rng)];
        match operation {
            Change::Delete => {}
            Change::Add => {
                let choice = rng.gen();
                writer.write(&[byte, choice])?;
            }
            Change::Alter => {
                let choice = rng.gen();
                writer.write(&[choice])?;
            }
            Change::Nothing => {
                writer.write(&[byte])?;
            }
        }
    }
    writer.flush()?;
    Ok(())
}

fn main() {
    let args = Args::parse();
    let path = args.file;
    let read_path = PathBuf::from(&path);
    let write_path = PathBuf::from(format!("{}.bad", path));

    let reader = file_reader(read_path).unwrap();
    let mut writer = file_writer(write_path).unwrap();

    traverse(reader, &mut writer).unwrap();
}
