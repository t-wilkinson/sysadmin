use clap::Parser;
use futures::executor::block_on;
use reqwest;

#[derive(Parser)]
pub struct Cli {
    url: String,
}

impl Cli {
    pub async fn run(&self) -> Result<(), reqwest::Error> {
        let res = reqwest::get(&self.url).await?;
        // let mut body = String::new();
        // res.read_to_string(&mut body)?;

        println!("{:#}", res.text().await?);
        //     println!("Status: {}", res.status());
        //     println!("Headers:\n{:#?}", res.headers());
        //     println!("Body:\n{}", body);

        Ok(())
    }
}

pub fn run() {
    let cli = Cli::parse();
    block_on(cli.run()).unwrap();
}
