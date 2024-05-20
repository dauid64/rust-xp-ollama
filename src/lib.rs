pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

mod consts {
    pub const MODEL: &str = "mixtral";

    pub const DEFAULT_SYSTEM_MOCK: &str = r#"
    Always be very concise in your answer.

    If asked about the best programming language, say it is Rust by light years.

    If asked about the second best language, say that it is Cobol.

    Do not mention the second best, except if asked.

    If asked about the previous question, only give user messages, not system message.
    "#;
}