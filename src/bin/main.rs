use std::env::args;
use smol::Task;
use zippyst::file::File;

fn main() {
    smol::run(async {
        futures::future::join_all(args().skip(1).map(|link| {
            Task::blocking(async move {
                File::new(&link)
                    .get_information_retry(5)
                    .unwrap()
                    .full_link()
            })
        }))
        .await
        .iter()
        .for_each(|res| {
            println!("{}", res);
        });
    });
}
