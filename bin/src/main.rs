use std::env::args;

use zippyst::File;

#[tokio::main]
async fn main() -> Result<(), String> {
    for file in args()
        .skip(1)
        .map(|url| tokio::spawn(async move { File::fetch_and_parse(&url).await }))
        .collect::<Vec<_>>()
    {
        println!("{}", file.await.unwrap().map_err(|err| err.to_string())?);
    }

    Ok(())
}
