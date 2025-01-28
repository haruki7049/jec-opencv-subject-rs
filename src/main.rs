use clap::Parser;
use opencv::core::Mat;
use opencv::core::Rect;
use opencv::core::Size_;
use opencv::core::Vector;
use opencv::imgcodecs;
use opencv::objdetect;
use opencv::objdetect::CascadeClassifier;
use opencv::prelude::CascadeClassifierTrait;

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

    // Read the photo as gray scale
    let image = imgcodecs::imread(&args.input, imgcodecs::ImreadModes::IMREAD_GRAYSCALE.into())?;

    // Import cascade file
    let mut cascade: CascadeClassifier = objdetect::CascadeClassifier::new(&args.cascade)?;

    // detect_multi_scale
    cascade.detect_multi_scale(
        &Mat::default(),
        &mut Vector::<Rect>::new(),
        1.0001,
        20,
        0,
        Size_::new(20, 20),
        Size_::new(0, 0),
    )?;

    println!("{:?}", cascade);

    Ok(())
}
