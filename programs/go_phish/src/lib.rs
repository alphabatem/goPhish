use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_spl::associated_token;
use anchor_spl::token;
use std::convert::TryFrom;

declare_id!("DW5y8reFZuDN1DfxDnpeRqFXrusBcEJxErB8Mv4Lc4Vj");

#[derive(Debug)]
#[account]
pub struct Phish {
    owner: Pubkey,
    n: u64,
}

pub fn init_handler(mut ctx: Context<Init>) -> Result<()> {
    let mut owner = &mut ctx.accounts.owner;
    let mut phish = &mut ctx.accounts.phish;
    let mut phish = phish;

    phish.n = 0;

    phish.owner = owner.key();

    Ok(())
}

pub fn go_phish_handler(mut ctx: Context<GoPhish>, mut url: String) -> Result<()> {
    let mut owner = &mut ctx.accounts.owner;
    let mut phish = &mut ctx.accounts.phish;

    msg!("{}", format!("{}~{}", url, phish.key()));

    Ok(())
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        seeds = ["randomness".as_bytes().as_ref(), owner.key().as_ref()],
        bump,
        space = 8 + std::mem::size_of::<Phish>()
    )]
    pub phish: Box<Account<'info, Phish>>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
# [instruction (url : String)]
pub struct GoPhish<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub phish: Box<Account<'info, Phish>>,
}

#[program]
pub mod go_phish {
    use super::*;

    pub fn init(ctx: Context<Init>) -> Result<()> {
        init_handler(ctx)
    }

    pub fn go_phish(ctx: Context<GoPhish>, url: String) -> Result<()> {
        go_phish_handler(ctx, url)
    }
}
