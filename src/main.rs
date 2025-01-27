use clap::Parser;
use opencv::imgcodecs;
use opencv::objdetect;

#[derive(Debug, Parser)]
#[command(version, about, author)]
struct CLIArgs {
    /// Input photo file
    #[arg(short)]
    input: String,

    /// Output photo file
    #[arg(short)]
    output: String,

    /// Cascade data file
    #[arg(short)]
    cascade: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CLIArgs::parse();
    println!("{:?}", args);

    // Read the photo as gray scale
    let _image = imgcodecs::imread(&args.input, imgcodecs::ImreadModes::IMREAD_GRAYSCALE.into())?;

    // Import cascade file
    let cascade = objdetect::CascadeClassifier::new(&args.cascade)?;

    // TODO: How to use detect_multi_scale?
    //cascade.detect_multi_scale( &Mat::default,);

    Ok(())
}
