use clap::Parser;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ARGS: Args = Args::parse();
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The port the api will be at
    #[arg(long, default_value_t = 23599)]
    pub port: u16,

    /// The database that ko will use (has to be posgres)
    #[arg(long, default_value_t = String::from("postgresql://127.0.0.1:5432/knockout"))]
    pub db: String,

    /// The redis server that ko will use
    #[arg(long, default_value_t = String::from("127.0.0.1:6379"))]
    pub redis: String,
}
