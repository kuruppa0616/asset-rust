extern crate dotenv;

use std::env;
use std::error;

use dotenv::dotenv;
mod SBI;
use SBI::SbiAssset;

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

    let gas_url = env::var("GAS_URL").expect("GAS_URL is not found");
    
    let sbi_asset = match try_scrape(&sbi_credential, 3).await {
        Ok(value) => value,
        Err(e) => Err(e)?,
    };

    // GASへポスト
    let res = surf::post(gas_url).body(surf::Body::from_json(&sbi_asset)?).await;

    match res {
        Ok(_) => {
            dbg!("ポスト成功");
            Ok(())
        },
        Err(e) => {
            Err(e)?
        },
    }
}

async fn try_scrape(credential: &Credential, try_times: i32) -> Result<SbiAssset> {
    
    for _ in 0..try_times {

        match SBI::fetch_sbi_asset(&credential).await {
            Ok(value) => return Ok(value),
            Err(e) => {
                dbg!(format!("{}:再試行します", &e));
            },
        }
    }
    Err(format!("スクレイプ失敗 試行回数{}",try_times))?
}