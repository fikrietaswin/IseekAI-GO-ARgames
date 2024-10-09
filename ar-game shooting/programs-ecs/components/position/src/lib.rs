use bolt_lang::*;

declare_id!("79JjQBf6EM7Sj1U8NdzKxxpVeuVGLGAmXRantvAxXMBY");

#[component]
#[derive(Default)]
pub struct Position {
    pub x: i64,
    pub y: i64,
    pub z: i64,
    #[max_len(20)]
    pub description: String,
}
