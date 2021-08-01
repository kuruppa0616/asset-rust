use std::str::FromStr;
use std::{num::ParseFloatError, time::Duration};
use std::error::Error;

use async_std::task::sleep;
use chromiumoxide::{
    browser::{Browser, BrowserConfig},
    cdp::browser_protocol::page::{CaptureScreenshotFormat, CaptureScreenshotParams},
};
use futures::StreamExt;

use super::*;

#[derive(Debug)]
pub struct SbiAssset {
    total: i32,
    profit: i32,
    profit_percent: f32,
}

impl SbiAssset {
    fn default() -> Self {
        Self {
            total: 0,
            profit: 0,
            profit_percent: 0.0,
        }
    }

    fn new(total: i32, profit: i32, profit_percent: f32) -> Self {
        Self {
            total,
            profit,
            profit_percent,
        }
    }
}

const BASE_URL: &str = "https://site1.sbisec.co.jp/ETGate/";

pub async fn fetch_sbi_asset(
    credential: &Credential,
) -> Result<SbiAssset> {
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

    // ログイン
    login_sbi(&page, &credential).await?;
    page.wait_for_navigation().await?;
    sleep(Duration::from_secs(1)).await;

    // 口座管理
    page.find_element("#link02M > ul > li:nth-child(3) > a")
        .await
        .expect("not found ポートフォリオ")
        .click()
        .await?;
    page.wait_for_navigation().await?;
    sleep(Duration::from_secs(1)).await;

    // トータルリターン
    page.find_element("#navi02P > ul > li:nth-child(5) > div > a")
        .await
        .expect("not found トータルリターン")
        .click()
        .await?;
    page.wait_for_navigation().await?;
    sleep(Duration::from_secs(1)).await;

    let profit= page.find_element("#printArea1 > div > table > tbody > tr:nth-child(7) > td.vaM.alR.fUp")
    .await?
    .inner_text()
    .await?;

    let mut aa = match &profit {
        Some(value) => {
            let splited = value.split("\n");
            splited
        },
        None => Err("値の取得に失敗")?,
    };

    let profit = extract_number::<i32>(aa.next())?;
    let profit_percent = extract_number::<f32>(aa.next())?;

    dbg!(&profit);    
    dbg!(&profit_percent);    


    page.save_screenshot(
        CaptureScreenshotParams::builder()
            .format(CaptureScreenshotFormat::Png)
            .build(),
        "hn-page.png",
    )
    .await?;

    Ok(SbiAssset{
        total: 0,
        profit: profit,
        profit_percent: profit_percent,
    })
}

fn extract_number<F: FromStr>(arg: Option<&str>) -> Result<F> {
    match arg{
        Some(value) =>{
            let replaced = value.replace(",", "").replace("%", "");
            let parsed = replaced.parse::<F>();
            match parsed {
                Ok(ret) => Ok(ret),
                Err(_) => Err(format!("parse failed:{}", replaced))?,
            }
        },
        None => Err("value is none")?,
    }
}

async fn login_sbi(
    page: &chromiumoxide::Page,
    credential: &Credential,
) ->    Result<()> {
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

    page.wait_for_navigation().await?;
    sleep(Duration::from_secs(3)).await;

    Ok(())
}
