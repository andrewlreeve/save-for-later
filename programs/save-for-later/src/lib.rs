use anchor_lang::prelude::*;

declare_id!("42a8gq9MKp3oH6YKvemYQrveQ1BwpAaRqBNJyYyJxmzw");

#[program]
pub mod myepicproject {
    use super::*;

    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_tweets = 0;
        Ok(())
    }

    pub fn add_tweet_id(ctx: Context<AddTweetId>, tweet_id: u64) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        let item = ItemStruct {
            tweet_id: tweet_id,
            user_address: *user.to_account_info().key,
        };

        base_account.tweet_ids.push(item);
        base_account.total_tweets += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddTweetId<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub tweet_id: u64,
    pub user_address: Pubkey,
}

#[account]
pub struct BaseAccount {
    pub total_tweets: u64,
    pub tweet_ids: Vec<ItemStruct>,
}
