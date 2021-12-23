use std::env::args;

use zippyst::Error;
use zippyst::File;

#[tokio::main]
async fn main() -> Result<(), Error> {
    for file in args()
        .skip(1)
        .map(|url| tokio::spawn(async move { File::fetch_and_parse(&url).await }))
        .collect::<Vec<_>>()
    {
        println!("{}", file.await.unwrap()?);
    }

    Ok(())
}
