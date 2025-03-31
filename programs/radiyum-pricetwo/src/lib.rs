use anchor_lang::prelude::*;
use borsh::BorshDeserialize;

use anchor_spl::token::{self, Token, TokenAccount, Transfer as SplTransfer};
use bytemuck::{ Pod, Zeroable};
use primitive_types_solana::U128;
use primitive_types_solana::U256;

pub const MAX_ORDER_LIMIT: usize = 10;

declare_id!("6RCDFGqR38ffYyMHLN8yNNtC5hRQDFBRUn6MC21cemCp");

#[program]
pub mod radiyum_pricetwo {
    
    use solana_program::native_token::LAMPORTS_PER_SOL;

    use super::*;

    pub fn fetch_pump_price(ctx:Context<PumpDepositDemo>) -> Result<()>{

        let base_coin_vault = ctx.accounts.base_vault.clone();
        let quote_coin_vault = ctx.accounts.quote_vault.clone();

       if base_coin_vault.mint != ctx.accounts.base_token.key() {
           panic!("Base coin vault is not correct");
       
       }

         let base_coin_amount = base_coin_vault.amount;

         let quote_coin_amount = quote_coin_vault.amount;

         msg!("Base coin amount is {}",base_coin_amount);

         msg!("Quote coin amount is {}",quote_coin_amount);

         let price:f64 = ((base_coin_amount as f64) / (quote_coin_amount as f64)) * (1000 as f64);

         msg!("Price {}",price);

         Ok(())



    }

    // pub fn fetch(ctx: Context<DepositDemo>) -> Result<()> {
    //     // msg!(
    //     //     "2 Account data length: {}",
    //     //     ctx.accounts.amm_info.data_len()
    //     // );
    //     // msg!("Expected size: {}", std::mem::size_of::<AmmInfo>());

    //     let amm = &mut AmmInfo::try_from(&ctx.accounts.amm_info).unwrap();
    //     let amm_coin_vault = ctx.accounts.amm_coin_vault.clone();
    //     let amm_pc_vault = ctx.accounts.amm_pc_vault.clone();
    //     let target_order = ctx.accounts.target_orders.clone();

    //     let target_orders_data: TargetOrders = {
    //         let data = target_order.try_borrow_data()?;
    //         *bytemuck::from_bytes::<TargetOrders>(&data)
    //     };

    //     // msg!(&format!("coin decimals {}", amm.coin_decimals));

    //     msg!(&format!("pc decimals {}", amm.pc_decimals));

    //     // msg!(&format!("u128 value: {}", target_orders_data.calc_pnl_x));

    //     let mut total_pc_without_take_pnl: u64;
    //     let mut total_coin_without_take_pnl: u64;
    //     (total_pc_without_take_pnl, total_coin_without_take_pnl) =
    //         calc_total_without_take_pnl_no_orderbook(
    //             amm_pc_vault.amount,
    //             amm_coin_vault.amount,
    //             &amm,
    //         )
    //         .unwrap();

    //     msg!("total_pc_without_take_pnl is {}", total_pc_without_take_pnl);

    //     msg!(
    //         "total_coin_without_take_pnl is {}",
    //         total_coin_without_take_pnl
    //     );

    //     let x = normalize_decimal_v2(
    //         amm_pc_vault.amount as u128,
    //         amm.pc_decimals as u32,
    //         amm.sys_decimal_value,
    //     );
    //     let y = normalize_decimal_v2(
    //         amm_coin_vault.amount as u128,
    //         amm.coin_decimals as u32,
    //         amm.sys_decimal_value,
    //     );
    //     msg!(&format!("x value: {}", x));
    //     msg!(&format!("y value: {}", y));
    //     let (pc_total, coin_total, delta_x, delta_y, pc_pnl, coin_pnl) = calc_take_pnl(
    //         &target_orders_data,
    //         amm,
    //         &mut total_pc_without_take_pnl,
    //         &mut total_coin_without_take_pnl,
    //         x.into(),
    //         y.into(),
    //     )?;

    //     msg!("pc_total is {}", pc_total);

