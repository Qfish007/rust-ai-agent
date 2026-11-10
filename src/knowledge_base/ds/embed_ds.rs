use std::collections::{HashMap, HashSet};

/// 本地哈希向量的维度。不需要任何外部 embedding 服务，
/// 把每个 token 哈希到其中一维上即可。
const EMBEDDING_DIM: usize = 512;

/// 把文本切成 token：
/// - 英文/数字按连续字母数字串（小写化）切
/// - 中文按单字 + 相邻两字（bigram）切，bigram 能保留一点上下文
fn tokenize(text: &str) -> Vec<String> {
    fn flush_ascii(buf: &mut String, tokens: &mut Vec<String>) {
        if !buf.is_empty() {
            tokens.push(std::mem::take(buf).to_lowercase());
        }
    }

    fn is_cjk(ch: char) -> bool {
        matches!(
            ch as u32,
            0x3400..=0x4DBF | 0x4E00..=0x9FFF | 0x3040..=0x30FF | 0xAC00..=0xD7AF
        )
    }

    let mut tokens = Vec::new();
    let mut ascii = String::new();
    let mut prev_cjk: Option<char> = None;

    for ch in text.chars() {
        if ch.is_ascii_alphanumeric() {
            ascii.push(ch);
            prev_cjk = None;
        } else if is_cjk(ch) {
            flush_ascii(&mut ascii, &mut tokens);

            tokens.push(ch.to_string());
            if let Some(prev) = prev_cjk {
                let mut bigram = String::with_capacity(8);
                bigram.push(prev);
                bigram.push(ch);
                tokens.push(bigram);
            }
            prev_cjk = Some(ch);
        } else {
            flush_ascii(&mut ascii, &mut tokens);
            prev_cjk = None;
        }
    }
    flush_ascii(&mut ascii, &mut tokens);

    tokens
}

/// FNV-1a 哈希，返回 (维度下标, 正负号)。
/// 正负号用来让不同 token 的哈希碰撞互相抵消，减少偏差。
fn hash_token(token: &str) -> (usize, f32) {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in token.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x100_0000_01b3);
    }

    let index = (hash as usize) % EMBEDDING_DIM;
    let sign = if (hash >> 63) & 1 == 0 { 1.0 } else { -1.0 };
    (index, sign)
}

/// 用「字符 n-gram + 哈希 + TF-IDF」在本地把一批文本转成向量，无需调用 embedding API。
///
/// 同一批文本共享一份 IDF，因此 query 和候选块要在**同一次调用**里一起传入，
/// 这样算出来的相似度才有可比性。
pub fn embed_texts(texts: &[String]) -> Vec<Vec<f32>> {
    if texts.is_empty() {
        return Vec::new();
    }

    let docs: Vec<Vec<String>> = texts.iter().map(|text| tokenize(text)).collect();

    // 统计文档频率（每个 token 出现在多少篇文本里）
    let mut doc_freq: HashMap<&str, usize> = HashMap::new();
    for doc in &docs {
        let mut seen = HashSet::new();
        for token in doc {
            if seen.insert(token.as_str()) {
                *doc_freq.entry(token.as_str()).or_insert(0) += 1;
            }
        }
    }

    let total_docs = docs.len() as f32;
    docs.iter()
        .map(|doc| {
            let mut term_freq: HashMap<&str, usize> = HashMap::new();
            for token in doc {
                *term_freq.entry(token.as_str()).or_insert(0) += 1;
            }

            let mut vector = vec![0.0f32; EMBEDDING_DIM];
            for (token, count) in term_freq {
                let df = *doc_freq.get(token).unwrap_or(&1) as f32;
                let idf = (1.0 + total_docs / df).ln();
                // 次线性 TF，避免长文本里重复词权重爆炸
                let weight = (1.0 + (count as f32).ln()) * idf;

                let (index, sign) = hash_token(token);
                vector[index] += sign * weight;
            }

            // L2 归一化，余弦相似度就等价于点积
            let norm = vector.iter().map(|x| x * x).sum::<f32>().sqrt();
            if norm > 0.0 {
                for value in &mut vector {
                    *value /= norm;
                }
            }
            vector
        })
        .collect()
}
