use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

//use token program accounts in Anchor

declare_id!("ANMC4r582ErAaCrFFJZQ9PhkxtPmFpWFMkoZEEQT1mvk");

#[program]
pub mod ft {
    use super::*;

    pub fn initialize_payment(
        ctx: Context<InitializePayment>,
        initializer_amount: u64,
    ) -> Result<()> {
        //state of the vault, we'll use it to withdraw/cancel
        // let vault = &mut ctx.accounts.vault;
        // vault.authority= ctx.accounts.user_sending.key();

        let user_sending = &ctx.accounts.user_sending_token_account;
        let user_sending_token_account = &ctx.accounts.user_sending_token_account;
        let vault_token_account = &ctx.accounts.vault_token_account;
        ctx.accounts.vault.amount = initializer_amount;
        //transfer token ownership from initializer to vault
        anchor_spl::token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: user_sending_token_account.to_account_info(),
                    to: vault_token_account.to_account_info(),
                    authority: user_sending.to_account_info(),
                },
            ),
            initializer_amount,
        )?;

        Ok(())
    }

    pub fn withdraw(_ctx: Context<Withdraw>, _amount: u64) -> Result<()> {
        todo!()
    }

}

#[account]
pub struct Vault {
    authority: Pubkey,
    bump: u8,
    amount: u64,
}


#[derive(Accounts)]
pub struct InitializePayment<'info> {
    #[account(mut)]
    user_sending: Signer<'info>,
    token_mint: Account<'info, Mint>,
    #[account(mut)]
    user_sending_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    vault_token_account: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = user_sending,
        space = 8 + 8 + 8,
        seeds = ["vault".as_bytes(), user_sending.key().as_ref()],
        bump,
    )]
    pub vault: Account<'info, Vault>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct  Withdraw {

}