    //     msg!("coin_total is {}", coin_total);
    //     let price = U128::from(LAMPORTS_PER_SOL)
    //             .checked_mul(pc_total.into())
    //             .unwrap()
    //             .checked_div(coin_total.into())
    //             .unwrap();
         

    //     let price_u128: u128 = price.as_u128();

    //     msg!("Price is {}", price);

    //     emit!(Price {
    //         coin_total: coin_total,
    //         pc_total: pc_total,
    //         price: price_u128
    //     });

    //     Ok(())
    // }
}

// pub fn calc_total_without_take_pnl_no_orderbook<'a>(
//     pc_amount: u64,
//     coin_amount: u64,
//     amm: &'a AmmInfo,
// ) -> Result<(u64, u64)> {
//     let total_pc_without_take_pnl = pc_amount
//         .checked_sub(amm.state_data.need_take_pnl_pc)
//         .ok_or(MyErrors::SubtractionUnderflow)?;
//     let total_coin_without_take_pnl = coin_amount
//         .checked_sub(amm.state_data.need_take_pnl_coin)
//         .ok_or(MyErrors::SubtractionUnderflow)?;
//     Ok((total_pc_without_take_pnl, total_coin_without_take_pnl))
// }

// pub fn normalize_decimal_v2(val: u128, native_decimal: u32, sys_decimal_value: u64) -> u128 {
//     let ret_mut = val.checked_mul(sys_decimal_value.into()).unwrap();
//     let ret = ret_mut.checked_div((10u128).pow(native_decimal)).unwrap();
//     ret
// }

// pub fn calc_take_pnl(
//     target: &TargetOrders,
//     amm: &mut AmmInfo,
//     total_pc_without_take_pnl: &mut u64,
//     total_coin_without_take_pnl: &mut u64,
//     x1: U256,
//     y1: U256,
// ) -> Result<(u64, u64, u128, u128, u64, u64)> {
//     let calc_pc_amount = restore_decimal(
//         target.calc_pnl_x.into(),
//         amm.pc_decimals,
//         amm.sys_decimal_value,
//     );
//     let calc_coin_amount = restore_decimal(
//         target.calc_pnl_y.into(),
//         amm.coin_decimals,
//         amm.sys_decimal_value,
//     );

//     let pool_pc_amount = U128::from(*total_pc_without_take_pnl);
//     let pool_coin_amount = U128::from(*total_coin_without_take_pnl);
//     let mut delta_x: u128 = 0;
//     let mut delta_y: u128 = 0;
//     let mut pc_pnl_amount: u64 = 0;
//     let mut coin_pnl_amount = 0;

//     if pool_pc_amount.checked_mul(pool_coin_amount).unwrap()
//         >= calc_pc_amount.checked_mul(calc_coin_amount).unwrap()
//     {
//         msg!(&format!("last checked value x: {}", target.calc_pnl_x));
//         msg!(&format!("last checked value y: {}", target.calc_pnl_y));

//         let x2_power = calc_x_power(target.calc_pnl_x.into(), target.calc_pnl_y.into(), x1, y1);
//         let x2 = x2_power.integer_sqrt();
//         let y2 = x2.checked_mul(y1).unwrap().checked_div(x1).unwrap();

//         let diff_x = U128::from(x1.checked_sub(x2).unwrap().as_u128());
//         let diff_y = U128::from(y1.checked_sub(y2).unwrap().as_u128());

//         delta_x = diff_x
//             .checked_mul(amm.fees.pnl_numerator.into())
//             .unwrap()
//             .checked_div(amm.fees.pnl_denominator.into())
//             .unwrap()
//             .as_u128();
//         delta_y = diff_y
//             .checked_mul(amm.fees.pnl_numerator.into())
//             .unwrap()
//             .checked_div(amm.fees.pnl_denominator.into())
//             .unwrap()
//             .as_u128();

//         let diff_pc_pnl_amount = restore_decimal(diff_x, amm.pc_decimals, amm.sys_decimal_value);
//         msg!("diff_pc_pnl_amount {}",diff_pc_pnl_amount);
//         let diff_coin_pnl_amount =
//             restore_decimal(diff_y, amm.coin_decimals, amm.sys_decimal_value);
//         msg!("diff_coin_pnl_amount {}",diff_coin_pnl_amount);

