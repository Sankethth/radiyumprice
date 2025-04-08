use anchor_lang::prelude::*;
use borsh::BorshDeserialize;
use solana_program::instruction::Instruction;
use solana_program::program::invoke;
use std::str::FromStr;

use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Token, TokenAccount, Transfer as SplTransfer};
use bytemuck::{Pod, Zeroable};
use primitive_types_solana::U128;
use primitive_types_solana::U256;
use pumpfun_cpi::cpi::accounts::Create as PumpCreateAccounts;
use pumpfun_cpi::cpi::create as pump_create;

pub const MAX_ORDER_LIMIT: usize = 10;

declare_id!("6RCDFGqR38ffYyMHLN8yNNtC5hRQDFBRUn6MC21cemCp");

#[program]
pub mod radiyum_pricetwo {

    use std::string;

    use solana_program::{
        instruction::Instruction, native_token::LAMPORTS_PER_SOL, program::invoke,
    };

    use super::*;

    pub fn fetch_pump_price(ctx: Context<PumpDepositDemo>) -> Result<()> {
        let base_coin_vault = ctx.accounts.base_vault.clone();
        let quote_coin_vault = ctx.accounts.quote_vault.clone();

        if base_coin_vault.mint != ctx.accounts.base_token.key() {
            panic!("Base coin vault is not correct");
        }

        let base_coin_amount = base_coin_vault.amount;

        let quote_coin_amount = quote_coin_vault.amount;

        msg!("Base coin amount is {}", base_coin_amount);

        msg!("Quote coin amount is {}", quote_coin_amount);

        let price: f64 = ((base_coin_amount as f64) / (quote_coin_amount as f64)) * (1000 as f64);

        msg!("Price {}", price);

        Ok(())
    }

    pub fn create_pump_token(
        ctx: Context<CallPumpCreate>,
        mint_a_name: String,
        mint_a_symbol: String,
        mint_a_uri: String,
        mint_b_name: String,
        mint_b_symbol: String,
        mint_b_uri: String,
    ) -> Result<()> {
        let discriminator: [u8; 8] = [24, 30, 200, 40, 5, 28, 7, 119];
        let creator = ctx.accounts.user.key();
    
        create_single_token(
            &ctx,
            MintAccounts {
                mint: ctx.accounts.mint_a_address.to_account_info(),
                bonding_curve: ctx.accounts.mint_a_bonding_curve.to_account_info(),
                associated_bonding_curve: ctx.accounts.mint_a_associated_bonding_curve.to_account_info(),
                metadata: ctx.accounts.mint_a_metadata.to_account_info(),
            },
            &mint_a_name,
            &mint_a_symbol,
            &mint_a_uri,
            discriminator,
            creator,
        )?;
    
        create_single_token(
            &ctx,
            MintAccounts {
                mint: ctx.accounts.mint_b_address.to_account_info(),
                bonding_curve: ctx.accounts.mint_b_bonding_curve.to_account_info(),
                associated_bonding_curve: ctx.accounts.mint_b_associated_bonding_curve.to_account_info(),
                metadata: ctx.accounts.mint_b_metadata.to_account_info(),
            },
            &mint_b_name,
            &mint_b_symbol,
            &mint_b_uri,
            discriminator,
            creator,
        )?;
    
        Ok(())
    }
    
}



