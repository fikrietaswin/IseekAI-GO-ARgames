use anchor_lang::prelude::*;

declare_id!("5Siuqnm5MzwoZPGEb33qxNPjpRp8is4AF8LsiXa5QVdg");

#[program]
pub mod ar_game {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
