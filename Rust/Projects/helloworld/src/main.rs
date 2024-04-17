// use std::thread::current;
use bigdecimal::{BigDecimal, ToPrimitive};

// use std::result;
// use rug::float::{MiniFloat, Round};
// use rug::ops::DivAssignRound;
// use rug::Float;
// use std::cmp;

// below are AI-generated rust source code
const INITIAL_PRICE: f64 = 0.0001;
const MAX_PRICE: f64 = 0.1;
const ISO_MAX_TVL: f64 = 10000.0;
const MAX_TVL: f64 = 1000000.0;
const TVL_FACTOR: f64 = 1.0;
const SLOPE: f64 = 5.0;

fn calculate_token_price(current_tvl: f64) -> f64 {
    let exponent = -((current_tvl / (MAX_TVL * TVL_FACTOR)).powf(SLOPE));
    INITIAL_PRICE + MAX_PRICE * (1.0 - f64::exp(exponent))
}

#[cfg(feature = "my_feature")]
const MY_VARIABLE: &str = "feature value";

fn main() {
    let mut token_prices: Vec<f64> = Vec::new();

    let mut current_tvl = ISO_MAX_TVL;
    while current_tvl <= 1000000.0 {
        let token_price = calculate_token_price(current_tvl);
        token_prices.push(token_price);
        current_tvl += 10000.0;
    }

    let mut lp_amount = 0.0;

    // Print the token prices
    for (index, token_price) in token_prices.iter().enumerate() {
        
        lp_amount += 10000.0 / token_price;
        println!(
            "CurrentTVL = {}, TokenPrice = {}, LpAmount = {}, step of lp_amount: {}",
            index * 10000,
            token_price,
            lp_amount,
            10000.0 / token_price
        );
    }

    fn calculate_profit(token_prices: &[f64], index1: usize, index2: usize, amount: usize) -> f64 {
        if index1 == index2 {
            return 0.0;
        }
        let mut total_profit = 0.0;
        for i in index1..=index2 {
            let buy_price = token_prices[i];
            let sell_price = token_prices[i + 1];
            let profit = (sell_price - buy_price) * amount as f64;
            total_profit += profit;
            println!(
                "Buy at index {}, Sell at index {}, CurrentProfit {}, TotalProfit: {}",
                index1, i, profit, total_profit
            )
        }

        total_profit
    }

    let index1 = 1; // Set the index to buy
    let index2 = 2; // Set the index to sell
    let amount = 100; // Set the quantity to buy/sell

    let profit = calculate_profit(&token_prices, index1, index2, amount);
    println!("Profit: {}", profit);

}
// /**
//  * Reference: https://math.stackexchange.com/questions/3542734/alternatives-for-sigmoid-curve-starting-from-0-with-interpretable-parameters
//  *
//  * TokenPrice = InitialPrice + MaxPrice * (1 - e^(-(CurrentTVL/(MaxTVL*TVLFactor))^Slope))
//  *
//  * InitialPrice:    the initial token price during fair launch ISO phase
//  * MaxPrice:        the max expected token price (excluding initial price)
//  * CurrentTVL:      the current TVL in reserve token
//  * MaxTVL:          the max expected TVL in reserve token
//  * TVLFactor:       the TVL factor to adjust the scale of bonding curve
//  * Slope:           the slop of bonding curve
//  */
//  // ############ assume an TEST ISO pool ############
//  // - with DAI as reverse token (9 decimals)
//  // - with the initial price 0.0001
//  // - with the max price 0.1
//  // - with max TVL 1M Test Token (1M * 10**9)
//  //
//  // - (1) ISO period with fixed initial price (with a TVL cap, only deposit is allowed)
//  //
//  // - (2) MSF period with bonding curves (the higher the TVL, the smaller the buying & selling curve difference)
//  //
//  // - with max expected TVL
//  // - with predefined number of steps
//  // - with different buying and selling curves
//  //
//  // - creator fee
//  // - protocol sharing fee
//  //
//  // #################################################
// static mut G_INITIAL_PRICE: f64 = 0.0001;
// static mut G_MAX_PRICE: f64 = 0.1;
// static mut G_MAX_TVL: u64 = 1000000;
// static mut G_TVL_FACTOR: f64 = 0.6;
// static mut G_SLOPE: f64 = 5.0;

// fn get_token_price(current_tvl: u64) -> f64 {
//     let current_tvl_f64 = current_tvl as f64;
//     let max_tvl_f64 = unsafe { G_MAX_TVL } as f64;
//     let temp_pow_value = 0.0 - (current_tvl_f64 / (max_tvl_f64 * unsafe { G_TVL_FACTOR })).powf(unsafe { G_SLOPE });
//     let curve_value = 1.0 - std::f64::consts::E.powf(temp_pow_value);
//     let token_price = unsafe { G_INITIAL_PRICE } + unsafe { G_MAX_PRICE } * curve_value;
//     return token_price;
// }

// fn main() {
//     // let current_tvl: u64 = 1000;
//     // let token_price = get_token_price(current_tvl);
//     // println!("token price: {}", token_price);

//     for index in 0..100 {
//         let tvl: u64 = index * 10000;
//         let token_price = get_token_price(tvl);
//         println!("tvl: {}, token price: {}", tvl, token_price);
//     }
// }
