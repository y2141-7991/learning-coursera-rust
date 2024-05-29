use caesar_cipher_cli::{decrypt, encrypt};
use clap::Parser;



#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    encrypt: bool,

    #[arg(short, long)]
    decrypt: bool,

    #[arg(short, long)]
    message: String,

    #[arg(short, long, default_value="3")]
    shift: u8,
}

fn main() {
    let args = Args::parse();
    if args.decrypt {
        println!("{}", decrypt(&args.message, args.shift));
    }
    else if args.encrypt {
        println!("{}", encrypt(&args.message, args.shift));
    }
    else {
        println!("Please specify either --encrypt or --decrypt");
    }
}
