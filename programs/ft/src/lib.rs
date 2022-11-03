use anchor_lang::prelude::*;

declare_id!("3Fzp351iQFQHLS2ePYBsE9uiRr9xee5tyyxzUHjEpak2");

#[program]
pub mod ft {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>,
    _initializer_amount: u64,
    ) -> Result<()> {
        Ok(())
    }

    pub fn deposit(_ctx: Context<Deposit>) -> Result<()> {
        todo!()
    }

    pub fn withdraw(_ctx: Context<Withdraw>) -> Result<()> {
        todo!()
    }

    pub fn pull_out(_ctx: Context<PullOut>) -> Result<()> {
        todo!()
    }
}

#[derive(Accounts)]
pub struct Initialize {
    
}

#[derive(Accounts)]
pub struct Deposit {

}

#[derive(Accounts)]
pub struct Withdraw {

}

#[derive(Accounts)]
pub struct PullOut{
    
}



