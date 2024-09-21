use anchor_lang::prelude::*;

pub mod context;
use context::*;

declare_id!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo");

#[program]
pub mod lb_clmm {

    use super::*;

    pub fn swap<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Swap<'info>>,
        amount_in: u64,
        min_amount_out: u64,
    ) -> Result<()> {
        context::handle_exact_in(ctx, amount_in, min_amount_out)
    }
}
