use crate::{
    agent::{
        ContentItem, ExecutionContext,
        llm_request::{BeforeLlmCallback, LlmRequest},
    },
    callback::context_optimizer::{count_tokens, find_safe_start},
};

pub struct SlidingWindow {
    pub model: String,
    pub token_threshold: usize,// 上下文阈值阈值
    pub window_size: usize, // 滑动窗口大小
}

#[async_trait::async_trait]
impl BeforeLlmCallback for SlidingWindow {
    async fn call(&self, _context: &mut ExecutionContext, request: &mut LlmRequest) {
        // 如果上下文不足阈值，直接返回
        let before = count_tokens(&self.model, request);
        if before < self.token_threshold {
            return;
        }

        // 如果内容不足窗口大小，直接返回
        if request.contents.len() <= self.window_size {
            return;
        }

        // 找到最近一次用户消息的索引
        let user_idx = request
            .contents
            .iter()
            .position(|item| matches!(item, ContentItem::Message { role , .. } if role == "user"));
        let Some(user_idx) = user_idx else { return };

        tracing::info!(
            "SlidingWindow: {before} tokens >= threshold {}, trimming to last {} items",
            self.token_threshold,
            self.window_size
        );

        let start = find_safe_start(&request.contents, request.contents.len() - self.window_size);
        let start = start.max(user_idx + 1);

        request.contents.drain(user_idx + 1..start);
        
        let after = count_tokens(&self.model, request);
        tracing::info!(
            "SlidingWindow: after trim, {after} tokens ({} items kept)",
            request.contents.len()
        );
    }
}
 