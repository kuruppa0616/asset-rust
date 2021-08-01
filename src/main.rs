extern crate dotenv;

use std::env;
use std::error;

use dotenv::dotenv;

mod SBI;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;


#[derive(Debug)]
pub struct Credential{
    id: String,
    password: String,
}

#[async_std::main]
async fn main() -> Result<()> {
    dotenv().ok();
    
    let sbi_credential = Credential{
        id: env::var("SBI_ID").expect("SBI_ID is not found"),
        password:env::var("SBI_PASS").expect("SBI_PASS is not found"),
    };
    
    let sbi_asset = SBI::fetch_sbi_asset(&sbi_credential).await;
    match sbi_asset {
        Ok(_) => println!("{:?}",sbi_asset),
        Err(e) => Err(e)?,
    }

    Ok(())
}