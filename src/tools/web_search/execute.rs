


use anyhow::Context;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


#[derive(Debug,Deserialize,JsonSchema)]
pub struct WebSearchArgs{
    pub query:String,
    
    #[schemars(range(min=0,max=20))]
    #[serde(default="default_max_results")]
    pub max_results:u8,

    #[serde(default="default_topic")]
    pub topic:String,

    pub time_range:Option<String>,
}

fn default_max_results()->u8{
    2
}

fn default_topic()->String{
    "general".to_string()
}



impl WebSearchArgs {
    pub fn name()->&'static str{
        "web_search"
    }
}

#[derive(Debug,Deserialize,Clone,Serialize)]
struct TavilyRequest<'a>{
    api_key:&'a str,
    pub query:&'a str,
    pub max_results:u8,
    pub topic:&'a str,

    #[serde(skip_serializing_if="Option::is_none")]
    pub time_range:Option<&'a str>,

    search_depth:&'a str,
    include_answer:bool,
}

#[derive(Debug,Deserialize,Clone,Serialize)]
pub struct SearchResult{
    pub title:String,
    pub url:String,
    pub content:String,
    // pub score:Option<f64>,
}

#[derive(Debug,Deserialize,Clone,Serialize)]
pub struct TavilyResponse{
    results:Vec<SearchResult>, 
    answer:Option<String>,
}

// 这个返回给大语音模型的
#[derive(Debug,Deserialize,Clone,Serialize)]
pub struct WebSearchOutput{
    #[serde(skip_serializing_if="Option::is_none")]
    answer:Option<String>,
    results:Vec<SearchResult>,
}


pub async fn search_web(args:WebSearchArgs)->anyhow::Result<WebSearchOutput>{
    let api_key = std::env::var("TAVILY_API_KEY").context("TAVILY_API_KEY not found")?;
    let body = TavilyRequest{
        api_key:&api_key,
        query:&args.query,
        max_results:args.max_results,
        topic:&args.topic,
        time_range:args.time_range.as_deref(),
        search_depth:"advanced",
        include_answer:true,
    };

    let resp = reqwest::Client::new()
        .post("https://api.tavily.com/search")
        .json(&body)
        .send()
        .await
        .context("send request to tavily failed")?;

        if !resp.status().is_success(){
            anyhow::bail!("tavily api failed, status: {}",resp.status());
        }

        let parsed:TavilyResponse = resp.json().await.context("parse response body failed")?;
        Ok(WebSearchOutput{
            answer:parsed.answer,
            results:parsed.results,
        })
}