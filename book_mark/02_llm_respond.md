# LLM 响应示例1
```rust 
CreateChatCompletionResponse {
    id: "7461e1ce-738f-45d4-9fd0-631eceb04e16",
    choices: [
        ChatChoice {
            index: 0,
            message: ChatCompletionResponseMessage {
                content: Some(
                    "中国的首都是**北京**。",
                ),
                refusal: None,
                tool_calls: None,
                annotations: None,
                role: Assistant,
                function_call: None,
                audio: None,
            },
            finish_reason: Some(
                Stop,
            ),
            logprobs: None,
        },
    ],
    created: 1793623617,
    model: "deepseek-flash",
    service_tier: None,
    system_fingerprint: Some(
        "bc19ba97cbe016479903222bb1578db6",
    ),
    object: "chat.completion",
    usage: Some(
        CompletionUsage {
            prompt_tokens: 39,
            completion_tokens: 23,
            total_tokens: 62,
            prompt_tokens_details: Some(
                PromptTokensDetails {
                    audio_tokens: None,
                    cached_tokens: Some(
                        0,
                    ),
                    text_tokens: None,
                    image_tokens: None,
                    cache_write_tokens: None,
                },
            ),
            completion_tokens_details: Some(
                CompletionTokensDetails {
                    accepted_prediction_tokens: None,
                    audio_tokens: None,
                    reasoning_tokens: Some(
                        15,
                    ),
                    rejected_prediction_tokens: None,
                    text_tokens: None,
                },
            ),
        },
    ),
    metadata: None,
    moderation: None,
}
```

