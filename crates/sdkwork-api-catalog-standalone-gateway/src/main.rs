use sdkwork_api_catalog_assembly::assemble_api_router_from_env;
use sdkwork_iam_web_adapter::{
    build_web_framework_builder, iam_web_request_context_resolver_from_env,
};
use sdkwork_web_bootstrap::{infra_public_path_prefixes, ComposedApiAssembly};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let assembly = assemble_api_router_from_env()
        .await
        .expect("catalog API assembly failed");
    let framework = build_web_framework_builder(
        iam_web_request_context_resolver_from_env().await,
        assembly.route_manifest.clone(),
        infra_public_path_prefixes(),
    );
    let app = ComposedApiAssembly::try_compose("SDKWork Catalog API", vec![assembly])
        .expect("catalog API composition failed")
        .into_hosted(framework)
        .router
        .layer(sdkwork_web_bootstrap::application_cors_layer_from_env(
            &["SDKWORK_CATALOG_ENVIRONMENT"],
            &["SDKWORK_CORS_ALLOWED_ORIGINS"],
        ));
    let addr = std::env::var("CATALOG_API_BIND").unwrap_or_else(|_| "0.0.0.0:18099".to_owned());
    let listener = tokio::net::TcpListener::bind(&addr).await.expect("bind");
    axum::serve(listener, app).await.expect("serve");
}
