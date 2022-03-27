use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction;
use crate::state::Store;


pub fn donate(ctx: Context<Donate>, lamports: u64) -> Result<()> {
    let from = &ctx.accounts.from;
    let to = &ctx.accounts.to;
    let result = invoke(&system_instruction::transfer(
        from.key, to.key,
        lamports), &[from.to_account_info(), to.to_account_info()],
    );
    let store = &mut ctx.accounts.store;
    store.put_donate(from.key(), lamports);
    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::from(err))
    }
}

#[derive(Accounts)]
pub struct Donate<'info> {
    #[account(mut)]
    pub from: Signer<'info>,

    #[account(mut)]
    /// CHECK: mut
    pub to: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub store: Account<'info, Store>,
}
