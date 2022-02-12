#[macro_use]
extern crate juniper;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate diesel;

mod constants;
mod errors;
mod http;
mod models;
mod mutation_roots;
mod query_roots;
mod schema;
mod seeders;
mod subscription_roots;

// use juniper::http::graphiql::graphiql_source;
// use aws_config::meta::region::RegionProviderChain;
// use crate::models::category::Category;
use chrono::Utc;
// use diesel::prelude::*;
use dotenv::dotenv;
use http::response::NcmsValueErrors;
use juniper::execute_sync;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use juniper::{execute, EmptySubscription, RootNode, Variables};
use ncms_core::errors::http::{ValueError, ValueErrors};
use seeders::*;

use lambda_runtime::{handler_fn, Context};

use mutation_roots::MutationRoot;
use query_roots::QueryRoot;
use regex::Regex;
use serde::Deserialize;
use serde_json::Value;
use std::env;

// use ncms_core::db::mysql::establish_connection;
use ncms_lambda_core::mysql::Migration;

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer};

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;
type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;

/// Lambda に GET パラメーターを渡した場合 queryStringParameters に入る。
/// そこから GET パラメーターを取得する
fn get_query(event: Value) -> Result<Value, Value> {
    // println!("{:?}", event);

    // Lambda では queryStringParameters の中に GET パラメーターが入る
    let query_string_parameters = match event.get("queryStringParameters") {
        Some(event) => event.clone(),
        None => {
            let field_errors = NcmsValueErrors::new(
                errors::validation::CANNOT_FIND_QUERY_STRING_PARAMETERS.message,
            );
            return Err(serde_json::to_value(field_errors).expect("fatal error"));
        }
    };

    // query に GraphQL query を入れるため、ここから取得
    match query_string_parameters.get("query") {
        Some(query) => Ok(query.clone()),
        None => {
            let field_errors = NcmsValueErrors::new(errors::validation::CANNOT_FIND_QUERY.message);
            return Err(serde_json::to_value(field_errors).expect("fatal error"));
        }
    }
}

/// GET で送られてきたものは "" で囲まれてしまうため、 "" を解除する
/// "query { humans(i: 0) { name } }" -> query { humans(i: 0) { name } }
fn format_query(query: Value) -> Result<String, Value> {
    let query = match serde_json::to_string(&query) {
        Ok(result) => result,
        Err(_) => return Err(NcmsValueErrors::new("fatal error").to_value()),
    };
    // let query = query.to_string();
    let re = Regex::new(r#""(.*)""#).unwrap();
    let caps = re.captures(&query).unwrap();
    let query = caps.get(1).map_or("", |m| m.as_str());

    Ok(query.to_owned())
}

/// GraphQL 実行
async fn execute_query(query: Value) -> Result<Value, Value> {
    let query = format_query(query)?;

    let schema = Schema::new(QueryRoot, MutationRoot, EmptySubscription::new());
    // println!("query1: {}", query2);
    // let query2 = "query { humans(i: 0) { name } }";
    // println!("query2: {}", query2);

    let result = execute(&query, None, &schema, &Variables::default(), &()).await;

    // println!("{:?}", result);

    let result = match result {
        Ok((result, _)) => serde_json::to_value(result).expect("fatal error"),
        Err(err) => {
            println!("{}", err);
            let msg = err.to_string();
            NcmsValueErrors::new(&msg).to_value()
        }
    };

    Ok(result)
}

async fn local_handler(event: Value) -> Result<Value, Value> {
    // let query = get_query(event)?;
    // let query = serde_json::to_string(&query).unwrap();
    // match get_query(event) {
    //     Ok(query) => Ok(query),
    //     Err(err) => Ok(err),
    // }
    let query = get_query(event)?;
    let result = execute_query(query).await?;

    // let conn = establish_connection();

    println!("database connected");

    // use schema::category::dsl::category as dsl_category;

    // let category = dsl_category.limit(5).load::<Category>(&conn).expect("err");
    // println!(
    //     "{} {} {} {} {}",
    //     category[0].uuid,
    //     category[0].name,
    //     category[0].slug,
    //     category[0].created_at,
    //     category[0].updated_at
    // );

    Ok(result)
}

async fn handler(event: Value, _: Context) -> Result<Value, Error> {
    println!("{:?}", event);
    let query = match get_query(event) {
        Ok(result) => result,
        Err(result) => return Ok(result),
    };
    let result = execute_query(query).await;
    // let conn = establish_connection();

    // println!("database connected");

    // use schema::category::dsl::category as dsl_category;

    // let category = dsl_category.limit(5).load::<Category>(&conn).expect("err");
    // println!(
    //     "{} {} {} {} {}",
    //     category[0].uuid,
    //     category[0].name,
    //     category[0].slug,
    //     category[0].created_at,
    //     category[0].updated_at
    // );

    // Ok(result.unwrap())
    match result {
        Ok(value) => Ok(value),
        Err(_) => panic!(),
    }
}

#[derive(Debug, Deserialize, Clone)]
struct Formdata {
    query: String,
}

/// ローカルから GraphQLRequest が来た場合に使用する GraphQL Executor
async fn graphql(
    graphql_request: Option<web::Json<GraphQLRequest>>,
    // query: Option<web::Query<Info>>,
    form_data: Option<web::Form<Formdata>>,
    // req: HttpRequest,
    // ) -> Result<HttpResponse, actix_web::Error> {
) -> Result<HttpResponse, actix_web::Error> {
    // let req = req.head();
    // let headers = req.headers();

    println!("----------");
    // println!("graphql_object: {:?}", graphql_request);
    // println!("query    : {:?}", query);
    // println!("form_data: {:?}", form_data);
    // println!("headers  : {:?}", headers);

    // GraphQLRequest の場合は graphiql のアクセスなのでこちらを使用する
    if let Some(data) = graphql_request {
        println!("graphiql requested");
        let result = web::block(move || {
            let schema = Schema::new(QueryRoot, MutationRoot, EmptySubscription::new());
            let result = data.execute_sync(&schema, &());
            Ok::<_, serde_json::error::Error>(serde_json::to_string(&result)?)
        })
        .await?;

        // 早期リターン
        return Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(result));
    }

    // そうでない場合 ( 普通の POST )
    let result = match form_data {
        Some(form_data) => {
            println!("POST requested");
            let schema = Schema::new(QueryRoot, MutationRoot, EmptySubscription::new());
            let result = execute_sync(&form_data.query, None, &schema, &Variables::default(), &());

            let result = match result {
                // Ok((result, _)) => serde_json::to_value(result).expect("fatal error"),
                Ok((result, _)) => serde_json::to_string(&result).expect("fatal error"),
                Err(err) => {
                    println!("{}", err);

                    let msg = err.to_string();
                    let field_error = ValueError {
                        property: Some(msg),
                        ..Default::default()
                    };
                    let field_errors = ValueErrors::new(vec![field_error]);

                    println!("{:?}", field_errors);

                    // field_errors.to_value()
                    serde_json::to_string(&field_errors).unwrap()
                }
            };
            result
        }
        None => panic!(),
    };

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(result))
}