//         pc_pnl_amount = diff_pc_pnl_amount
//             .checked_mul(amm.fees.pnl_numerator.into())
//             .unwrap()
//             .checked_div(amm.fees.pnl_denominator.into())
//             .unwrap()
//             .as_u64();
//         coin_pnl_amount = diff_coin_pnl_amount
//             .checked_mul(amm.fees.pnl_numerator.into())
//             .unwrap()
//             .checked_div(amm.fees.pnl_denominator.into())
//             .unwrap()
//             .as_u64();
//         msg!("total_pc_without_take_pnl {}",total_pc_without_take_pnl);
//         msg!("total_coin_without_take_pnl {}",total_coin_without_take_pnl);
//         msg!("pc_pnl_amount {}",pc_pnl_amount);
//         msg!("coin_pnl_amount {}",coin_pnl_amount);
        
//         if pc_pnl_amount != 0 && coin_pnl_amount != 0 {

//             *total_pc_without_take_pnl = (*total_pc_without_take_pnl)
//                 .checked_sub(pc_pnl_amount)
//                 .unwrap();
//             *total_coin_without_take_pnl = (*total_coin_without_take_pnl)
//                 .checked_sub(coin_pnl_amount)
//                 .unwrap();
//         }
//     }

//     Ok((
//         *total_pc_without_take_pnl,
//         *total_coin_without_take_pnl,
//         delta_x,
//         delta_y,
//         pc_pnl_amount,
//         coin_pnl_amount,
//     ))
// }

// pub fn restore_decimal(val: U128, native_decimal: u64, sys_decimal_value: u64) -> U128 {
//     let ret_mut = val
//         .checked_mul(U128::from(10).checked_pow(native_decimal.into()).unwrap())
//         .unwrap();
//     let ret = ret_mut.checked_div(sys_decimal_value.into()).unwrap();
//     ret
// }

// pub fn calc_x_power(last_x: U256, last_y: U256, current_x: U256, current_y: U256) -> U256 {
//     // let reduced_y = (last_y as f64) / (current_y as f64);

//     // // Convert back to a scaled integer with moderate scaling to avoid overflow
//     // let scaled_reduced_y = (reduced_y * 1_000.0).round() as u128; // Use smaller scaling factor

//     // // Perform calculations
//     // let intermediate = last_x * scaled_reduced_y; // Intermediate result
//     // let result = (intermediate  * current_x)/1000;
//     // return result;

//     last_x
//         .checked_mul(last_y)
//         .unwrap()
//         .checked_div(current_y)
//         .unwrap()
//         .checked_mul(current_x)
//         .unwrap()

//     //last_x*last_y*current_x/current_y;
// }

// #[event]
// pub struct Price {
//     pub pc_total: u64,
//     pub coin_total: u64,
//     pub price: u128,
// }

#[derive(Accounts)]
pub struct Initialize {}

// #[cfg_attr(feature = "client", derive(Debug))]
// #[repr(C, packed)]
// #[derive(Clone, Copy, Default)]
// pub struct TargetOrder {
//     pub price: u64,
//     pub vol: u64,
// }

// #[cfg(target_endian = "little")]
// unsafe impl Zeroable for TargetOrder {}
// #[cfg(target_endian = "little")]
// unsafe impl Pod for TargetOrder {}
// #[cfg(target_endian = "little")]
// #[repr(C)]
// #[derive(Clone, Copy)]
// pub struct TargetOrderArray([TargetOrder; 50]);

// #[cfg(target_endian = "little")]
// unsafe impl Zeroable for TargetOrderArray {}
// #[cfg(target_endian = "little")]
// unsafe impl Pod for TargetOrderArray {}

