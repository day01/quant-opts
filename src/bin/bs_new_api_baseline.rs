use std::time::Instant;

use quant_opts::{BlackScholes, MarketData, OptionStyle, OptionType, VanillaOption};

fn main() {
    println!("== New API pricing baseline (BlackScholes) ==");
    pricing_baseline();
    println!();

    println!("== New API implied volatility baseline (rational) ==");
    iv_baseline();
    println!();

    println!("== New API timing baseline (release) ==");
    timing_baseline();
}

fn pricing_baseline() {
    let call_otm = VanillaOption::new(
        OptionStyle::European,
        OptionType::Call,
        110.0,
        20.0 / 365.25,
    );
    let call_itm = VanillaOption::new(OptionStyle::European, OptionType::Call, 90.0, 20.0 / 365.25);
    let put_otm = VanillaOption::new(OptionStyle::European, OptionType::Put, 90.0, 20.0 / 365.25);
    let put_itm = VanillaOption::new(OptionStyle::European, OptionType::Put, 110.0, 20.0 / 365.25);

    let branch_cut = VanillaOption::new(OptionStyle::European, OptionType::Put, 100.0, 1.0);

    let mkt_common = MarketData::new(100.0, 0.05, 0.05);
    let mkt_branch = MarketData::new(100.0, 0.0, 0.0);

    let sigma = 0.2;
    let sigma_branch = 0.421;

    for (name, opt, mkt) in [
        ("call_otm", &call_otm, &mkt_common),
        ("call_itm", &call_itm, &mkt_common),
        ("put_otm", &put_otm, &mkt_common),
        ("put_itm", &put_itm, &mkt_common),
    ] {
        let p = BlackScholes::price(opt, mkt, sigma).unwrap();
        let p_rational = BlackScholes::rational_price(opt, mkt, sigma).unwrap();
        println!(
            "{name}: price = {:.10}, rational_price = {:.10}",
            p, p_rational
        );
    }

    let branch_price =
        BlackScholes::rational_price(&branch_cut, &mkt_branch, sigma_branch).unwrap();
    println!("branch_cut rational_price = {:.10}", branch_price);
}

fn iv_baseline() {
    let cases = [
        (
            "put_otm",
            VanillaOption::new(OptionStyle::European, OptionType::Put, 100.0, 45.0 / 365.25),
            MarketData::new(90.0, 0.03, 0.02),
            0.25,
        ),
        (
            "call_itm",
            VanillaOption::new(
                OptionStyle::European,
                OptionType::Call,
                100.0,
                60.0 / 365.25,
            ),
            MarketData::new(120.0, 0.01, 0.0),
            0.15,
        ),
        (
            "put_itm",
            VanillaOption::new(OptionStyle::European, OptionType::Put, 100.0, 60.0 / 365.25),
            MarketData::new(80.0, 0.04, 0.03),
            0.18,
        ),
        (
            "call_atm",
            VanillaOption::new(
                OptionStyle::European,
                OptionType::Call,
                100.0,
                90.0 / 365.25,
            ),
            MarketData::new(100.0, 0.05, 0.04),
            0.20,
        ),
        (
            "put_atm",
            VanillaOption::new(
                OptionStyle::European,
                OptionType::Put,
                100.0,
                120.0 / 365.25,
            ),
            MarketData::new(100.0, 0.06, 0.01),
            0.22,
        ),
    ];

    for (name, opt, mkt, true_sigma) in cases {
        let price = BlackScholes::price(&opt, &mkt, true_sigma).unwrap();
        let iv = BlackScholes::rational_implied_vol(price, &opt, &mkt).unwrap();
        println!(
            "{name}: true_sigma = {:.10}, rational_iv = {:.10}",
            true_sigma, iv
        );
    }
}

fn timing_baseline() {
    const N: u64 = 1_000_000;

    println!("Measurements for N = {N} iterations (single-threaded)");

    let opt = VanillaOption::new(OptionStyle::European, OptionType::Call, 100.0, 1.0);
    let mkt = MarketData::new(100.0, 0.05, 0.01);
    let sigma = 0.2;

    // price
    let start = Instant::now();
    let mut acc = 0.0;
    for _ in 0..N {
        acc += BlackScholes::price(&opt, &mkt, sigma).unwrap();
    }
    let elapsed = start.elapsed();
    let ns_per_op = elapsed.as_nanos() as f64 / N as f64;
    println!(
        "BlackScholes::price:               ~{:.2} ns/op (acc={:.4})",
        ns_per_op, acc
    );

    // rational_price
    let start = Instant::now();
    let mut acc = 0.0;
    for _ in 0..N {
        acc += BlackScholes::rational_price(&opt, &mkt, sigma).unwrap();
    }
    let elapsed = start.elapsed();
    let ns_per_op = elapsed.as_nanos() as f64 / N as f64;
    println!(
        "BlackScholes::rational_price:      ~{:.2} ns/op (acc={:.4})",
        ns_per_op, acc
    );

    // rational_implied_vol
    let price = BlackScholes::price(&opt, &mkt, sigma).unwrap();
    let start = Instant::now();
    let mut acc = 0.0;
    for _ in 0..N {
        acc += BlackScholes::rational_implied_vol(price, &opt, &mkt).unwrap();
    }
    let elapsed = start.elapsed();
    let ns_per_op = elapsed.as_nanos() as f64 / N as f64;
    println!(
        "BlackScholes::rational_implied_vol:~{:.2} ns/op (acc={:.4})",
        ns_per_op, acc
    );
}
