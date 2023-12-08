use clap::Parser;
use eyre::Result;
use classreader::read_classfile;

/// Reads the provided class filename and dumps all parsed information
#[derive(Parser)]
struct Args {
    /// The class file to parse
    filename: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut classblob = read_classfile(&args.filename, classreader::Parse::WithBytecode)?;

    println!("{:?}", classblob.class());

    Ok(())
}
