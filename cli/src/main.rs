use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "Brian Silah", version = "0.1", about = "A simple CLI that echos the word given", long_about = None)]
struct Args {
    #[arg(short,long)]
    word: String,

    #[arg(short,long, default_value = "1")]
    times: u32,
}

fn main() {
    let args = Args::parse();
    for _ in 0..args.times {
        println!("{}", args.word);
    }
}
