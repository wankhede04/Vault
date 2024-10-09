use anchor_lang::prelude::*;
use std::mem::size_of;
use anchor_spl::token::{self, Token, TokenAccount};


declare_id!("9ArmAnXSSzomBxTauKd84cLgmVe5q2LAXQj87HAV9hdW");

#[program]
pub mod vault {

    use token::Transfer;

    use super::*;

    pub fn initialize_vault(ctx: Context<Initialize>) -> Result<()> {
        msg!("Initializing vault");
        let vault = &mut ctx.accounts.vault;
        vault.vault_owner = ctx.accounts.signer.key();
        vault.token_mint = ctx.accounts.token_mint.key(); // token which is to be used in vault for deposits & withdrawals
        Ok(())
    }

    pub fn deposit_tokens(ctx: Context<Deposit>, amount: u64) -> Result<()>{
        msg!("Depositing Tokens in vault");
        let res = token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer{
                    from: ctx.accounts.depositor_token_account.to_account_info(),
                    to: ctx.accounts.vault_token_account.to_account_info(),
                    authority: ctx.accounts.depositor.to_account_info() // transfer from user token account
                }
            ),
            amount
        );
        if res.is_ok() {
            Ok(())
        }else {
            return err!(Errors::DepositTokenFailed);
        }
    }

    pub fn withdraw_tokens(ctx: Context<Withdraw>, amount: u64) -> Result<()>{
        msg!("Withdrawing Tokens from Vault");
        let vault_bump = ctx.bumps.vault;
        let seed = &[b"vault".as_ref(), &[vault_bump]];
        let signer: &[&[&[u8]]] = &[&seed[..]];// vault PDA
        let res = token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer{
                    from: ctx.accounts.vault_token_account.to_account_info(),
                    to: ctx.accounts.recipient_token_account.to_account_info(),
                    authority: ctx.accounts.vault.to_account_info() // transfer from vault token account to user
                },
                signer //vault_seeds
            ),
            amount,
        );

        if res.is_ok() {
            Ok(())
        }else {
            err!(Errors::WithdrawTokenFailed)
        }
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = size_of::<Vault>() + 8, seeds = [b"vault"], bump)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub token_mint: Account<'info, token::Mint>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'deposit>{
    #[account(mut)]
    pub vault: Account<'deposit, Vault>,
    #[account(mut)]
    pub vault_token_account: Account<'deposit, TokenAccount>,
    #[account(mut)]
    pub depositor_token_account: Account<'deposit, TokenAccount>,
    pub depositor: Signer<'deposit>,
    pub token_program: Program<'deposit, Token>
}

#[derive(Accounts)]
pub struct Withdraw<'withdraw>{
    #[account(mut, seeds = [b"vault"], bump)]
    pub vault: Account<'withdraw, Vault>,
    #[account(mut)]
    pub vault_token_account: Account<'withdraw, TokenAccount>,
    #[account(mut)]
    pub recipient_token_account: Account<'withdraw, TokenAccount>,
    #[account(mut)]
    pub recipient: Signer<'withdraw>,
    pub token_program: Program<'withdraw, Token>
}

#[account]
pub struct Vault{
    pub vault_owner: Pubkey,
    pub token_mint: Pubkey
}

#[error_code]
pub enum Errors {
    #[msg("Deposit token in vault failed")]
    DepositTokenFailed,
    #[msg("Withdraw token from vault failed")]
    WithdrawTokenFailed,
}