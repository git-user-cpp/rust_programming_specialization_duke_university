use clap::Parser;
use customize_fruit_salad_with_a_cli::create_fruit_salad;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "author",
    about = "make a fruit salad",
)]

struct Opts {
    #[clap(short, long)]
    fruits: Option<String>,
    csvfile: Option<String>,
}

fn csv_to_vec(csv: &str) -> Vec<String> {
    csv.split(',')
        .map(|s| s.trim().to_string())
        .collect()
}

fn display_salad(fruits: Vec<String>) {
    println!("Your fruit salad contains:");
    for fruit in fruits {
        println!("{}", fruit);
    }
}

fn main() {
    let opts: Opts = Opts::parse();

    let fruit_list = match opts.csvfile {
        Some(filename) => {
            let fruits = std::fs::read_to_string(filename)
                .expect("Couldn't read the file");
            csv_to_vec(&fruits)
        },
        None => {
            opts.fruits.unwrap_or_default()
                .split(',')
                .map(|s| s.trim().to_string())
                .collect()
        },
    };

    let fruit_salad = create_fruit_salad(fruit_list);
    display_salad(fruit_salad);
}
