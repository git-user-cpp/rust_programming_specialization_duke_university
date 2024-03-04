use clap::Parser;
use decoder_ring::print_stats_analysis;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    message: String,

    #[arg(short, long)]
    stats: bool,

    #[arg(short, long)]
    guess: bool,
}

fn main() {
    let args = Args::parse();

    if args.stats {
        print_stats_analysis(&args.message);
    }

    if args.guess {
        let (depth, best_shift, decrypted, max_score) = decoder_ring::guess_shift("something", 2);
        println!(
            "Best shift: {} (out of {}), score {}",
            best_shift, depth, max_score
        );
        println!("Decrypted message: {}", decrypted);
    }
}