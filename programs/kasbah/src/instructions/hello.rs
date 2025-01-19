use anchor_lang::prelude::*;

pub fn _hello(ctx: Context<HelloAccounts>, input: HelloInput) -> Result<()> {
    msg!("Hello, from Kasbah! {}", input.id);
    msg!("Signer: {}", ctx.accounts.signer.key());

    Ok(())
}

#[derive(Accounts)]
pub struct HelloAccounts<'info> {
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct HelloInput {
    pub id: u8,
}
