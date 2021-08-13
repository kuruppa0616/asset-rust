use std::str::FromStr;
use chromiumoxide::browser::{Browser, BrowserConfig};
use futures::StreamExt;
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct SbiAssset {
    date: String,
    total: i32,
    profit: i32,
    profit_percent: f32,
}

impl SbiAssset {
    #[allow(dead_code)]
    fn default() -> Self {
        Self {
            date: String::new(),
            total: 0,
            profit: 0,
            profit_percent: 0.0,
        }
    }

    #[allow(dead_code)]
    fn new(date: String, total: i32, profit: i32, profit_percent: f32) -> Self {
        Self {
            date,
            total,
            profit,
            profit_percent,
        }
    }
}

const BASE_URL: &str = "https://site1.sbisec.co.jp/ETGate/";

pub async fn fetch_sbi_asset(credential: &Credential) -> Result<SbiAssset> {
    
    let (browser, mut handler) = Browser::launch(BrowserConfig::builder().build()?)
        .await
        .expect("Cant Launch Browser");

    async_std::task::spawn(async move {
        loop {
            let _ = handler.next().await.unwrap();
        }
    });

    // create a new browser page and navigate to the url
    let page = browser.new_page("google.com").await?;
    page.goto(BASE_URL).await?;

    // ログインページ
    login_sbi(&page, &credential).await?;
    page.wait_for_navigation().await?.content().await?;

    // ログイン成功判定
    let logout_button = page.find_element("#logoutM > a")
        .await;

    if let Err(_) = logout_button {
        Err("ログイン失敗")?
    } 

    // 口座管理ページ
    page.find_element("#link02M > ul > li:nth-child(3) > a")
        .await
        .expect("not found ポートフォリオ")
        .click()
        .await?;
    let _portfolio = page.wait_for_navigation().await?.content().await?;
    // dbg!(&portfolio);

    // トータルリターンページ
    page.find_element("#navi02P > ul > li:nth-child(5) > div > a")
        .await
        .expect("not found トータルリターン")
        .click()
        .await?;
    let _totalreturn = page.wait_for_navigation().await?.content().await?;
    // dbg!(&totalreturn);


    // トータルリターン取得
    let profit = page
        .find_element("#printArea1 > div > table > tbody > tr:nth-child(7) > td.vaM.alR.fUp")
        .await?
        .inner_text()
        .await?;

    // トータルリターンは「含み損益 \n 含み損益(%)」というフォーマットになっているので改行でスプリット
    let mut aa = match &profit {
        Some(value) => {
            let splited = value.split("\n");
            splited
        }
        None => Err("値の取得に失敗")?,
    };

    // スプリットした文字列を一つづつ数値へパース
    let profit = extract_number::<i32>(aa.next())?;
    let profit_percent = extract_number::<f32>(aa.next())?;

    //総評価金額取得
    let total_str = page
        .find_element("#printArea1 > div > table > tbody > tr:nth-child(7) > td:nth-child(2)")
        .await?
        .inner_text()
        .await?;
    let total = extract_number::<i32>(total_str.as_deref())?;

    //算出日時
    let date_str = page
        .find_element("#summaryForm > div > p.fm01.floatL")
        .await?
        .inner_text()
        .await?;
    let date = match date_str {
        Some(value) => {
            let ret = value.replace("算出日：", "");
            ret
        },
        None => Err("date is invalid")?,
    };

    Ok(SbiAssset::new(date, total, profit, profit_percent))
}

fn extract_number<F: FromStr>(arg: Option<&str>) -> Result<F> {
    match arg {
        Some(value) => {
            let replaced = value.replace(",", "").replace("%", "");
            let parsed = replaced.parse::<F>();
            match parsed {
                Ok(ret) => Ok(ret),
                Err(_) => Err(format!("parse failed:{}", replaced))?,
            }
        }
        None => Err("value is none")?,
    }
}

async fn login_sbi(page: &chromiumoxide::Page, credential: &Credential) -> Result<()> {
    // ID入力
    page.find_element("#user_input > input[type=text]")
        .await?
        .click()
        .await?
        .type_str(&credential.id)
        .await?;

    // パスワード入力 + エンターでログイン
    page.find_element("#password_input > input[type=password]")
        .await?
        .click()
        .await?
        .type_str(&credential.password)
        .await?
        .press_key("Enter")
        .await?;

    page.wait_for_navigation().await?.content().await?;

    Ok(())
}
