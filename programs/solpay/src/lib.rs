use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("YOUR_PROGRAM_ID");

#[program]
pub mod solpay {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn create_stream(
        ctx: Context<CreateStream>,
        amount: u64,
        duration: i64,
    ) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);
        require!(duration > 0, ErrorCode::InvalidDuration);

        let stream = &mut ctx.accounts.stream;
        let clock = Clock::get()?;

        stream.sender = ctx.accounts.sender.key();
        stream.recipient = ctx.accounts.recipient.key();
        stream.amount = amount;
        stream.start_time = clock.unix_timestamp;
        stream.end_time = clock.unix_timestamp.checked_add(duration).ok_or(ErrorCode::Overflow)?;
        stream.withdrawn_amount = 0;
        stream.bump = *ctx.bumps.get("stream").unwrap();

        let transfer_instruction = Transfer {
            from: ctx.accounts.sender_token.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
            authority: ctx.accounts.sender.to_account_info(),
        };

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                transfer_instruction,
            ),
            amount,
        )?;

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        let stream = &mut ctx.accounts.stream;
        let clock = Clock::get()?;

        require!(clock.unix_timestamp >= stream.start_time, ErrorCode::StreamNotStarted);

        let elapsed_time = clock.unix_timestamp.saturating_sub(stream.start_time);
        let total_time = stream.end_time.saturating_sub(stream.start_time);
        
        let withdrawable_amount = (stream.amount as u128)
            .checked_mul(elapsed_time as u128)
            .and_then(|val| val.checked_div(total_time as u128))
            .and_then(|val| val.try_into().ok())
            .ok_or(ErrorCode::CalculationError)?;

        let withdraw_amount = withdrawable_amount
            .checked_sub(stream.withdrawn_amount)
            .ok_or(ErrorCode::InsufficientFunds)?;

        stream.withdrawn_amount = stream.withdrawn_amount
            .checked_add(withdraw_amount)
            .ok_or(ErrorCode::Overflow)?;

        let seeds = &[
            b"stream",
            stream.sender.as_ref(),
            stream.recipient.as_ref(),
            &[stream.bump],
        ];
        let signer = &[&seeds[..]];

        let transfer_instruction = Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.recipient_token.to_account_info(),
            authority: stream.to_account_info(),
        };

        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                transfer_instruction,
                signer,
            ),
            withdraw_amount,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct CreateStream<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub recipient: AccountInfo<'info>,
    #[account(
        init,
        payer = sender,
        space = 8 + 32 + 32 + 8 + 8 + 8 + 8 + 1,
        seeds = [b"stream", sender.key().as_ref(), recipient.key().as_ref()],
        bump
    )]
    pub stream: Account<'info, PaymentStream>,
    #[account(mut, constraint = sender_token.owner == sender.key())]
    pub sender_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub sender: AccountInfo<'info>,
    #[account(mut)]
    pub recipient: Signer<'info>,
    #[account(
        mut,
        seeds = [b"stream", sender.key().as_ref(), recipient.key().as_ref()],
        bump = stream.bump,
        constraint = stream.recipient == recipient.key(),
    )]
    pub stream: Account<'info, PaymentStream>,
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut, constraint = recipient_token.owner == recipient.key())]
    pub recipient_token: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct PaymentStream {
    pub sender: Pubkey,
    pub recipient: Pubkey,
    pub amount: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub withdrawn_amount: u64,
    pub bump: u8,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Invalid duration")]
    InvalidDuration,
    #[msg("Calculation error")]
    CalculationError,
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Overflow")]
    Overflow,
    #[msg("Stream has not started yet")]
    StreamNotStarted,
}