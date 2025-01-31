use serde::{Deserialize, Serialize};

use crate::request::KimiModel;

/*

{
    "id": "cmpl-04ea926191a14749b7f2c7a48a68abc6",
    "object": "chat.completion",
    "created": 1698999496,
    "model": "moonshot-v1-8k",
    "choices": [
        {
            "index": 0,
            "message": {
                "role": "assistant",
                "content": " 你好，李雷！1+1等于2。如果你有其他问题，请随时提问！"
            },
            "finish_reason": "stop"
        }
    ],
    "usage": {
        "prompt_tokens": 19,
        "completion_tokens": 21,
        "total_tokens": 40
    }
}和{
    "error": {
        "type": "content_filter",
        "message": "The request was rejected because it was considered high risk"
    }
}这俩是kimi的api的chat的返回消息，现在为他写一个rust类型

*/

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Response {}

#[derive(Serialize, Deserialize)]
pub struct ResponseData {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: KimiModel,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseError {}