/// GraphQL クライアント
fn graphiql() -> HttpResponse {
    let html = graphiql_source("/", None);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[actix_web::main]
async fn run_actix_web_server() -> Result<(), Error> {
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["POST", "GET"])
                    .supports_credentials()
                    .max_age(3600),
            )
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
            .service(web::resource("/").route(web::post().to(graphql)))
    })
    .bind("ndock_rust:8888")?
    .run()
    .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    // env の MODE によって実行する関数を切り替える
    if let Ok(v) = env::var("MODE") {
        if v == "testing" {
        } else if v == "migrating" {
            let bucket = env::var("S3_MIGRATIONS_BUCKET").expect("env S3_MIGRATIONS_BUCKET error");
            let region = env::var("S3_MIGRATIONS_REGION").expect("env S3_MIGRATIONS_REGION error");
            let mysql = Migration::new(&bucket, &region);
            let _ = mysql.execute_up_migrations().await?;
        } else if v == "seeding" {
            // env の ENV によって実行する関数を切り替える
            if let Ok(v) = env::var("ENV") {
                if v == "main".to_owned() {
                    let _ = insert_testdatas();
                } else if v == "staging" {
                    let _ = insert_testdatas_on_staging();
                } else if v == "master" {
                    let _ = insert_testdatas_on_master();
                }
            }
        }
    } else {
        panic!("not set environment variable: MODE");
    };

    // env の ENV によって実行する関数を切り替える
    if let Ok(v) = env::var("ENV") {
        if v == "main".to_owned() {
            // ローカル向けの Lambda 関数を実行する
            let lambda_values =
                json!({ "queryStringParameters": json!({"query":"query { human { name } }"}) });
            let lambda_values = serde_json::to_value(lambda_values).unwrap();

            println!("{}", Utc::now());

            let res = local_handler(lambda_values).await;

            println!("{:?}", res);

            let _res = run_actix_web_server();
        } else if v == "staging" {
            lambda_runtime::run(handler_fn(handler)).await?
        } else if v == "master" {
            lambda_runtime::run(handler_fn(handler)).await?
        }
    } else {
        panic!("not set environment variable: ENV");
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn handler_handles() {
        let _event = Value::default();
        // assert_eq!(
        //     handler(event, Context::default())
        //         .await
        //         .expect("expected Ok(_) value"),
        //     event
        // )
    }
}
