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
        // state of the vault, we'll use it to withdraw/cancel
        let vault = &mut ctx.accounts.vault;
        vault.authority= ctx.accounts.user_sending.key();
        vault.amount = initializer_amount;
        let user_sending = &ctx.accounts.user_sending;
        let user_sending_token_account = &ctx.accounts.user_sending_token_account;
        let vault_token_account = &ctx.accounts.vault_token_account;
        //transfer token ownership from initializer to vault
        anchor_spl::token::transfer(
            CpiContext::new(
                //program arg
                ctx.accounts.token_program.to_account_info(),
                //accounts args
                anchor_spl::token::Transfer {
                    from: user_sending_token_account.to_account_info(),
                    to: vault_token_account.to_account_info(),
                    authority: user_sending.to_account_info(),
                },
            ),
            initializer_amount,
        )?;

        msg!("Initialized new Fund Transfer instance for {}", initializer_amount);

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let vault_account =  &ctx.accounts.vault_account;
        let user_receiving_token_account = &ctx.accounts.user_receiving_token_account;
        let vault_token_account = &ctx.accounts.vault_token_account;

        anchor_spl::token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer{
                from: vault_token_account.to_account_info(),
                to: user_receiving_token_account.to_account_info(),
                authority: vault_account.to_account_info(),
            }, 
            &[&[ctx.accounts.vault.authority.as_ref(), &[ctx.accounts.vault.bump]]]
        ),     
            amount
        )?;

        Ok(())
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
    #[account()]
    vault_account: Account<'info, Vault>,
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
pub struct Withdraw<'info> {
    //#[account(mut)]
   // pub user_receiving: Signer<'info>,
    #[account(mut)]
    vault_account: Signer<'info>,
    token_mint: Account<'info, Mint>,
    #[account(mut)]
    user_receiving_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = vault_account,
        space = 8 + 8 + 8,
        seeds = ["vault".as_bytes(), vault_account.key().as_ref()],
        bump,
    )]
    pub vault: Account<'info, Vault>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
}

