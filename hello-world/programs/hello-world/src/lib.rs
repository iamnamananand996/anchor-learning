use anchor_lang::prelude::*;

declare_id!("3Pm9nnZVFtKu8ms8C6JaZtCHnijkTLKF599sPmM195Vf");

#[program]
pub mod hello_world {
    use std::process::id;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello Solana");
        msg!("Our Program's Program ID: {}", &id());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
