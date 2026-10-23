use crate::gaia::{GaiaRow, HfResponse};
// 加载GAIA-2023 Level1数据集
// 通过http请求获取数据集的验证集
pub async fn load_gaia_level1()->anyhow::Result<Vec<GaiaRow>> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://datasets-server.huggingface.co/rows")
        .query(&[
            ("dataset", "gaia-benchmark/GAIA"),
            ("config", "2023_level1"),
            ("split", "validation"),
            ("offset", "0"), 
            ("length", "10"),
        ])
        .bearer_auth(std::env::var("HF_TOKEN")?)
        .send()
        .await?
        .json::<HfResponse>().await?;

    Ok(response.rows.into_iter().map(|r| r.row).collect())
}

    // println!("{:#?}", response);
    // response 转成 json 字符串
    // let json = serde_json::to_string(&response)?;
    // println!("{:#?}", json);
    // panic!("unimplemented");