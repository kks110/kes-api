
extern crate console_error_panic_hook;

use std::collections::HashMap;
use serde_json::Value;
use worker::*;
use worker::wasm_bindgen::JsValue;
use crate::models::Slab;

pub fn authorised(req: &Request, ctx: &RouteContext<()>) -> bool {
    if let Ok(Some(key)) = req.headers().get("Authorization") {
        if let Ok(secret) = ctx.secret("AUTH_KEY") {
            return key == secret.to_string();
        }
    }
    false
}

pub async fn handle_get(ctx: RouteContext<()>) -> Result<Response> {
    let d1 = ctx.env.d1("DB")?;
    let statement = d1.prepare("SELECT * FROM Slabs");
    let results: Result<Vec<Slab>> = statement.all().await?.results();

    match results {
        Ok(res) => Response::from_json(&res),
        Err(e) => Response::error(format!("Database error: {:?}", e), 500)
    }
}

pub async fn handle_post(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let slabs_to_add = req.json::<Vec<Slab>>().await?;
    let d1 = ctx.env.d1("DB")?;

    for slab in slabs_to_add {
        if let Err(e) = process_slab(&d1, slab).await {
            return Response::error(format!("Database error: {:?}", e), 500);
        }
    }

    Response::ok("Data inserted successfully")
}

async fn process_slab(d1: &D1Database, slab: Slab) -> Result<()> {
    if slab_exists(d1, &slab.cert_number).await? {
        update_slab(d1, slab).await
    } else {
        insert_slab(d1, slab).await
    }
}

async fn slab_exists(d1: &D1Database, cert_number: &str) -> Result<bool> {
    let statement = d1.prepare("SELECT 1 FROM Slabs WHERE cert_number = ? LIMIT 1;");
    let query = statement.bind(&[cert_number.into()])?;
    let result: Result<Vec<HashMap<String, Value>>> = query.all().await?.results();

    Ok(!result?.is_empty())
}

async fn update_slab(d1: &D1Database, slab: Slab) -> Result<()> {
    let statement = d1.prepare(
        "UPDATE Slabs SET owner = ?, for_sale = ?, card_name = ?, card_number = ?, set_name = ?, tcg = ?, language = ?, cost = ?, grading_company = ?, grade = ?, price = ?, sold = ?, sold_value = ?, date_sold = ?, notes = ?, image_url = ?, ace_label_url = ? WHERE cert_number = ?"
    );
    let query = statement.bind(&[
        slab.owner.into(),
        slab.for_sale.into(),
        slab.card_name.into(),
        slab.card_number.into(),
        slab.set_name.into(),
        slab.tcg.into(),
        slab.language.into(),
        slab.cost.into(),
        slab.grading_company.into(),
        slab.grade.into(),
        slab.price.map_or(JsValue::NULL, |v| v.into()),
        slab.sold.into(),
        slab.sold_value.map_or(JsValue::NULL, |v| v.into()),
        slab.date_sold.map_or(JsValue::NULL, |v| v.into()),
        slab.notes.into(),
        slab.image_url.unwrap_or_else(|| "https://limitlesstcg.com/images/image_not_available.png".to_string()).into(),
        slab.ace_label_url.map_or(JsValue::NULL, |v| v.into()),
        slab.cert_number.into(),
    ])?;
    let _: Vec<Slab> = query.all().await?.results()?;
    console_log!("Updated slab");

    Ok(())
}

async fn insert_slab(d1: &D1Database, slab: Slab) -> Result<()> {
    let statement = d1.prepare(
        "INSERT INTO Slabs (owner, for_sale, card_name, card_number, set_name, tcg, language, cost, grading_company, grade, cert_number, price, sold, sold_value, date_sold, notes, image_url, ace_label_url) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    );
    let query = statement.bind(&[
        slab.owner.into(),
        slab.for_sale.into(),
        slab.card_name.into(),
        slab.card_number.into(),
        slab.set_name.into(),
        slab.tcg.into(),
        slab.language.into(),
        slab.cost.into(),
        slab.grading_company.into(),
        slab.grade.into(),
        slab.cert_number.into(),
        slab.price.map_or(JsValue::NULL, |v| v.into()),
        slab.sold.into(),
        slab.sold_value.map_or(JsValue::NULL, |v| v.into()),
        slab.date_sold.map_or(JsValue::NULL, |v| v.into()),
        slab.notes.into(),
        slab.image_url.unwrap_or_else(|| "https://limitlesstcg.com/images/image_not_available.png".to_string()).into(),
        slab.ace_label_url.map_or(JsValue::NULL, |v| v.into()),
    ])?;
    let _: Vec<Slab> = query.all().await?.results()?;
    console_log!("Created slab");

    Ok(())
}
