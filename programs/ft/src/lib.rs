use anchor_lang::prelude::*;
//use token program accounts in Anchor

declare_id!("3Fzp351iQFQHLS2ePYBsE9uiRr9xee5tyyxzUHjEpak2");

#[program]
pub mod ft {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
       let mut base_account = *ctx.accounts.my_data;
       base_account.data = 0;
        Ok(())
    }

    pub fn set_data(ctx: Context<SetData>) -> Result<()> {
        let mut base_account = *ctx.accounts.my_data;
        base_account.data += 1; 
        Ok(())
    }
    #[derive(Copy)]
    #[account()]
    pub struct MyData{
        pub data: u8,
    }

    #[derive(Accounts)]
    pub struct SetData<'info>{
        #[account(mut)]
        pub my_data: Account<'info, MyData>,
    }

    #[derive(Accounts)]
    pub struct Initialize<'info> {
        #[account(init, payer = signer, space = 8)]
        initializer: AccountInfo<'info>,
        #[account(mut)]
        signer: Signer<'info>,
        system_program: Program<'info, System>,
        #[account(mut)]
        my_data: Account<'info, MyData>,
    }


    // pub fn initialize_payment(_ctx: Context<InitializePayment>,
    // _initializer_amount: u64,
    // ) -> Result<()> {
    //     Ok(())
    // }

    // pub fn deposit(_ctx: Context<Deposit>) -> Result<()> {
    //     todo!()
    // }

    // pub fn withdraw(_ctx: Context<Withdraw>) -> Result<()> {
    //     todo!()
    // }

    // pub fn pull_out(_ctx: Context<PullOut>) -> Result<()> {
    //     todo!()
    // }
}

// #[derive(Accounts)]
// pub struct InitializePayment<'info > {
//     #[account(init, payer = signer, space = 8+8)]
//     user_key: AccountInfo<'info >,
//     #[account(mut)]
//     signer: Signer<'info>,
//     system_program: Program<'info, System>,
// }

// #[derive(Accounts)]
// pub struct Deposit {

// }

// #[derive(Accounts)]
// pub struct Withdraw {

// }

// #[derive(Accounts)]
// pub struct PullOut{

// }

//3 parties: Alice, Bob, Program(Escrow)
//accounts for each of these parties
//System program, Token Program
//most of our logic is around thesse paries


