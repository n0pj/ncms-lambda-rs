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
mod utils;

// use aws_config::meta::region::RegionProviderChain;
// use crate::models::category::Category;
use chrono::Utc;
// use diesel::prelude::*;
use dotenv::dotenv;
use http::response::NcmsValueErrors;
use juniper::execute_sync;
use juniper::http::GraphQLRequest;
use juniper::{execute, EmptySubscription, RootNode, Variables};
use ncms_core::{
    db::mysql::establish_connection,
    errors::http::{ValueError, ValueErrors},
    http::graphql::client::graphiql_source,
    Header,
};
use ncms_lambda_core::http::request::{find_param, format_query, get_query};
use seeders::*;

use lambda_runtime::{handler_fn, Context};

use mutation_roots::MutationRoot;
use query_roots::QueryRoot;
use regex::Regex;
use serde::Deserialize;
use serde_json::Value;
use std::env;

use ncms_lambda_core::mysql::Migration;

use actix_cors::Cors;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;
type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;

/// GraphQL 実行
async fn execute_query(header: Header, query: Value) -> Result<Value, Value> {
    let query = match format_query(&query) {
        Ok(query) => query,
        Err(e) => {
            return Ok(ValueErrors::new(vec![ValueError {
                message: e.to_string(),
                ..Default::default()
            }])
            .to_value())
        }
    };
    let schema = Schema::new(
        QueryRoot {
            header: header.clone(),
        },
        MutationRoot { header },
        EmptySubscription::new(),
    );
    let result = execute(&query, None, &schema, &Variables::default(), &()).await;
    let result = match result {
        Ok((result, _)) => serde_json::to_value(result).expect("fatal error"),
        Err(err) => {
            println!("{}", err);

            let msg = err.to_string();
            let field_error = ValueError {
                property: Some(msg),
                ..Default::default()
            };
            let field_errors = ValueErrors::new(vec![field_error]);

            field_errors.to_value()
        }
    };

    Ok(result)
}

async fn local_handler(event: Value) -> Result<Value, Value> {
    let query = match get_query(&event) {
        Ok(query) => query,
        Err(e) => {
            return Ok(ValueErrors::new(vec![ValueError {
                message: e.to_string(),
                ..Default::default()
            }])
            .to_value())
        }
    };
    let headers = find_param(&event, "headers");
    let headers = match headers {
        Ok(headers) => headers,
        Err(e) => {
            return Ok(ValueErrors::new(vec![ValueError {
                message: e.to_string(),
                ..Default::default()
            }])
            .to_value())
        }
    };
    let authorization = match headers.get("authorization") {
        Some(authorization) => Some(authorization.to_string()),
        None => None,
    };
    let header = Header { authorization };
    let result = execute_query(header, query).await?;
    let _conn = establish_connection();

    println!("database connected");

    Ok(result)
}

async fn handler(event: Value, _: Context) -> Result<Value, Error> {
    let query = match get_query(&event) {
        Ok(result) => result,
        Err(e) => {
            return Ok(ValueErrors::new(vec![ValueError {
                message: e.to_string(),
                ..Default::default()
            }])
            .to_value())
        }
    };
    let headers = find_param(&event, "headers");
    let headers = match headers {
        Ok(headers) => headers,
        Err(e) => {
            return Ok(ValueErrors::new(vec![ValueError {
                message: e.to_string(),
                ..Default::default()
            }])
            .to_value())
        }
    };
    let authorization = match headers.get("authorization") {
        Some(authorization) => Some(authorization.to_string()),
        None => None,
    };
    let header = Header { authorization };
    let result = execute_query(header, query).await;
    let _conn = establish_connection();

    println!("database connected");

    match result {
        Ok(value) => Ok(value),
        Err(_) => Ok(ValueErrors::new(vec![ValueError {
            message: "fatal error".to_string(),
            ..Default::default()
        }])
        .to_value()),
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
    request: HttpRequest,
    // ) -> Result<HttpResponse, actix_web::Error> {
) -> Result<HttpResponse, actix_web::Error> {
    let request_head = request.head();
    let headers = request_head.headers();

    println!("----------");
    // println!("graphql_object: {:?}", graphql_request);
    // println!("query    : {:?}", query);
    // println!("form_data: {:?}", form_data);
    // println!("headers  : {:?}", headers);
    let authorization = headers.get("Authorization");
    let authorization = match authorization {
        Some(authorization) => Some(authorization.to_str().unwrap().to_owned()),
        None => None,
    };
    let header = Header { authorization };

    // GraphQLRequest の場合は graphiql のアクセスなのでこちらを使用する
    if let Some(data) = graphql_request {
        println!("graphiql requested");
        let result = web::block(move || {
            let schema = Schema::new(
                QueryRoot {
                    header: header.clone(),
                },
                MutationRoot {
                    header: header.clone(),
                },
                EmptySubscription::new(),
            );
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
            let schema = Schema::new(
                QueryRoot {
                    header: header.clone(),
                },
                MutationRoot {
                    header: header.clone(),
                },
                EmptySubscription::new(),
            );
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
    let html = graphiql_source("/");

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
