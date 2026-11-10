use std::{cmp::Ordering, collections::BinaryHeap};

use anyhow::Ok;
use crate::constant::*;
use crate::knowledge_base::*;


#[derive(Debug, Clone)]
pub struct SearchHit {
    pub text: String,
    pub similarity: f32,
}

pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
    let norm_a = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot / (norm_a * norm_b)
    }
}

struct ScoredIndex {
    similarity: f32,
    index: usize,
}

impl PartialEq for ScoredIndex {
    fn eq(&self, other: &Self) -> bool {
        self.similarity == other.similarity
    }
}

impl Eq for ScoredIndex {}
impl Ord for ScoredIndex {
    fn cmp(&self, other: &Self) -> Ordering {
        other.similarity.total_cmp(&self.similarity)
    }
}

impl PartialOrd for ScoredIndex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub async fn vector_search(
    query: &str,
    chunks: &[String],
    top_k: usize,
) -> anyhow::Result<Vec<SearchHit>> {
    if chunks.is_empty() || top_k == 0 {
        return Ok(Vec::new());
    }

    // 查询向量
    let query_embedding = embed_text(query, TEXT_EMBEDDING_3_SMALL_MODEL).await?;
    // 计算分块向量
    let chunk_embeddings = embed_texts(chunks, TEXT_EMBEDDING_3_SMALL_MODEL).await?;

    // 优先队列，维护 top_k 个最大相似度的索引
    let mut heap: BinaryHeap<ScoredIndex> = BinaryHeap::with_capacity(top_k + 1);

    // 计算每个分块的相似度并加入堆
    for (index, embedding) in chunk_embeddings.iter().enumerate() {
        let similarity = cosine_similarity(&query_embedding, embedding);
        heap.push(ScoredIndex { similarity, index });

        // 保持堆大小不超过 top_k
        if heap.len() > top_k {
            heap.pop();
        }
    }

    let mut ranked: Vec<ScoredIndex> = heap.into_vec();
    ranked.sort_by(|a, b| b.similarity.total_cmp(&a.similarity));

    Ok(ranked
        .into_iter() 
        .map(|scored| SearchHit {
            text: chunks[scored.index].clone(),
            similarity: scored.similarity,
        })
        .collect())
}