// #[repr(C)]
// #[derive(Clone, Copy, Pod, Zeroable)]
// pub struct TargetOrders {
//     pub owner: [u64; 4],
//     pub buy_orders: TargetOrderArray,
//     pub padding1: [u64; 8],
//     pub target_x: u128,
//     pub target_y: u128,
//     pub plan_x_buy: u128,
//     pub plan_y_buy: u128,
//     pub plan_x_sell: u128,
//     pub plan_y_sell: u128,
//     pub placed_x: u128,
//     pub placed_y: u128,
//     pub calc_pnl_x: u128,
//     pub calc_pnl_y: u128,
//     pub sell_orders: TargetOrderArray,
//     pub padding2: [u64; 6],
//     pub replace_buy_client_id: [u64; MAX_ORDER_LIMIT],
//     pub replace_sell_client_id: [u64; MAX_ORDER_LIMIT],
//     pub last_order_numerator: u64,
//     pub last_order_denominator: u64,
//     pub plan_orders_cur: u64,
//     pub place_orders_cur: u64,
//     pub valid_buy_order_num: u64,
//     pub valid_sell_order_num: u64,
//     pub padding3: [u64; 10],
//     pub free_slot_bits: u128,
// }

// impl Default for TargetOrders {
//     #[inline]
//     fn default() -> TargetOrders {
//         TargetOrders {
//             owner: [0; 4],
//             buy_orders: TargetOrderArray([TargetOrder::default(); 50]),
//             padding1: [0; 8],
//             target_x: 0,
//             target_y: 0,
//             plan_x_buy: 0,
//             plan_y_buy: 0,
//             plan_x_sell: 0,
//             plan_y_sell: 0,
//             placed_x: 0,
//             placed_y: 0,
//             calc_pnl_x: 0,
//             calc_pnl_y: 0,
//             sell_orders: TargetOrderArray([TargetOrder::default(); 50]),
//             padding2: [0; 6],
//             replace_buy_client_id: [0; MAX_ORDER_LIMIT],
//             replace_sell_client_id: [0; MAX_ORDER_LIMIT],
//             last_order_denominator: 0,
//             last_order_numerator: 0,
//             plan_orders_cur: 0,
//             place_orders_cur: 0,
//             valid_buy_order_num: 0,
//             valid_sell_order_num: 0,
//             padding3: [0; 10],
//             free_slot_bits: std::u128::MAX,
//         }
//     }
// }
// #[repr(C)]
// #[derive(Clone, Debug, Copy, Default, PartialEq, Pod, Zeroable)]
// pub struct AmmInfo {
//     /// Initialized status.
//     pub status: u64,
//     /// Nonce used in program address.
//     /// The program address is created deterministically with the nonce,
//     /// amm program id, and amm account pubkey.  This program address has
//     /// authority over the amm's token coin account, token pc account, and pool
//     /// token mint.
//     pub nonce: u64,
//     /// max order count
//     pub order_num: u64,
//     /// within this range, 5 => 5% range
//     pub depth: u64,
//     /// coin decimal
//     pub coin_decimals: u64,
//     /// pc decimal
//     pub pc_decimals: u64,
//     /// amm machine state
//     pub state: u64,
//     /// amm reset_flag
//     pub reset_flag: u64,
//     /// min size 1->0.000001
//     pub min_size: u64,
//     /// vol_max_cut_ratio numerator, sys_decimal_value as denominator
//     pub vol_max_cut_ratio: u64,
//     /// amount wave numerator, sys_decimal_value as denominator
//     pub amount_wave: u64,
//     /// coinLotSize 1 -> 0.000001
//     pub coin_lot_size: u64,
//     /// pcLotSize 1 -> 0.000001
//     pub pc_lot_size: u64,
//     /// min_cur_price: (2 * amm.order_num * amm.pc_lot_size) * max_price_multiplier
//     pub min_price_multiplier: u64,
//     /// max_cur_price: (2 * amm.order_num * amm.pc_lot_size) * max_price_multiplier
//     pub max_price_multiplier: u64,
//     /// system decimal value, used to normalize the value of coin and pc amount
//     pub sys_decimal_value: u64,
//     /// All fee information
//     pub fees: Fees,
//     /// Statistical data
//     pub state_data: StateData,
//     /// Coin vault
//     pub coin_vault: Pubkey,
//     /// Pc vault
//     pub pc_vault: Pubkey,
//     /// Coin vault mint
//     pub coin_vault_mint: Pubkey,
//     /// Pc vault mint
//     pub pc_vault_mint: Pubkey,
//     /// lp mint
//     pub lp_mint: Pubkey,
//     /// open_orders key
//     pub open_orders: Pubkey,
//     /// market key
//     pub market: Pubkey,
//     /// market program key
//     pub market_program: Pubkey,
//     /// target_orders key
//     pub target_orders: Pubkey,
//     /// padding
//     pub padding1: [u64; 8],
//     /// amm owner key
//     pub amm_owner: Pubkey,
//     /// pool lp amount
//     pub lp_amount: u64,
//     /// client order id
//     pub client_order_id: u64,
//     /// recent epoch
//     pub recent_epoch: u64,
//     /// padding
//     pub padding2: u64,

