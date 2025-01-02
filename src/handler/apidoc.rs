

use crate::common::*;
use axum::Router;
use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};
use utoipa::OpenApi;

// use utoipa_rapidoc::RapiDoc;
use utoipa_scalar::Scalar;
use utoipa_scalar::Servable;
use utoipauto::utoipauto;
// use utoipa_swagger_ui::SwaggerUi;

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    #[utoipauto(paths = "./src/handler, ./src/vo ")]
    #[derive(OpenApi)]
    #[openapi(
        // nest(
        //     (path = "/admin", api = user_handler::TodoApi)
        // ),
        components(schemas(Response<String>)),
        // tags(
        //     (name = "user", description = "User API"),
        // ),
        modifiers(&SecurityAddon),
        servers(
            (url = "http://localhost:8000", description = "dev"),
            (url = "http://localhost:8000/admin", description = "dev/admin")
        ),
    )]
    struct ApiDoc;

    struct SecurityAddon;

    impl utoipa::Modify for SecurityAddon {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            if let Some(components) = openapi.components.as_mut() {
                components.add_security_scheme(
                    "token",
                    SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
                )
            }
        }
    }
    Router::new()
        // .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        // .merge(RapiDoc::with_openapi("/api-docs/openapi.json", ApiDoc::openapi()).path("/rapidoc"))
        .merge(Scalar::with_url("/scalar", ApiDoc::openapi()))
}
