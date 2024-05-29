use clap::Parser;
use cli_rust::create_fruit_salad;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Your Name <your.email@example.com>",
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
        "Create fruit {}, {:?}",
        num_fruits,
        create_fruit_salad(num_fruits)
    )
}
