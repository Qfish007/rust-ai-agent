use anyhow::Ok;
use std::env;
use async_openai::{
    config::OpenAIConfig,
    types::embeddings::{CreateEmbeddingRequestArgs, EmbeddingInput},
    Client,
};

/// text-embedding-v4 单次请求最多接受 10 条文本
const MAX_BATCH_SIZE: usize = 10;

/// embedding 服务和对话模型不是同一家，所以单独构造 client：
/// base_url / api_key 都从独立的环境变量读取，不污染 OPENAI_* 配置
fn embedding_client() -> anyhow::Result<Client<OpenAIConfig>> {
    let api_key = env::var("EMBEDDING_API_KEY").map_err(|_| anyhow::anyhow!("未配置 EMBEDDING_API_KEY"))?;
    let api_base = env::var("EMBEDDING_BASE_URL").map_err(|_| anyhow::anyhow!("未配置 EMBEDDING_BASE_URL"))?;

    Ok(Client::with_config(OpenAIConfig::new().with_api_base(api_base).with_api_key(api_key)))
}

// 将文本转换为向量列表
pub async fn embed_texts(texts: &[String], model: &str) -> anyhow::Result<Vec<Vec<f32>>> {
    if texts.is_empty() {
        return Ok(Vec::new());
    }

    let client = embedding_client()?;
    let mut vectors = Vec::with_capacity(texts.len());

    // 超过单次批量上限时拆成多批请求，返回顺序与输入一致
    for batch in texts.chunks(MAX_BATCH_SIZE) {
        // 将文本转换为向量的请求
        let request = CreateEmbeddingRequestArgs::default()
            .model(model)
            .input(EmbeddingInput::StringArray(batch.to_vec()))
            .build()?;
        // 发送请求
        let response = client.embeddings().create(request).await?;

        let mut data = response.data;
        data.sort_by_key(|embedding| embedding.index);
        vectors.extend(data.into_iter().map(|embedding| embedding.embedding));
    }

    Ok(vectors)
}

// 将文本转换为向量
pub async fn embed_text(text: &str, model: &str) -> anyhow::Result<Vec<f32>> {
    let owned = [text.to_string()];
    let mut vectors = embed_texts(&owned, model).await?;
    vectors
        .pop()
        .ok_or_else(|| anyhow::anyhow!("embedding API returned no vectors"))
}
