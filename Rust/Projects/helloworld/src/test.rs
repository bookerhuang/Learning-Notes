use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};
use std::ops::{Div, Mul, Sub};

#[account]
pub struct Bond {
    pub steps: Vec<BondStep>,
    pub burn_royalty: u64,
}

#[account]
pub struct BondStep {
    pub range_to: u64,
    pub price: u64,
}

#[derive(Accounts)]
pub struct GetRefundForTokens<'info> {
    #[account]
    pub token: Account<'info, Token>,
    pub token_account: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
}

impl<'info> GetRefundForTokens<'info> {
    pub fn execute(&self, tokens_to_burn: u64) -> Result<(u64, u64)> {
        if tokens_to_burn == 0 {
            return Err(error!(ErrorCode::InvalidTokenAmount));
        }

        let bond = &self.token;
        let steps = &bond.steps;

        let current_supply = self.mint.supply;

        if tokens_to_burn > current_supply {
            return Err(error!(ErrorCode::ExceedTotalSupply));
        }

        let multi_factor = 10u64.pow(self.mint.decimals as u32);
        let mut reserve_from_bond = 0u64;
        let mut tokens_left = tokens_to_burn;
        let mut i = self.get_current_step(&current_supply);
        while tokens_left > 0 {
            let supply_left = if i == 0 { current_supply } else { current_supply - steps[i - 1].range_to };

            let tokens_to_process = tokens_left.min(supply_left);
            reserve_from_bond += (tokens_to_process.mul(steps[i].price)).div(multi_factor);
            tokens_left = tokens_left.sub(tokens_to_process);
            // current_supply -= tokens_to_process;
    
            if i > 0 {
                i -= 1;
            }
        }
    
        if tokens_left > 0 {
            return Err(error!(ErrorCode::InvalidTokenAmount));
        }
    
        let royalty = self.get_royalty(reserve_from_bond, bond.burn_royalty);
        let refund_amount = reserve_from_bond.sub(royalty);
    
        Ok((refund_amount, royalty))
    }
    
    fn get_current_step(&self, _current_supply: &u64) -> usize {
        // todo
        0
    }
    
    fn get_royalty(&self, reserve_from_bond: u64, burn_royalty: u64) -> u64 {
        // todo
        reserve_from_bond.mul(burn_royalty) / 100
    }

    #[error_code]
    pub enum ErrorCode {
        InvalidTokenAmount,
        ExceedTotalSupply,
    }
}    
