mod io;
mod cv;

use clap::Parser;
use opencv::core::MatTraitConst;
use crate::cv::GetPosition;
use crate::io::Print;

#[derive(Parser)]
struct Args {
    /// File path to the image to search on (a.k.a. a haystack)
    host_image: std::path::PathBuf,
    /// File path to the target image (a.k.a. a needle)
    target_image: std::path::PathBuf,
    /// Output path, or omit for coordinates
    output: Option<std::path::PathBuf>,
    /// How close to the needle for it to be counted as match, the higher, the more confident
    #[arg(long, short = 't', default_value_t = 0.8)]
    threshold: f32,
    /// Only match the best piece
    #[arg(long, short = 'o', default_value_t = false)]
    once: bool,
    /// More logcat
    #[arg(long, short = 'v', default_value_t = false)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    let mut templated = cv::Templated::from_files(args.host_image, args.target_image).unwrap();

    if args.verbose {
        println!("threshold = {}, host = {} * {}, target = {} * {}", args.threshold,
                 templated.haystack.rows(), templated.haystack.cols(), templated.needle.rows(), templated.needle.cols())
    }

    if args.once {
        if let Some((p, v)) = templated.get_best_position(args.threshold).unwrap() {
            if args.verbose {
                (p, v).println()
            }
            if let Some(output) = args.output {
                templated.mark(p).unwrap();
                templated.write(&output).unwrap();
            } else if !args.verbose {
                p.println()
            }
        }
    } else {
        let filtered = templated.get_positions(args.threshold).unwrap();
        if args.verbose {
            for p in &filtered {
                p.println()
            }
        }
        if let Some(output) = args.output {
            for (p, _) in filtered {
                templated.mark(p).unwrap()
            }
            templated.write(&output).unwrap();
        } else if !args.verbose {
            for (p, _) in filtered {
                p.println()
            }
        }
    }
}