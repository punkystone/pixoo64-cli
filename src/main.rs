mod arguments;
mod error;
use arguments::Arguments;
use clap::Parser;
use error::{ArgumentError, RunError};
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
    if arguments.clear {
        pixoo.send().await?;
        return Ok(());
    }
    if let (Some(r), Some(g), Some(b)) = (arguments.r, arguments.g, arguments.b) {
        if arguments.full {
            for x in 0..64 {
                for y in 0..64 {
                    pixoo.set(x, y, &Color { r, g, b })?;
                }
            }
        } else if let (Some(x), Some(y)) = (arguments.x, arguments.y) {
            pixoo.set(x, y, &Color { r, g, b })?;
        } else {
            return Err(RunError::ArgumentError(ArgumentError));
        }

        pixoo.send().await?;
    } else {
        return Err(RunError::ArgumentError(ArgumentError));
    }

    Ok(())
}
