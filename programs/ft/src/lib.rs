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
pub struct InitializeTransfer {}

#[derive(Accounts)]
pub struct Exchange{
    //todo!
}
