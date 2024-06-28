use anchor_lang::prelude::*;

declare_id!("5TJkgRqurg8BjL7SB4dnU7Wd5dSHLurvYNdKXHHo14tA");

#[program]
pub mod account_data {
    use super::*;

    pub fn initialize(
        ctx: Context<CreateAddressInfo>,
        name: String,
        house_number: u8,
        street: String,
        city: String,
    ) -> Result<()> {
        *ctx.accounts.address_info = AddressInfo {
            name,
            house_number,
            street,
            city,
        };
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateAddressInfo<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init, payer = payer, space = 8 + 4 + 50 + 1 + 4 + 50 + 4 + 50)]
    pub address_info: Account<'info, AddressInfo>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct AddressInfo {
    #[max_len(50)]
    pub name: String,
    pub house_number: u8,
    #[max_len(50)]
    pub street: String,
    #[max_len(50)]
    pub city: String,
}
