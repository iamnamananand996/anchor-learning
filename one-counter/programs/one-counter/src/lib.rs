use anchor_lang::prelude::*;

declare_id!("GabNw63q5xUNayLk1f72wewy7JCw4Dk5a5gj5SUae2nh");

#[program]
pub mod one_counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &ctx.accounts.counter;
        msg!("Counter account created! Current count: {}", counter.count);
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter: {}", counter.count);

        counter.count = counter.count + 1;

        msg!("Counter incremented! Current count: {}", counter.count);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,         // specifies we are creating this account
        payer = user, // specifies account paying for the creation of the account
        space = 8 + 8 // space allocated to the new account (8 byte discriminator + 8 byte for u64)
    )]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

#[account]
pub struct Counter {
    pub count: u64, // define count value type as u64
}
