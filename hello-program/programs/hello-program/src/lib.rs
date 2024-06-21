use anchor_lang::prelude::*;

declare_id!("EPdraVsTAbiXpUuoWA67Z6xWL9k83qwti7dDhmAtsFMM");

#[program]
pub mod hello_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
