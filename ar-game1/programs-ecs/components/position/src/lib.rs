use bolt_lang::*;

declare_id!("8kjeiyq3Zh7XHDHPgNocbkunSLeDBgcbbUTfnQbkqiN6");

#[component]
#[derive(Default)]
pub struct Position {
    pub x: i64,
    pub y: i64,
    pub z: i64,
    #[max_len(20)]
    pub description: String,
}
