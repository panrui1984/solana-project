pub mod instructions;
use instructions::*;

use anchor_lang::prelude::*;

// 请将下方的 ID 替换为你自己 `anchor keys list` 得到的 ID
declare_id!("22222222222222222222222222222222222222222222"); 

#[program]
pub mod anchor_escrow {
    use super::*;

    // 指令：创建托管
    pub fn make(
        ctx: Context<Make>, 
        seed: u64, 
        deposit_amount: u64, 
        receive_amount: u64
    ) -> Result<()> {
        instructions::make::make(ctx, seed, deposit_amount, receive_amount)
    }

    // 指令：完成托管
    pub fn take(ctx: Context<Take>) -> Result<()> {
        instructions::take::take(ctx)
    }

    // 指令：退款
    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        instructions::refund::refund(ctx)
    }
}

// ---------------- State (Escrow Account) ----------------
// 为了方便，直接定义在这里。也可以放在 src/state/escrow.rs

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub seed: u64,
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub receive_amount: u64,
    pub bump: u8,
}