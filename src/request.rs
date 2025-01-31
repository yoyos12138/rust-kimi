use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

///请求体
#[skip_serializing_none]
#[derive(Serialize, Clone)]
pub struct RequestBody<T = ()>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    /// 模型类型
    pub model: KimiModel,

    /// 包含迄今为止对话的消息列表
    pub messages: Vec<SingleMessage>,

    /// 聊天完成时生成的最大 token 数。
    /// 如果到生成了最大 token 数个结果仍然没有结束，
    /// finish reason 会是 "length", 否则会是 "stop"
    /// 这个值建议按需给个合理的值，如果不给的话，
    /// 我们会给一个不错的整数比如 1024。特别要注意的是，
    /// 这个 max_tokens 是指您期待我们返回的 token 长度，
    /// 而不是输入 + 输出的总长度。比如对一个 moonshot-v1-8k 模型，
    /// 它的最大输入 + 输出总长度是 8192，当输入 messages 总长度为 4096 的时候，
    /// 您最多只能设置为 4096，否则我们服务会返回不合法的输入参数（ invalid_request_error ），
    /// 并拒绝回答。如果您希望获得“输入的精确 token 数”，可以使用下面的“计算 Token” API 使
    /// 用我们的计算器获得计数
    pub max_tokens: Option<i32>,

    /// 使用什么采样温度，介于 0 和 1 之间。
    /// 较高的值（如 0.7）将使输出更加随机，
    /// 而较低的值（如 0.2）将使其更加集中和确定性
    /// 默认为 0，如果设置，值域须为 [0, 1] 我们推荐 0.3，以达到较合适的效果
    pub temperature: Option<f32>,

    /// 另一种采样方法，即模型考虑概率质量为 top_p 的标记的结果。
    /// 因此，0.1 意味着只考虑概率质量最高的 10% 的标记。一般情况下，
    /// 我们建议改变这一点或温度，但不建议 同时改变
    pub top_p: Option<f32>,

    /// 为每条输入消息生成多少个结果
    /// 默认为 1，不得大于 5。特别的，当 temperature 非常小靠近 0 的时候，
    /// 我们只能返回 1 个结果，如果这个时候 n 已经设置并且 > 1，我们的服
    /// 务会返回不合法的输入参数(invalid_request_error)
    pub n: Option<i32>,

    /// 存在惩罚，介于-2.0到2.0之间的数字。
    /// 正值会根据新生成的词汇是否出现在文本中来进行惩罚，
    /// 增加模型讨论新话题的可能性,默认0
    pub presence_penalty: Option<f32>,

    /// 频率惩罚，介于-2.0到2.0之间的数字。
    /// 正值会根据新生成的词汇在文本中现有的频率来进行惩罚，
    /// 减少模型一字不差重复同样话语的可能性
    /// 默认0
    pub frequency_penalty: Option<f32>,

    /// 设置为 {"type": "json_object"} 可启用 JSON 模式，
    /// 从而保证模型生成的信息是有效的 JSON。
    /// 当你将 response_format 设置为 {"type": "json_object"} 时，
    /// 你需要在 prompt 中明确地引导模型输出 JSON 格式的内容，
    /// 并告知模型该 JSON 的具体格式，否则将可能导致不符合预期的结果。
    /// 默认为 {"type": "text"}
    pub response_format: Option<T>,

    /// 停止词，当全匹配这个（组）词后会停止输出，这个（组）词本身不会输出。最多不能超过 5 个字符串，每个字符串不得超过 32 字节
    pub stop: Option<Vec<String>>,

    /// 是否流式返回，默认为 false，如果设置为 true，则返回一个流式响应，每次收到一部分结果
    pub stream: Option<bool>,
}

///单条消息
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone)]
pub struct SingleMessage {
    pub role: Role,
    pub content: String,

    /// 在使用大模型时，有时我们希望通过预填（Prefill）部分模型回复来引导模型的输出。
    /// 在 Kimi 大模型中，我们提供 Partial Mode 来实现这一功能，
    /// 它可以帮助我们控制输出格式，引导输出内容，以及让模型在角色扮演场景中保持更好的一致性。
    /// 您只需要在最后一个 role 为 assistant 的 messages 条目中，
    /// 增加 "partial": True 即可开启 partial mode。
    /// 注意！请勿混用 partial mode 和 response_format=json_object，否则可能会获得预期外的模型回复。
    pub partial: Option<bool>,

    /// 基于同样的原理，我们也可以能将角色信息补充在 Partial Mode 来提高角色扮演时的一致性。
    /// 我们使用明日方舟里的凯尔希医生为例。 注意此时我们还可以在 partial mode 的基础上，
    /// 比如使用 "name":"凯尔希" 字段来更好的保持该角色的一致性，注意这里可视 name 字段为
    /// 输出前缀的一部分。
    pub name: Option<String>,
}

///消息角色类型
#[derive(Serialize, Deserialize, Clone)]
pub enum Role {
    ///系统消息预设定等等
    #[serde(rename = "system")]
    System,
    ///用户消息
    #[serde(rename = "user")]
    User,
    ///模型消息
    #[serde(rename = "assistant")]
    Assistant,
}

///模型类型
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum KimiModel {
    /// kimi的自动选择模型
    #[serde(rename = "moonshot-v1-auto")]
    MoonshotV1Auto,
    /// kimi的8k模型
    #[serde(rename = "moonshot-v1-8k")]
    MoonshotV1_8k,
    /// kimi的32k模型
    #[serde(rename = "moonshot-v1-32k")]
    MoonshotV1_32k,
    /// kimi的128k模型
    #[serde(rename = "moonshot-v1-128k")]
    MoonshotV1_128K,
    ///自定义模型
    #[serde(untagged)]
    Other(String),
}
