extern crate dotenv;

use std::env;
use std::error;

use dotenv::dotenv;
use sbi::SbiAssset;
mod sbi;

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
    
    let sbi_asset = match try_scrape(&sbi_credential, 5).await {
        Ok(value) => value,
        Err(e) => Err(e)?,
    };

    dbg!(&sbi_asset);
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
        if let Ok(value) = sbi::fetch_sbi_asset(&credential).await {
            return Ok(value)
        }
    }
    Err(format!("スクレイプ失敗 試行回数{}",try_times))?
}