//     pub padding3: [u8; 16],
// }

// impl<'info> From<&AccountInfo<'info>> for AmmInfo {
//     fn from(account_info: &AccountInfo<'info>) -> Self {
//         let data = account_info.try_borrow_data().unwrap();
//         *bytemuck::from_bytes::<AmmInfo>(&data)
//     }
// }

// impl<'info> From<&AccountInfo<'info>> for TargetOrders {
//     fn from(account_info: &AccountInfo<'info>) -> Self {
//         let data = account_info.try_borrow_data().unwrap();
//         *bytemuck::from_bytes::<TargetOrders>(&data)
//     }
// }

// #[repr(C)]
// #[derive(Clone, Debug, Copy, Default, PartialEq, Pod, Zeroable)]
// pub struct Fees {
//     /// numerator of the min_separate
//     pub min_separate_numerator: u64,
//     /// denominator of the min_separate
//     pub min_separate_denominator: u64,

//     /// numerator of the fee
//     pub trade_fee_numerator: u64,
//     /// denominator of the fee
//     /// and 'trade_fee_denominator' must be equal to 'min_separate_denominator'
//     pub trade_fee_denominator: u64,

//     /// numerator of the pnl
//     pub pnl_numerator: u64,
//     /// denominator of the pnl
//     pub pnl_denominator: u64,

//     /// numerator of the swap_fee
//     pub swap_fee_numerator: u64,
//     /// denominator of the swap_fee
//     pub swap_fee_denominator: u64,
// }

// #[repr(C, packed)]
// #[derive(Clone, Debug, Copy, Default, PartialEq, Pod, Zeroable)]
// pub struct StateData {
//     /// delay to take pnl coin
//     pub need_take_pnl_coin: u64,
//     /// delay to take pnl pc
//     pub need_take_pnl_pc: u64,
//     /// total pnl pc
//     pub total_pnl_pc: u64,
//     /// total pnl coin
//     pub total_pnl_coin: u64,

//     /// swap coin in amount
//     pub swap_coin_in_amount: u128,
//     /// swap pc out amount
//     pub swap_pc_out_amount: u128,
//     /// charge pc as swap fee while swap pc to coin
//     pub swap_acc_pc_fee: u64,

//     /// swap pc in amount
//     pub swap_pc_in_amount: u128,
//     /// swap coin out amount
//     pub swap_coin_out_amount: u128,
//     /// charge coin as swap fee while swap coin to pc
//     pub swap_acc_coin_fee: u64,

//     /// padding to ensure correct size
//     pub padding: [u8; 16],
// }

// #[derive(Accounts)]
// pub struct DepositDemo<'info> {
//     /// CHECK: No check needed
//     #[account(mut)]
//     pub amm_info: AccountInfo<'info>,

//     /// CHECK : No check needed
//     #[account(mut)]
//     pub target_orders: AccountInfo<'info>,
//     pub amm_coin_vault: Account<'info, TokenAccount>,
//     pub amm_pc_vault: Account<'info, TokenAccount>,
//     pub token_program: Program<'info, Token>,
//     pub authority: Signer<'info>,
//     pub system_program: Program<'info, System>,
// }

#[derive(Accounts)]
pub struct PumpDepositDemo<'info> {

    pub quote_vault: Account<'info, TokenAccount>,
    pub base_vault: Account<'info, TokenAccount>,
    /// CHECK : No Check Needed
    pub base_token: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

// #[error_code]
// pub enum MyErrors {
//     #[msg("Subtraction underflow occurred.")]
//     SubtractionUnderflow,
// }
