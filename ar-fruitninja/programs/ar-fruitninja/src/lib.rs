use anchor_lang::prelude::*;

declare_id!("DhZ2Z7CETH3iLQzw4TiUmiU1CcNJYvphFa4eNop3UptF");

#[program]
pub mod ar_fruitninja {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
