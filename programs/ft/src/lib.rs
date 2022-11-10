use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

//use token program accounts in Anchor

declare_id!("3Fzp351iQFQHLS2ePYBsE9uiRr9xee5tyyxzUHjEpak2");

#[program]
pub mod ft {
    use super::*;


    pub fn initialize_payment(ctx: Context<InitializePayment>,
    _initializer_amount: u64,
    ) -> Result<()> {
        
        let _payment_initializer = &ctx.accounts.initializer;
        
        let _vault_account = &ctx.accounts.vault_account;

        let _initializer_token_account = &ctx.accounts.initializer_token_account;

        let _vault_token_account = &ctx.accounts.vault_token_account;

        Ok(())

    }
   
}

#[derive(Accounts)]
pub struct InitializePayment<'info> { 
    #[account(mut)]
    signer: Signer<'info>,
    #[account(init, payer = signer, space = 8)]
    initializer: AccountInfo<'info>,
    #[account(mut)] 
    initializer_token_account: Account<'info, TokenAccount>,
    #[account(mut)] 
    vault_token_account: Account<'info, TokenAccount>, 
    vault_account: Account<'info, VaultAccount>, 
    system_program: Program<'info, System>, 
    token_program: Program<'info, Token> }

#[account]
pub struct VaultAccount {
    pub authority: Pubkey,
    pub deposit_token_account: Pubkey,
    pub receiver_token_account: Pubkey,
}
