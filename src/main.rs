use clap::Parser;
use opencv::core::Rect;
use opencv::core::Size_;
use opencv::core::Vector;
use opencv::core::VecN;
use opencv::imgcodecs;
use opencv::imgproc;
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
    let mut image = imgcodecs::imread(&args.input, imgcodecs::ImreadModes::IMREAD_GRAYSCALE.into())?;

    // Import cascade file
    let mut cascade: CascadeClassifier = objdetect::CascadeClassifier::new(&args.cascade)?;

    let mut rect = Vector::<Rect>::new();

    // detect_multi_scale
    cascade.detect_multi_scale(
        &image,
        &mut rect,
        1.0001,
        20,
        0,
        Size_::new(20, 20),
        Size_::new(20, 20),
    )?;

    println!("{:?}", rect);

    for r in rect.iter() {
        println!("{:?}", r);

        // rectangle
        imgproc::rectangle(
            &mut image,
            r,
            VecN::new(0.0, 0.0, 0.0, 0.0),
            0,
            0,
            0,
        )?;
    }

    // SAVING!!
    imgcodecs::imwrite(
        &args.output,
        &image,
        &Vector::<i32>::new(),
    )?;

    Ok(())
}
