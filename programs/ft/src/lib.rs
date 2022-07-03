use anchor_lang::prelude::*;

declare_id!("3Fzp351iQFQHLS2ePYBsE9uiRr9xee5tyyxzUHjEpak2");

#[program]
pub mod ft {
    use super::*;

    pub fn initialize(_ctx: Context<InitializeTransfer>,
    _initializer_amount: u64,
    ) -> Result<()> {
        Ok(())
    }

    pub fn exchange(_ctx: Context<Exchange>) -> Result<()> {
        todo!()
    }
}

#[derive(Accounts)]
#[account(
    init,
    payer = user_sending,
    seeds=[b"state".as_ref(), user_sending.key().as_ref(),
    user_receiving.key().as_ref()],
    bump, 
)]
pub struct InitializeTransfer {}

#[derive(Accounts)]
pub struct Exchange{
    //todo!
}


//store state for our program
#[account]
#[derive(Default)]
pub struct State{

    //Alice Pubkey
    user_sending: Pubkey,

    //Bob
    user_receiving: Pubkey,

    //mint of token that Alice want to send Bob
    mint_of_token_being_sent: Pubkey,

    //the amount of tokens Alice wants to send
    tokens_amount: u64,

    //enum to act as state machine
    stage: u8,
}