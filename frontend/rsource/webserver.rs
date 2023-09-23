use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use tera::{Tera, Context};
use serde::Deserialize;
use std::process::Command;
use actix_web::web::Data;
use actix_files as fs;

#[derive(Debug, Deserialize)]
pub struct AlgoOperation {
    high_percent: f32,
    low_percent: f32,
    start_price: String,
    units: String,
    operation_type: String,
}

#[derive(Debug, Deserialize)]
struct BuyResult {
    #[serde(rename = "Percentages")]
    percentages: Option<f64>,
    #[serde(rename = "Buy Point")]
    buy_point: Option<f64>,
    #[serde(rename = "Coin")]
    coin: Option<f64>,
    #[serde(rename = "Amount $")]
    amount_dollar: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct SellResult {
    #[serde(rename = "Percentages")]
    percentages: Option<f64>,
    #[serde(rename = "Sell Point")]
    sell_point: Option<f64>,
    #[serde(rename = "Coin")]
    coin: Option<f64>,
    #[serde(rename = "Gain")]
    gain: Option<f64>,
}

// Handler for the index route
async fn index(tera: web::Data<Tera>) -> impl Responder {
    let context = Context::new();
    let rendered = tera.render("index.html.tera", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}

// Handler for running the algorithm operation
async fn run_algo_operation(info: web::Form<AlgoOperation>) -> impl Responder {
    // Construct and execute the external command
    let output = Command::new("./al_go2")
        .arg(format!("--{}", info.operation_type))
        .arg((info.high_percent / 100.0).to_string())
        .arg((info.low_percent / 100.0).to_string())
        .arg(&info.start_price)
        .arg(&info.units)
        .arg("--output")
        .arg("json")
        .output()
        .expect("Failed to execute command");

    // Convert the output to a UTF-8 string and parse it as JSON
    let output_str = String::from_utf8(output.stdout).unwrap();

    // Generate HTML table based on the operation type
    let table_html = if info.operation_type == "buy" {
        let output_json: Vec<BuyResult> = serde_json::from_str(&output_str).unwrap();
        generate_buy_table_from_output(output_json) 
    } else {
        let output_json: Vec<SellResult> = serde_json::from_str(&output_str).unwrap();
        generate_sell_table_from_output(output_json) 
    };

    HttpResponse::Ok().body(table_html)
}

fn optional_f64_to_string(opt: Option<f64>) -> String {
    opt.map_or("N/A".to_string(), |v| v.to_string())
}

fn generate_buy_table_from_output(output: Vec<BuyResult>) -> String { 
    let mut table_html = String::from("<table><thead><tr>");

    // Add the headers for buy operations
    let headers = ["percentages", "buy_point", "coin", "amount_dollar"];
    for header in &headers {
        table_html.push_str(&format!("<th>{}</th>", header));
    }

    table_html.push_str("</tr></thead><tbody>");

    // Add the rows
    for row in output.iter() {
        table_html.push_str("<tr>");
        table_html.push_str(&format!("<td>{}</td>", optional_f64_to_string(row.percentages)));
        table_html.push_str(&format!("<td>{}</td>", optional_f64_to_string(row.buy_point)));
        table_html.push_str(&format!("<td>{}</td>", optional_f64_to_string(row.coin)));
        table_html.push_str(&format!("<td>{}</td>", optional_f64_to_string(row.amount_dollar)));
        table_html.push_str("</tr>");
    }

    table_html.push_str("</tbody></table>");
    table_html
}

fn generate_sell_table_from_output(output: Vec<SellResult>) -> String { 
    let mut table_html = String::from("<table><thead><tr>");

    // Add the headers for sell operations
    let headers = ["percentages", "sell_point", "coin", "gain"];
    for header in &headers {
        table_html.push_str(&format!("<th>{}</th>", header));
    }

    table_html.push_str("</tr></thead><tbody>");

    // Add the rows
    for row in output.iter() {
        table_html.push_str("<tr>");
        table_html.push_str(&format!("<td>{}</td>", optional_f64_to_string(row.percentages)));
        table_html.push_str(&format!("<td>{}</td>", optional_f64_to_string(row.sell_point)));
        table_html.push_str(&format!("<td>{}</td>", optional_f64_to_string(row.coin)));
        table_html.push_str(&format!("<td>{}</td>", optional_f64_to_string(row.gain)));
        table_html.push_str("</tr>");
    }

    table_html.push_str("</tbody></table>");
    table_html
}


// Handler for saving to the database
async fn save_to_db(info: web::Form<AlgoOperation>) -> impl Responder {
    // Construct and execute the external command for saving to the database
    let status = Command::new("./al_go2")
        .arg(format!("--{}", info.operation_type))
        .arg((info.high_percent / 100.0).to_string())
        .arg((info.low_percent / 100.0).to_string())
        .arg(&info.start_price)
        .arg(&info.units)
        .arg("--output")
        .arg("db")
        .status()
        .expect("Failed to execute command");

    // Check the execution status
    if status.success() {
        HttpResponse::Ok().body("Data saved to the database.")
    } else {
        HttpResponse::InternalServerError().body("Failed to save data.")
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new("templates/**/*").expect("Failed to initialize Tera");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(tera.clone()))
            .service(fs::Files::new("/static", "./static"))  // Serving static files
            .route("/", web::get().to(index))
            .route("/run-algo-operation", web::post().to(run_algo_operation))
            .route("/save-to-db", web::post().to(save_to_db))
    })
    .bind("127.0.0.1:5242")?
    .run()
    .await
}

