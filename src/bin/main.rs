use std::env::args;

use zippyst::Error;
use zippyst::File;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    for link in args().skip(1) {
        println!("{}", File::fetch_and_parse(&link).await?);
    }
    Ok(())
}
