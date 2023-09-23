extern crate clap;
extern crate prettytable;
extern crate mysql;
extern crate serde_json;

use mysql::{Pool, params};
use mysql::prelude::Queryable;
use clap::{App, Arg};
use prettytable::{Table, Row, Cell};
use serde_json::json;

fn calculate_sell_data(high_percent: f64, low_percent: f64, start_price: f64, units: f64) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let step = (high_percent - low_percent) / 9.0;
    let percentages: Vec<f64> = (0..10).map(|i| low_percent + i as f64 * step).collect();
    let di: Vec<f64> = percentages.iter().map(|p| (start_price * (1.0 + p)).round()).collect();

    let phi: f64 = 1.218033988749895;
    let pa_raw: Vec<f64> = (0..10).map(|i| phi.powf(-i as f64)).collect::<Vec<f64>>();
    let sum_pa_raw: f64 = pa_raw.iter().sum();
    let pa_normalized: Vec<f64> = pa_raw.iter().map(|x| x * units / sum_pa_raw).collect();
    let pa = pa_normalized.iter().rev().cloned().collect::<Vec<f64>>();

    let final_price: Vec<f64> = di.iter().zip(pa.iter())
    .map(|(d, p)| (d * p * 100.0).floor() / 100.0)
    .collect();
    return (percentages, di, pa, final_price);
}

fn calculate_buy_data(high_percent: f64, low_percent: f64, current_price: f64, total_amount: f64) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let step = (high_percent - low_percent) / 9.0;
    let percentages: Vec<f64> = (0..10).map(|i| low_percent + i as f64 * step).collect();
    let buy_point: Vec<f64> = percentages.iter().map(|p| (current_price * (1.0 - p)).round()).collect();

    let phi: f64 = 1.218033988749895;
    let amount_raw: Vec<f64> = (0..10).map(|i| phi.powf(-i as f64)).collect::<Vec<f64>>();
    let sum_amount_raw: f64 = amount_raw.iter().sum();
    let amount_normalized: Vec<f64> = amount_raw.iter().map(|x| x * total_amount / sum_amount_raw).collect();
    let amount_dollar: Vec<f64> = amount_normalized.iter()
    .map(|x| (x * 100.0).floor() / 100.0)
    .rev()
    .collect();

    let cost: Vec<f64> = amount_dollar.iter().zip(buy_point.iter()).map(|(a, b)| (a / b)).collect();
    return (percentages, buy_point, amount_dollar, cost);
}

fn generate_sell_table(percentages: Vec<f64>, di: Vec<f64>, pa: Vec<f64>, final_price: Vec<f64>) {
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Percentages"),
        Cell::new("Sell Point"),
        Cell::new("Coin"),
        Cell::new("Gain"),
    ]));

    for i in 0..10 {
        table.add_row(Row::new(vec![
            Cell::new(&percentages[i].to_string()),
            Cell::new(&di[i].to_string()),
            Cell::new(&pa[i].to_string()),
            Cell::new(&final_price[i].to_string()),
        ]));
    }
    table.printstd();
}

fn generate_buy_table(percentages: Vec<f64>, buy_point: Vec<f64>, amount_dollar: Vec<f64>, cost: Vec<f64>) {
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Percentages"),
        Cell::new("Buy Point"),
        Cell::new("Amount $"),
        Cell::new("Coin"),
    ]));

    for i in 0..10 {
        table.add_row(Row::new(vec![
            Cell::new(&percentages[i].to_string()),
            Cell::new(&buy_point[i].to_string()),
            Cell::new(&amount_dollar[i].to_string()),
            Cell::new(&cost[i].to_string()),
        ]));
    }
    table.printstd();
}

