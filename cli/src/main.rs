use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "Brian Silah", version = "0.1", about = "A simple CLI that echos the word given", long_about = None)]
struct Args {
    #[arg(short,long)]
    word: String,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args.word);
}
