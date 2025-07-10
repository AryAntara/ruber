use std::{fs, os};

use serde_json::Value;

use crate::browser::client;

pub async fn auth() {
    let browser = client::new().await;
    if let Err(e) = browser
        .goto("http://127.0.0.1:8000/master-data/user-role")
        .await
    {
        print!("{e}");
        return;
    }

    let script =
        fs::read_to_string("/home/ary/Dev/Projects/Autest/src/case/script/login.js").unwrap();
    let page = match browser.execute(&script, vec![]).await {
        Ok(data) => data,
        Err(e) => {
            print!("{e}");
            Value::Null
        }
    };

    print!("{page}")
}
