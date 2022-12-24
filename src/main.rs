mod arguments;
mod error;
use arguments::Arguments;
use clap::Parser;
use error::RunError;
use pixoo64::{color::Color, pixoo64::Pixoo64};

#[tokio::main]
async fn main() {
    match run().await {
        Ok(_) => (),
        Err(error) => eprintln!("{error}"),
    }
}

async fn run() -> Result<(), RunError> {
    let arguments = Arguments::parse();
    let mut pixoo = Pixoo64::new(&arguments.address);
    for x in 0..64 {
        for y in 0..64 {
            pixoo.set(
                x,
                y,
                &Color {
                    r: arguments.r,
                    g: arguments.g,
                    b: arguments.b,
                },
            )?;
        }
    }

    pixoo.send().await?;
    Ok(())
}
