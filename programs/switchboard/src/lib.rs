use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock;
pub use switchboard_v2::{
    history_buffer::AggregatorHistoryBuffer, AggregatorAccountData, SwitchboardDecimal,
    SWITCHBOARD_V2_DEVNET,
};

use std::convert::TryInto;

declare_id!("Eyx38NvnsWRpQJho1fTzj7BvJB29d4JTJYTrkVUiQ4Ur");

#[program]

pub mod switchboard {
    use super::*;
    pub fn create_sol_feed(ctx: Context<CreatePrizeFeedAccount>) -> Result<()> {
        let feed_vec_acc = &mut ctx.accounts.feed_vector_acc;
        feed_vec_acc.authority = ctx.accounts.authority.key();
        feed_vec_acc.data_spread = 0f64;

        Ok(())
    }

    pub fn add_sol_feed_data(ctx: Context<ReadPriceFeed>, period: i64, days: u64) -> Result<()> {
        let cur_time = clock::Clock::get().unwrap().unix_timestamp;
        let feed_acc = &mut ctx.accounts.feed_acc;
        feed_acc.days = days;
        let feed = &mut ctx.accounts.feed_acc.feed;
        let mut period_mut = period;

        let history_buffer = ctx.accounts.history_buffer.to_account_info();
        let history_buffer_acc = AggregatorHistoryBuffer::new(&history_buffer).unwrap();

        loop {
            let result = history_buffer_acc.lower_bound(cur_time - period_mut);
            match result {
                Some(data) => {
                    let sol_price: f64 = data.value.try_into().unwrap();
                    let timestamp = data.timestamp;
                    let feed_struct = FeedStruct {
                        sol_price: sol_price,
                        timestamp: timestamp as f64,
                    };
                    feed.push(feed_struct);
                    period_mut = period_mut + period;
                }
                None => {
                    break;
                }
            }
        }

        Ok(())
    }

    pub fn data_spread_calculate(ctx: Context<DataSpreadCalculate>) -> Result<()> {
        let feed_acc = &mut ctx.accounts.feed_vector_acc;
        let nod = feed_acc.days;
        let feed_vector = &feed_acc.feed;
        let feed_array = feed_vector.as_slice();

        let mut sum = 0f64;
        for i in feed_array.iter() {
            sum = sum + i.sol_price;
        }
        let mean = sum / (feed_array.len() as f64);
        let mut sum_sq_devi = 0f64;
        for i in feed_array.iter() {
            let dev_i2 = (i.sol_price - mean).powi(2);
            sum_sq_devi = sum_sq_devi + dev_i2;
        }
        let sd = (sum_sq_devi / (feed_array.len() as f64)).sqrt();

        feed_acc.data_spread = sd * (nod as f64).sqrt();
        msg!("{:?}", sd);
        return Ok(());
    }

    pub fn empty_account(ctx: Context<ResetAccount>) -> Result<()> {
        let feed_acc = &mut ctx.accounts.feed_vec_acc;
        feed_acc.feed.clear();

        Ok(())
    }
}

#[derive(Debug)]
#[account]
pub struct SolanaFeed {
    pub feed: Vec<FeedStruct>,
    pub days: u64,
    pub data_spread: f64,
    pub authority: Pubkey,
}
#[derive(Accounts)]
pub struct CreatePrizeFeedAccount<'info> {
    #[account(init ,payer = authority,space = 8+1600+32+8)]
    pub feed_vector_acc: Account<'info, SolanaFeed>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReadPriceFeed<'info> {
    /// CHECK:safe account
    pub history_buffer: AccountInfo<'info>,
    #[account(mut,has_one = authority)]
    pub feed_acc: Account<'info, SolanaFeed>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct DataSpreadCalculate<'info> {
    #[account(mut,has_one = authority)]
    pub feed_vector_acc: Account<'info, SolanaFeed>,
    pub authority: Signer<'info>,
}
#[derive(Accounts)]
pub struct ResetAccount<'info> {
    #[account(mut,has_one=authority)]
    pub feed_vec_acc: Account<'info, SolanaFeed>,
    pub authority: Signer<'info>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone)]
pub struct FeedStruct {
    sol_price: f64,
    timestamp: f64,
}
