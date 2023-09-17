use anchor_lang::prelude::*;

// declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
declare_id!("BZBTgGa8thX4LbZYMdsBcLVTy9zimwU17UYzgRtGewzq");

#[program]
mod basic_1 {
    use super::*;

    pub fn initialize(ctx: Context<InitializeAccounts>) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = 0;
        Ok(())
    }

    pub fn set(ctx: Context<UpdateAccounts>, data: u64) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = data;
        Ok(())
    }
    
    pub fn increment(ctx: Context<UpdateAccounts>) -> Result<()> {
      let my_account = &mut ctx.accounts.my_account;
      my_account.data += 1;
      Ok(())
  }
  
  pub fn decrement(ctx: Context<UpdateAccounts>) -> Result<()> {
    let my_account = &mut ctx.accounts.my_account;
    my_account.data -= 1; //should check that counter is not zero.
    Ok(())
}
}

#[derive(Accounts)]
pub struct InitializeAccounts<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateAccounts<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
}

#[account]
pub struct MyAccount {
    pub data: u64,
}
