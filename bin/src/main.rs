use std::env::args;

use zippyst::File;

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = args().skip(1).collect::<Vec<_>>();
    if args.is_empty() {
        return Err("at least one link is required".to_string());
    }
    
    for file in args
        .into_iter()
        .map(|url| tokio::spawn(async move { File::fetch_and_parse(&url).await }))
        .collect::<Vec<_>>()
    {
        println!("{}", file.await.unwrap().map_err(|err| err.to_string())?);
    }

    Ok(())
}
