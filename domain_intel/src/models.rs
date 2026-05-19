use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Domain {
    #[arg(short, long, default_value_t = String::from("google.com"))]
    pub url: String,
}
