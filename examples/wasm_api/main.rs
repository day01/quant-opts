// This example is intended for the `wasm32-unknown-unknown` target.
// For non-WASM targets we provide a stub `main` so `cargo check --examples` succeeds.

#[cfg(target_arch = "wasm32")]
mod wasm_example {
    use quant_opts::{BlackScholes, MarketData, OptionStyle, OptionType, VanillaOption};
    use wasm_bindgen::prelude::*;

    // Expose a minimal WASM API for pricing and implied volatility.
    // Build with:
    //   cargo build --target wasm32-unknown-unknown --example wasm_bindgen
    // Then run wasm-bindgen to generate JS bindings (see README instructions).

    #[wasm_bindgen]
    pub fn price_call_bs(
        spot: f64,
        strike: f64,
        maturity_years: f64,
        rate: f64,
        dividend_yield: f64,
        vol: f64,
    ) -> Result<f64, JsValue> {
        let opt = VanillaOption::new(
            OptionStyle::European,
            OptionType::Call,
            strike,
            maturity_years,
        );
        let mkt = MarketData::new(spot, rate, dividend_yield);
        BlackScholes::price(&opt, &mkt, vol).map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen]
    pub fn rational_iv_bs(
        observed_price: f64,
        spot: f64,
        strike: f64,
        maturity_years: f64,
        rate: f64,
        dividend_yield: f64,
    ) -> Result<f64, JsValue> {
        let opt = VanillaOption::new(
            OptionStyle::European,
            OptionType::Call,
            strike,
            maturity_years,
        );
        let mkt = MarketData::new(spot, rate, dividend_yield);
        BlackScholes::rational_implied_vol(observed_price, &opt, &mkt)
            .map_err(|e| JsValue::from_str(&e))
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // This example only runs on wasm32. See examples/wasm/README.md for build steps.
}
