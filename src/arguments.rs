use clap::Parser;
#[derive(Parser, Debug)]
pub struct Arguments {
    /// Address
    #[arg(short, long)]
    pub address: String,

    /// Clear Screen
    #[arg(short, long)]
    pub clear: bool,

    #[arg(short, long)]
    pub full: bool,

    /// X Position
    #[arg(short, long, required_if_eq_all([
        ("clear", "false"),
        ("full", "false")
    ]))]
    pub x: Option<usize>,

    /// Y Position
    #[arg(short, long, required_if_eq_all([
        ("clear", "false"),
        ("full", "false")
    ]))]
    pub y: Option<usize>,

    /// Red value
    #[arg(short, long, required_if_eq("clear", "false"))]
    pub r: Option<u8>,

    /// Green value
    #[arg(short, long, required_if_eq("clear", "false"))]
    pub g: Option<u8>,

    /// Blue value
    #[arg(short, long)]
    pub b: Option<u8>,
}
