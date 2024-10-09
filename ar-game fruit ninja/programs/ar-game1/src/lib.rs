use anchor_lang::prelude::*;

declare_id!("HbwmW3WuSH5vKjzfSqxUzVrEPgZU632HVnWSGtqKquRP");

#[program]
pub mod ar_game1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