fn generate_buy_table_json(percentages: Vec<f64>, buy_point: Vec<f64>, amount_dollar: Vec<f64>, cost: Vec<f64>) -> String {
    let table_data: Vec<_> = percentages.iter().zip(buy_point.iter()).zip(amount_dollar.iter()).zip(cost.iter())
        .map(|(((p, b), a), c)| {
            json!({
                "Percentages": p,
                "Buy Point": b,
                "Amount $": a,
                "Coin": c
            })
        }).collect();
    
    serde_json::to_string(&table_data).unwrap()
}

fn generate_sell_table_json(percentages: Vec<f64>, di: Vec<f64>, pa: Vec<f64>, final_price: Vec<f64>) -> String {
    let table_data: Vec<_> = percentages.iter().zip(di.iter()).zip(pa.iter()).zip(final_price.iter())
        .map(|(((p, d), a), f)| {
            json!({
                "Percentages": p,
                "Sell Point": d,
                "Coin": a,
                "Gain": f
            })
        }).collect();
    
    serde_json::to_string(&table_data).unwrap()
}

fn insert_into_database(strategy_type: &str, percentages: Vec<f64>, prices: Vec<f64>, amounts: Vec<f64>, total_cost: f64) {
    let url = "mysql://root:@localhost:3306/your_database";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    conn.query_drop(r"CREATE TABLE IF NOT EXISTS StrategyPoints (
                        id INT AUTO_INCREMENT PRIMARY KEY,
                        strategy_type ENUM('buy', 'sell'),
                        strike_price DOUBLE,
                        coin_amount DOUBLE,
                        total_cost DOUBLE,
                        timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
                    );").unwrap();

    for i in 0..percentages.len() {
        let params = params!{
            "strategy_type" => strategy_type,
            "strike_price" => prices[i],
            "coin_amount" => amounts[i],
            "total_cost" => total_cost
        };

        conn.exec_drop(r"INSERT INTO StrategyPoints (strategy_type, strike_price, coin_amount, total_cost)
                        VALUES (:strategy_type, :strike_price, :coin_amount, :total_cost)", params).unwrap();
    }
}

fn main() {
    let matches = App::new("Generate buy or sell table")
        .arg(Arg::with_name("buy")
            .long("buy")
            .takes_value(true)
            .multiple(true)
            .number_of_values(4))
        .arg(Arg::with_name("sell")
            .long("sell")
            .takes_value(true)
            .multiple(true)
            .number_of_values(4))
        .arg(Arg::with_name("output")
            .long("output")
            .takes_value(true)
            .possible_values(&["db", "table", "json"])
            .required(true))
        .get_matches();

    let output_type = matches.value_of("output").unwrap();


    if let Some(buy_values) = matches.values_of("buy") {
        let args: Vec<f64> = buy_values.map(|s| s.parse().unwrap()).collect();
        let (percentages, buy_point, amount_dollar, cost) = calculate_buy_data(args[0], args[1], args[2], args[3]);
        
        match output_type {
            "db" => {
                let total_cost: f64 = cost.iter().sum();
                insert_into_database("buy", percentages.clone(), buy_point.clone(), amount_dollar.clone(), total_cost);
            },
            "json" => {
                let json_output = generate_buy_table_json(percentages, buy_point, amount_dollar, cost);
                println!("{}", json_output);
            },
            _ => {
                generate_buy_table(percentages, buy_point, amount_dollar, cost);
            }
        }
    } else if let Some(sell_values) = matches.values_of("sell") {
        let args: Vec<f64> = sell_values.map(|s| s.parse().unwrap()).collect();
        let (percentages, di, pa, final_price) = calculate_sell_data(args[0], args[1], args[2], args[3]);

        match output_type {
            "db" => {
                let total_gain: f64 = final_price.iter().sum();
                insert_into_database("sell", percentages.clone(), di.clone(), pa.clone(), total_gain);
            },
            "json" => {
                let json_output = generate_sell_table_json(percentages, di, pa, final_price);
                println!("{}", json_output);
            },
            _ => {
                generate_sell_table(percentages, di, pa, final_price);
            }
        }
    }
}

