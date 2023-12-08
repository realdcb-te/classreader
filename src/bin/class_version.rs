use clap::Parser;
use eyre::Result;
use classreader::read_classfile;

/// Reads the provided class file name and prints the major.minor version
#[derive(Parser)]
struct Args {
    /// The class file to parse
    filename: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // For our purpose we don't need to actually parse the bytecode
    let mut classblob = read_classfile(&args.filename, classreader::Parse::WithoutBytecode)?;

    let class = classblob.class()?;

    println!("{}.{}", class.major_version, class.minor_version);

    Ok(())
}
