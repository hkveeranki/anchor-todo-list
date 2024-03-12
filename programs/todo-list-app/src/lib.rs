use anchor_lang::prelude::*;

declare_id!("BwLPzhxWQf2Bqvz3rfXpGh5ayu6reNRZTJwryKsj5rKq");

#[program]
pub mod todo_list_app {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
