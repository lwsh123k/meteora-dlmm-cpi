use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub lb_pair: AccountInfo<'info>,

    pub bin_array_bitmap_extension: AccountInfo<'info>,

    #[account(mut)]
    pub reserve_x: AccountInfo<'info>,
    #[account(mut)]
    pub reserve_y: AccountInfo<'info>,

    #[account(mut)]
    pub user_token_in: AccountInfo<'info>,
    #[account(mut)]
    pub user_token_out: AccountInfo<'info>,

    pub token_x_mint: AccountInfo<'info>,
    pub token_y_mint: AccountInfo<'info>,

    #[account(mut)]
    pub oracle: AccountInfo<'info>,

    #[account(mut)]
    pub host_fee_in: Option<AccountInfo<'info>>,

    pub user: AccountInfo<'info>,
    pub token_x_program: AccountInfo<'info>,
    pub token_y_program: AccountInfo<'info>,
}

pub fn handle_exact_in<'a, 'b, 'c, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, Swap<'info>>,
    amount_in: u64,
    min_amount_out: u64,
) -> Result<()> {
    Ok(())
}

pub fn handle_exact_out<'a, 'b, 'c, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, Swap<'info>>,
    max_in_amount: u64,
    exact_out_amount: u64,
) -> Result<()> {
    Ok(())
}
pub fn handle_exact_in_with_price_impact<'a, 'b, 'c, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, Swap<'info>>,
    amount_in: u64,
    active_id: Option<i32>,
    max_price_impact_bps: u16,
) -> Result<()> {
    Ok(())
}
