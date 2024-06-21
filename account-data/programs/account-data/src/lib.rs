use anchor_lang::prelude::*;

declare_id!("5TJkgRqurg8BjL7SB4dnU7Wd5dSHLurvYNdKXHHo14tA");

#[program]
pub mod account_data {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
