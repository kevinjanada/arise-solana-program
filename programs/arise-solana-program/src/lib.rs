use anchor_lang::prelude::*;

declare_id!("8cUdKAzTmeJbrT8xR81Ai5qeHKKWdKfEguA4tpae1EDL");

#[program]
pub mod arise_solana_program {
    use super::*;

    pub fn create_user_data(ctx: Context<CreateUserData>, email: String) -> Result<()> {
        // TODO: validate email to be 100 
        if email.as_bytes().len() > 100 {
            panic!()
        }
        ctx.accounts.user_data.email = email;
        ctx.accounts.user_data.authority = ctx.accounts.authority.key();
        ctx.accounts.user_data.bump = *ctx.bumps.get("user_data").unwrap();
        Ok(())
    }

    pub fn update_user_data(ctx: Context<UpdateUserData>, email: String) -> Result<()> {
        // TODO: validate email to be 100 
        if email.as_bytes().len() > 100 {
            panic!()
        }
        ctx.accounts.user_data.email = email;
        ctx.accounts.user_data.authority = ctx.accounts.authority.key();
        ctx.accounts.user_data.bump = *ctx.bumps.get("user_data").unwrap();
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(email: String)]
pub struct CreateUserData<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = UserData::MAXIMUM_SIZE,
        seeds = [b"user-data", email.as_bytes()],
        bump
    )]
    pub user_data: Account<'info, UserData>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(email: String)]
pub struct UpdateUserData<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        close = authority,
        has_one = authority @ Error::CloseUserDataAccount,
        seeds = [b"user-data", existing_user_data.email.as_bytes()],
        bump = existing_user_data.bump
    )]
    pub existing_user_data: Account<'info, UserData>,
    #[account(
        init,
        payer = authority,
        space = UserData::MAXIMUM_SIZE,
        seeds = [b"user-data", email.as_bytes()],
        bump
    )]
    pub user_data: Account<'info, UserData>,
    pub system_program: Program<'info, System>
}

#[error_code]
pub enum Error {
    #[msg("Unable to close account")]
    CloseUserDataAccount
}

#[account]
#[derive(Default)]
pub struct UserData {
    email: String,
    authority: Pubkey,
    bump: u8,
}

impl UserData {
    pub const MAX_EMAIL_LENGTH: usize = 100;
    pub const EMAIL_LENGTH_INFO: usize = 4; // used by borsh
    pub const ANCHOR_DISCRIMINATOR: usize = 8;
    pub const PUBLIC_KEY_LENGTH: usize = 32;
    pub const BUMP: usize = 1;

    pub const MAXIMUM_SIZE: usize =
        Self::ANCHOR_DISCRIMINATOR +
        Self::EMAIL_LENGTH_INFO +
        Self::MAX_EMAIL_LENGTH +
        Self::PUBLIC_KEY_LENGTH +
        Self::BUMP;
}
