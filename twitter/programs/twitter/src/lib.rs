// use anchor_lang::prelude::*;
// use anchor_lang::solana_program::entrypoint::ProgramResult;

// declare_id!("8S6EBpxCU6ceCFWDPtdibCK6HCGY1yaURGdsvcPybRav");

// #[program]
// pub mod twitter {
//     use super::*;

//     pub fn initialize(ctx: Context<SendTweet>, topic: String, content: String) -> Result<()> {
//         let tweet = &mut ctx.accounts.tweet;
//         let author = &ctx.accounts.author;
//         let clock = Clock::get().unwrap();

//         if topic.chars().count() > 50 {
//             return Err(ErrorCode::TopicTooLong.into());
//         }

//         if content.chars().count() > 280 {
//             return Err(ErrorCode::ContentTooLong.into());
//         }

//         tweet.author = *author.key;
//         tweet.timestamp = clock.unix_timestamp;
//         tweet.topic = topic;
//         tweet.content = content;

//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct SendTweet<'info> {
//     #[account(init, payer = author, space = Tweet::LEN)]
//     pub tweet: Account<'info, Tweet>,
//     #[account(mut)]
//     pub author: Signer<'info>,
//     pub system_program: Program<'info, System>,
// }

// #[account]
// pub struct Tweet {
//     pub author: Pubkey,
//     pub timestamp: i64,
//     pub topic: String,
//     pub content: String,
// }

// const DISCRIMINATOR_LENGTH: usize = 8;
// const PUBLIC_KEY_LENGTH: usize = 32;
// const TIMESTAMP_LENGTH: usize = 8;
// const STRING_LENGTH_PREFIX: usize = 4; // Stores the size of the string.
// const MAX_TOPIC_LENGTH: usize = 50 * 4; // 50 chars max.
// const MAX_CONTENT_LENGTH: usize = 280 * 4;

// // 3. Add a constant on the Tweet account that provides its total size.
// impl Tweet {
//     const LEN: usize = DISCRIMINATOR_LENGTH
//         + PUBLIC_KEY_LENGTH // Author.
//         + TIMESTAMP_LENGTH // Timestamp.
//         + STRING_LENGTH_PREFIX + MAX_TOPIC_LENGTH // Topic.
//         + STRING_LENGTH_PREFIX + MAX_CONTENT_LENGTH; // Content.
// }

// #[error_code]
// pub enum ErrorCode {
//     #[msg("The provided topic should be 50 character long maximum")]
//     TopicTooLong,
//     #[msg("The provided content should be 280 charters long maximum")]
//     ContentTooLong,
// }

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
