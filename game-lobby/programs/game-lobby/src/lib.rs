use anchor_lang::prelude::*;

declare_id!("9yvyeDPRwZenYe1zwTWumeMEi3UX8pKJfCorgc66NVpV");

#[program]
pub mod game_lobby {
    use super::*;
    use anchor_lang::solana_program::{program::invoke, system_instruction};

    pub fn initialize(ctx: Context<Initialize>, owner: Pubkey) -> Result<()> {
        let deposit_account = &mut ctx.accounts.deposit_account;
        deposit_account.total_deposits = 0;
        deposit_account.user_deposits = Vec::new();
        deposit_account.owner = owner;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        invoke(
            &system_instruction::transfer(
                &ctx.accounts.user.key(),
                &ctx.accounts.deposit_account.key(),
                amount,
            ),
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.deposit_account.to_account_info(),
            ],
        )?;

        let deposit_account = &mut ctx.accounts.deposit_account;
        deposit_account.total_deposits = deposit_account.total_deposits + amount;

        let user_key = ctx.accounts.user.key();

        match deposit_account
            .user_deposits
            .iter_mut()
            .find(|deposit| deposit.user == user_key)
        {
            Some(deposit) => deposit.amount = deposit.amount + amount,
            None => deposit_account.user_deposits.push(UserDeposit {
                user: user_key,
                amount: amount,
            }),
        }

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let user_key = ctx.accounts.user.key();
        let mut deposit_found = false;

        for deposit in ctx.accounts.deposit_account.user_deposits.iter_mut() {
            if deposit.user == user_key {
                require!(deposit.amount >= amount, ErrorCode::InsufficientFunds);
                deposit_found = true;
                deposit.amount = deposit.amount - amount;
                ctx.accounts.deposit_account.total_deposits =
                    ctx.accounts.deposit_account.total_deposits - amount;
                break;
            }
        }

        if !deposit_found {
            return Err(ErrorCode::InsufficientFunds.into());
        }

        // Check if the deposit_account has enough lamports to cover the withdrawal
        let deposit_account_lamports = **ctx
            .accounts
            .deposit_account
            .to_account_info()
            .lamports
            .borrow();
        require!(
            deposit_account_lamports >= amount,
            ErrorCode::InsufficientFunds
        );

        // Perform the lamport transfer
        **ctx.accounts.user.to_account_info().lamports.borrow_mut() += amount;
        **ctx
            .accounts
            .deposit_account
            .to_account_info()
            .lamports
            .borrow_mut() -= amount;

        Ok(())
    }

    pub fn get_total_deposit(ctx: Context<GetTotalDeposits>) -> Result<()> {
        let total_deposit = &ctx.accounts.deposit_account;
        msg!("Total Deposit {}", total_deposit.total_deposits);
        Ok(())
    }

    pub fn create_game(ctx: Context<CreateGame>, wager: u64) -> Result<()> {
        require!(
            ctx.accounts.owner.key() == ctx.accounts.deposit_account.owner,
            ErrorCode::Unauthorized
        );

        require!(
            ctx.accounts.player1.balance >= wager,
            ErrorCode::InsufficientFunds
        );
        require!(
            ctx.accounts.player2.balance >= wager,
            ErrorCode::InsufficientFunds
        );

        let game = &mut ctx.accounts.game;
        game.player1 = ctx.accounts.player1.key();
        game.player2 = ctx.accounts.player2.key();
        game.wager = wager;
        game.is_active = true;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 40 + 500 + 32)]
    pub deposit_account: Account<'info, DepositAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DepositAccount {
    pub total_deposits: u64,
    pub user_deposits: Vec<UserDeposit>,
    pub owner: Pubkey,
}

#[derive(Accounts)]
pub struct GetTotalDeposits<'info> {
    pub deposit_account: Account<'info, DepositAccount>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct UserDeposit {
    pub user: Pubkey,
    pub amount: u64,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub deposit_account: Account<'info, DepositAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub deposit_account: Account<'info, DepositAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("The deposit amount is not the correct value")]
    InvalidAmount,
    #[msg("Insufficient funds for withdrawal.")]
    InsufficientFunds,
    #[msg("Unauthorized access.")]
    Unauthorized, // Added error code for unauthorized access
}

#[derive(Accounts)]
pub struct CreateGame<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub player1: Account<'info, Player>,
    #[account(mut)]
    pub player2: Account<'info, Player>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub deposit_account: Account<'info, DepositAccount>,
}

#[account]
pub struct Player {
    pub balance: u64,
    pub wager: u64,
    pub wins: u64,
    pub losses: u64,
    pub total_bets: u64,
    pub total_won: u64,
    pub total_lost: u64,
}

#[account]
pub struct Game {
    pub player1: Pubkey,
    pub player2: Pubkey,
    wager: u64,
    is_active: bool,
}
