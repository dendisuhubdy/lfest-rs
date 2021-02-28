mod load_trades;

use lfest::{Config, Exchange, Order, Side};
use load_trades::load_trades_from_csv;
use rand::{thread_rng, Rng};
use std::time::Instant;

fn main() {
    let t0 = Instant::now();

    let config = Config {
        fee_maker: -0.00025,
        fee_taker: 0.001,
        starting_balance_base: 1.0,
        use_candles: false,
        leverage: 1.0,
    };
    let mut exchange = Exchange::new(config);

    // load trades from csv file
    let trades = load_trades_from_csv("./data/Bitmex_XBTUSD_1M.csv").unwrap();

    // use random action to buy or sell
    let mut rng = thread_rng();

    for t in &trades {
        let liq = exchange.consume_trade(t);
        if liq {
            println!(
                "position liquidated, \
            but there could still be enough wallet_balance to open a new position"
            );
        }

        // randomly buy or sell using a market order
        let r = rng.gen::<f64>();
        // Trade a fraction of the available wallet balance
        let order_size: f64 = exchange.margin().wallet_balance() * 0.01;
        let order: Order = if r > 0.98 {
            // Sell order
            Order::market(Side::Sell, order_size)
        } else if r < 0.02 {
            // BUY
            Order::market(Side::Buy, order_size)
        } else {
            // Neutral
            continue;
        };
        let _order_err = exchange.submit_order(order);
        // Handle order error here if needed
    }
    println!(
        "time to simulate 1 million historical trades and {} orders: {}ms",
        exchange.acc_tracker().num_trades(),
        t0.elapsed().as_millis()
    );
    analyze_results(&exchange);
}

/// analyzer the resulting performance metrics of the traded orders
fn analyze_results(e: &Exchange) {
    let rpnl = e.acc_tracker().total_rpnl();
    let sharpe = e.acc_tracker().sharpe();
    let sortino = e.acc_tracker().sortino();
    let sterling_ratio = e.acc_tracker().sharpe_sterling_ratio();
    let max_drawdown = e.acc_tracker().max_drawdown();
    let max_upnl_drawdown = e.acc_tracker().max_upnl_drawdown();
    let num_trades = e.acc_tracker().num_trades();
    let buy_ratio = e.acc_tracker().buy_ratio();
    let turnover = e.acc_tracker().turnover();
    let win_ratio = e.acc_tracker().win_ratio();
    println!(
        "rpnl: {:.2}, sharpe: {:.2}, sortino: {:.2}, sr: {:.2}, \
    dd: {:.2}, upnl_dd: {:.2}, #trades: {}, buy_ratio: {:.2}, turnover: {}, win_ratio: {}",
        rpnl,
        sharpe,
        sortino,
        sterling_ratio,
        max_drawdown,
        max_upnl_drawdown,
        num_trades,
        buy_ratio,
        turnover,
        win_ratio
    );
}
