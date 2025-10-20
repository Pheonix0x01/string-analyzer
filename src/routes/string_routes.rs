use actix_web::{web, HttpResponse, http::StatusCode};
use crate::models::string::{StringInput, StoredString, FilteredResponse, NaturalLanguageResponse, InterpretedQuery};
use crate::services::analyzer::analyze_string;
use crate::services::storage;
use crate::services::nlp_parser::parse_natural_language;
use crate::utils::filters::{parse_filters, QueryParams};
use crate::utils::response::{success_response, error_response};
use chrono::Utc;
use serde::Deserialize;

pub async fn create_string(body: Result<web::Json<StringInput>, actix_web::Error>) -> HttpResponse {
    let body = match body {
        Ok(b) => b,
        Err(_) => return error_response("Invalid request body or missing value field", StatusCode::BAD_REQUEST),
    };
    
    let value = &body.value;
    
    if value.is_empty() {
        return error_response("value field cannot be empty", StatusCode::BAD_REQUEST);
    }
    
    let properties = analyze_string(value);
    let hash = properties.sha256_hash.clone();
    
    if storage::string_exists(&hash) {
        return error_response("String already exists", StatusCode::CONFLICT);
    }
    
    let stored = StoredString {
        id: hash,
        value: value.clone(),
        properties,
        created_at: Utc::now(),
    };
    
    match storage::save_string(stored.clone()) {
        Ok(_) => success_response(stored, StatusCode::CREATED),
        Err(e) => error_response(&e, StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_string(path: web::Path<String>) -> HttpResponse {
    let value = path.into_inner();
    
    match storage::get_string_by_value(&value) {
        Some(stored) => success_response(stored, StatusCode::OK),
        None => error_response("String not found", StatusCode::NOT_FOUND),
    }
}

pub async fn get_all_strings(query: web::Query<QueryParams>) -> HttpResponse {
    let filters = match parse_filters(query) {
        Ok(f) => f,
        Err(e) => return error_response(&e, StatusCode::BAD_REQUEST),
    };
    
    let data = storage::get_all_strings(filters.clone());
    let count = data.len();
    
    let response = FilteredResponse {
        data,
        count,
        filters_applied: filters,
    };
    
    success_response(response, StatusCode::OK)
}

#[derive(Deserialize)]
pub struct NLQuery {
    query: String,
}

pub async fn filter_by_natural_language(query: web::Query<NLQuery>) -> HttpResponse {
    let query_str = &query.query;
    
    let (filters, original) = match parse_natural_language(query_str) {
        Ok(result) => result,
        Err(e) => {
            if e.contains("Conflicting") {
                return error_response(&e, StatusCode::UNPROCESSABLE_ENTITY);
            }
            return error_response(&e, StatusCode::BAD_REQUEST);
        }
    };
    
    let data = storage::get_all_strings(filters.clone());
    let count = data.len();
    
    let response = NaturalLanguageResponse {
        data,
        count,
        interpreted_query: InterpretedQuery {
            original,
            parsed_filters: filters,
        },
    };
    
    success_response(response, StatusCode::OK)
}

pub async fn delete_string(path: web::Path<String>) -> HttpResponse {
    let value = path.into_inner();
    
    match storage::delete_string(&value) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => error_response("String not found", StatusCode::NOT_FOUND),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/strings")
            .route("", web::post().to(create_string))
            .route("", web::get().to(get_all_strings))
            .route("/filter-by-natural-language", web::get().to(filter_by_natural_language))
            .route("/{value}", web::get().to(get_string))
            .route("/{value}", web::delete().to(delete_string))
    );
}