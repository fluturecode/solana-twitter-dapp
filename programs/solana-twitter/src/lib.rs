use anchor_lang::prelude::*;

declare_id!("5tWvUdpE4wkTAff4FvcVQZdG8e1m2FqaWSaRPd7vSUMg");

#[program]
pub mod solana_twitter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
