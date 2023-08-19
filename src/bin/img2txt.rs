use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filename: String,
}

fn main() {
    let args = Args::parse();

    let mut textual_img = rs_img2txt::TextualImage::new(&args.filename, None).unwrap();
    textual_img.resize(120);

    print!("{}", textual_img.display());
}