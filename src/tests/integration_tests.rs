#[cfg(test)]
mod tests {
    use actix_web::{test, web, App, http::StatusCode};
    use crate::routes::string_routes;
    use crate::models::string::{StringInput, StoredString};
    use serde_json::json;

    #[actix_web::test]
    async fn test_create_string_success() {
        let app = test::init_service(
            App::new().configure(string_routes::configure)
        ).await;

        let payload = json!({
            "value": "test string"
        });

        let req = test::TestRequest::post()
            .uri("/strings")
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
    }

    #[actix_web::test]
    async fn test_create_string_duplicate() {
        let app = test::init_service(
            App::new().configure(string_routes::configure)
        ).await;

        let payload = json!({
            "value": "duplicate test"
        });

        let req1 = test::TestRequest::post()
            .uri("/strings")
            .set_json(&payload)
            .to_request();
        test::call_service(&app, req1).await;

        let req2 = test::TestRequest::post()
            .uri("/strings")
            .set_json(&payload)
            .to_request();
        let resp = test::call_service(&app, req2).await;
        
        assert_eq!(resp.status(), StatusCode::CONFLICT);
    }

    #[actix_web::test]
    async fn test_get_string_not_found() {
        let app = test::init_service(
            App::new().configure(string_routes::configure)
        ).await;

        let req = test::TestRequest::get()
            .uri("/strings/nonexistent")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_delete_string_success() {
        let app = test::init_service(
            App::new().configure(string_routes::configure)
        ).await;

        let payload = json!({
            "value": "to delete"
        });

        let req1 = test::TestRequest::post()
            .uri("/strings")
            .set_json(&payload)
            .to_request();
        test::call_service(&app, req1).await;

        let req2 = test::TestRequest::delete()
            .uri("/strings/to%20delete")
            .to_request();
        let resp = test::call_service(&app, req2).await;

        assert_eq!(resp.status(), StatusCode::NO_CONTENT);
    }

    #[actix_web::test]
    async fn test_filter_palindromes() {
        let app = test::init_service(
            App::new().configure(string_routes::configure)
        ).await;

        let req = test::TestRequest::get()
            .uri("/strings?is_palindrome=true")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_natural_language_query() {
        let app = test::init_service(
            App::new().configure(string_routes::configure)
        ).await;

        let req = test::TestRequest::get()
            .uri("/strings/filter-by-natural-language?query=all%20single%20word%20palindromic%20strings")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}