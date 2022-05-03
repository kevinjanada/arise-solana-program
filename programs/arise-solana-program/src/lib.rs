use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod arise_solana_program {
    use super::*;

    pub fn create_user_data(ctx: Context<CreateUserData>, email: String) -> Result<()> {
        ctx.accounts.user_data.public_key = ctx.accounts.user.key();
        // TODO: validate email to be 100 
        if email.as_bytes().len() > 100 {
            panic!()
        }
        ctx.accounts.user_data.email = email;
        ctx.accounts.user_data.bump = *ctx.bumps.get("user_data").unwrap();
        Ok(())
    }

    pub fn update_user_data_email(ctx: Context<UpdateUserDataEmail>, email: String) -> Result<()> {
        // TODO: validate email to be 100 
        if email.as_bytes().len() > 100 {
            panic!()
        }
        ctx.accounts.user_data.email = email;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateUserData<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = UserData::MAXIMUM_SIZE,
        seeds = [b"user-data", user.key().as_ref()],
        bump
    )]
    pub user_data: Account<'info, UserData>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct UpdateUserDataEmail<'info> {
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"user-data", user.key().as_ref()],
        bump = user_data.bump
    )]
    pub user_data: Account<'info, UserData>
}

#[account]
#[derive(Default)]
pub struct UserData {
    email: String,
    public_key: Pubkey,
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
