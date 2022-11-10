use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

//use token program accounts in Anchor

declare_id!("ANMC4r582ErAaCrFFJZQ9PhkxtPmFpWFMkoZEEQT1mvk");

#[program]
pub mod ft {
    use super::*;


    pub fn initialize_payment(ctx: Context<InitializePayment>,
    _initializer_amount: u64,
    ) -> Result<()> {
        
        let _initializer = &ctx.accounts.initializer;
        
        let _vault_account = &ctx.accounts.vault;

        let _initializer_token_account = &ctx.accounts.initializer_token_account;

        let _vault_token_account = &ctx.accounts;

        ctx.accounts.vault.amount = _initializer_amount;

        //transfer token ownership from initializer to vault

        Ok(())

    }
   
}

#[derive(Accounts)]
pub struct InitializePayment<'info> { 
    #[account(mut)]
    initializer: Signer<'info>,
    token_mint: Account<'info, Mint>,
    #[account(mut)] 
    initializer_token_account: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = initializer,
        space = 8 + 8 + 8,
        seeds = ["vault".as_bytes(), initializer.key().as_ref()],
        bump,
    )]  
    pub vault: Account<'info, Vault>, 
    system_program: Program<'info, System>, 
    token_program: Program<'info, Token> 

}

#[account]
pub struct Vault {
    authority: Pubkey,
    vault_token_account: Pubkey,
    bump: u8,
    amount: u64,
}
