use anchor_lang::prelude::*;

// declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
// declare_id!("BZBTgGa8thX4LbZYMdsBcLVTy9zimwU17UYzgRtGewzq");
declare_id!("BjwWGc5BR2EyAJnPmaj2ge3hyQXDxx4iLgHTaxcqBLvs");

#[program]
mod basic_1 {
    use super::*;

    pub fn initialize(ctx: Context<InitializeAccountInfo>) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = 0;
        Ok(())
    }

    pub fn set(ctx: Context<UpdateAccountInfo>, data: u64) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = data;
        Ok(())
    }
    
    pub fn increment(ctx: Context<UpdateAccountInfo>) -> Result<()> {
      let my_account = &mut ctx.accounts.my_account;
      my_account.data += 1;
      Ok(())
  }
  
  pub fn decrement(ctx: Context<UpdateAccountInfo>) -> Result<()> {
    let my_account = &mut ctx.accounts.my_account;
    my_account.data -= 1; //should check that counter is not zero.
    Ok(())
}
}

#[derive(Accounts)]
pub struct InitializeAccountInfo<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateAccountInfo<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
}

#[account]
pub struct MyAccount {
    pub data: u64,
}
