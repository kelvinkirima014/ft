use anchor_lang::prelude::*;
//use token program accounts in Anchor

declare_id!("3Fzp351iQFQHLS2ePYBsE9uiRr9xee5tyyxzUHjEpak2");

#[program]
pub mod ft {
    use super::*;


    pub fn initialize_payment(_ctx: Context<InitializePayment>,
    _initializer_amount: u64,
    ) -> Result<()> {
        Ok(())
    }
   
}

#[derive(Accounts)]
pub struct InitializePayment<'info> {
    pub initializer: AccountInfo<'info>,
}