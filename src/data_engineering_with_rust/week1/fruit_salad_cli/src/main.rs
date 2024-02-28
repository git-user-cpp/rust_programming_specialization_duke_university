use clap::Parser;
use fruit_salad_cli::create_fruit_salad;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Andrew Kushyk",
    about = "Number of fruits to include in the salad"
)]

struct Opts {
    #[clap(short, long)]
    number: usize,
}

fn main() {
    let opts = Opts::parse();

    let num_fruits = opts.number;

    create_fruit_salad(num_fruits);

    println!(
        "Created fruit salad with {} fruits {:?}",
        num_fruits,
        create_fruit_salad(num_fruits)
    );
}