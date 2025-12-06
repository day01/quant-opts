# quant-opts – Baseline (legacy Black-Scholes implementation)

These measurements were taken against the original `Inputs`-based API
before the refactor to `core` + `BlackScholes` model. They serve as
reference values for numerical correctness and performance when
validating the new architecture.

## Numerical baselines

All values below are from `src/bin/bs_baseline.rs` executed against the
legacy API in `release` mode.

### Pricing (`Inputs::calc_price` / `Inputs::calc_rational_price`)

- `call_otm`  
  - `price` = `0.0376589547`  
  - `rational_price` = `0.0376589547`
- `call_itm`  
  - `price` = `9.9913356994`  
  - `rational_price` = `9.9913356994`
- `put_otm`  
  - `price` = `0.0186767623`  
  - `rational_price` = `0.0186767623`
- `put_itm`  
  - `price` = `10.0103178918`  
  - `rational_price` = `10.0103178918`
- `branch_cut` (`calc_rational_price`)  
  - `rational_price` = `16.6722548339`

### Implied volatility (`Inputs::calc_rational_iv`)

- `put_otm`  
  - `true_sigma` = `0.25`  
  - `rational_iv` = `0.25`
- `call_itm`  
  - `true_sigma` = `0.15`  
  - `rational_iv` = `0.15`
- `put_itm`  
  - `true_sigma` = `0.18`  
  - `rational_iv` = `0.18`
- `call_atm`  
  - `true_sigma` = `0.20`  
  - `rational_iv` = `0.20`
- `put_atm`  
  - `true_sigma` = `0.22`  
  - `rational_iv` = `0.22`

## Performance baselines

Measured with:

```bash
cargo run --release --bin bs_baseline
```

On 1,000,000 iterations (single-threaded), using the legacy API:

- `Inputs::calc_price`  
  - ~`39.87 ns/op`
- `Inputs::calc_rational_price`  
  - ~`43.01 ns/op`
- `Inputs::calc_rational_iv`  
  - ~`475.28 ns/op`

These numbers are approximate and hardware/compiler dependent, but
serve as a sanity check when comparing the refactored model-based API
(`BlackScholes::price`, `BlackScholes::rational_price`,
`BlackScholes::rational_implied_vol`).

### Greeks (current Black–Scholes API)

Measured with:

```bash
cargo bench --no-default-features --bench greeks
```

On the refactored `BlackScholes` model (single-threaded):

- `BlackScholes::delta`  
  - ~`0.35 ns/op`
- `BlackScholes::gamma`  
  - ~`0.40 ns/op`
- `BlackScholes::theta`  
  - ~`15.55 ns/op`
- `BlackScholes::vega`  
  - ~`0.40 ns/op`
- `BlackScholes::rho`  
  - ~`0.35 ns/op`
- `BlackScholes::greeks` (all first-order greeks at once)  
  - ~`18.65 ns/op`
