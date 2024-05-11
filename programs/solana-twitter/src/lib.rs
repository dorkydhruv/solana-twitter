use anchor_lang::prelude::*;

declare_id!("9vu5GaSZNDos3hmK9bb9RiDapCkYcxJHccGy3qQ8iDkj");

#[program]
pub mod solana_twitter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