fn create_single_token<'info>(
    ctx: &Context<CallPumpCreate<'info>>,
    mint_accounts: MintAccounts<'info>,
    name: &str,
    symbol: &str,
    uri: &str,
    discriminator: [u8; 8],
    creator: Pubkey,
) -> Result<()> {
    let mut instruction_data = discriminator.to_vec();
    let args = Args {
        name: name.to_string(),
        symbol: symbol.to_string(),
        uri: uri.to_string(),
        creator,
    };
    args.serialize(&mut instruction_data)?;

    let account_metas = vec![
        AccountMeta::new(mint_accounts.mint.key(), true),
        AccountMeta::new_readonly(ctx.accounts.mint_authority.key(), false),
        AccountMeta::new(mint_accounts.bonding_curve.key(), false),
        AccountMeta::new(mint_accounts.associated_bonding_curve.key(), false),
        AccountMeta::new_readonly(ctx.accounts.global.key(), false),
        AccountMeta::new_readonly(ctx.accounts.mpl_token_metadata.key(), false),
        AccountMeta::new(mint_accounts.metadata.key(), false),
        AccountMeta::new(ctx.accounts.user.key(), true),
        AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
        AccountMeta::new_readonly(ctx.accounts.token_program.key(), false),
        AccountMeta::new_readonly(ctx.accounts.associated_token_program.key(), false),
        AccountMeta::new_readonly(ctx.accounts.rent.key(), false),
        AccountMeta::new_readonly(ctx.accounts.event_authority.key(), false),
        AccountMeta::new_readonly(ctx.accounts.pump_fun_program.key(), false),
    ];

    let instruction = Instruction {
        program_id: ctx.accounts.pump_fun_program.key(),
        accounts: account_metas,
        data: instruction_data,
    };

    invoke(
        &instruction,
        &[
            mint_accounts.mint.clone(),
            ctx.accounts.mint_authority.to_account_info(),
            mint_accounts.bonding_curve.clone(),
            mint_accounts.associated_bonding_curve.clone(),
            ctx.accounts.global.to_account_info(),
            ctx.accounts.mpl_token_metadata.to_account_info(),
            mint_accounts.metadata.clone(),
            ctx.accounts.user.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.associated_token_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
            ctx.accounts.event_authority.to_account_info(),
            ctx.accounts.pump_fun_program.to_account_info(),
        ],
    )?;

    let clock = Clock::get()?;

    emit!(TokensCreated {
        user: ctx.accounts.user.key(),
        mint_a: ctx.accounts.mint_a_address.key(),
        mint_b: ctx.accounts.mint_b_address.key(),
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

struct MintAccounts<'info> {
    mint: AccountInfo<'info>,
    bonding_curve: AccountInfo<'info>,
    associated_bonding_curve: AccountInfo<'info>,
    metadata: AccountInfo<'info>,
}


#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct PumpDepositDemo<'info> {
    pub quote_vault: Account<'info, TokenAccount>,
    pub base_vault: Account<'info, TokenAccount>,
    /// CHECK : No Check Needed
    pub base_token: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct CallPumpCreate<'info> {

    
    #[account(mut, signer)]
    /// CHECK : No check needed
    pub mint_a_address: AccountInfo<'info>,

    #[account(mut, signer)]
     /// CHECK : No check needed
     pub mint_b_address: AccountInfo<'info>,

    /// CHECK : No check needed
    pub mint_authority: AccountInfo<'info>,

    /// CHECK : No check needed
    #[account(mut)]
    pub mint_a_bonding_curve: AccountInfo<'info>,

    /// CHECK : No check needed
    #[account(mut)]
    pub mint_b_bonding_curve: AccountInfo<'info>,

    /// CHECK : No check needed
    #[account(mut)]
    pub mint_a_associated_bonding_curve: AccountInfo<'info>,

        /// CHECK : No check needed
    #[account(mut)]
    pub mint_b_associated_bonding_curve: AccountInfo<'info>,

    /// CHECK : No check needed
    pub global: AccountInfo<'info>,

    /// CHECK : No check needed
    pub mpl_token_metadata: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK : No check needed
    pub mint_a_metadata: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK : No check needed
    pub mint_b_metadata: AccountInfo<'info>,

    /// CHECK : No check needed
    #[account(mut, signer)]
    pub user: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    /// Associated Token program
    pub associated_token_program: Program<'info, AssociatedToken>,

    pub rent: Sysvar<'info, Rent>,

    /// CHECK : No check needed
    pub event_authority: AccountInfo<'info>,

    /// CHECK : No check needed
    pub pump_fun_program: AccountInfo<'info>,
}

#[derive(AnchorSerialize)]
struct Args {
    name: String,
    symbol: String,
    uri: String,
    creator: Pubkey,
}

#[event]
pub struct TokensCreated {
    pub user: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub timestamp: i64,
}


// #[error_code]
// pub enum MyErrors {
//     #[msg("Subtraction underflow occurred.")]
//     SubtractionUnderflow,
// }
