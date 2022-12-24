use clap::Parser;
#[derive(Parser)]
pub struct Arguments {
    /// Address
    #[arg(short, long)]
    pub address: String,

    /// Red value
    #[arg(short, long)]
    pub r: u8,

    /// Green value
    #[arg(short, long)]
    pub g: u8,

    /// Blue value
    #[arg(short, long)]
    pub b: u8,
}
