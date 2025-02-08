use actix_graphql::graphql;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

async fn graphql_handler(
    schema: web::Data<graphql::BlogSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

// GraphQL Playground ハンドラー
async fn graphql_playground() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = graphql::create_schema();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .route("/graphql", web::post().to(graphql_handler))
            .route("/graphql", web::get().to(graphql_playground)) // 追加
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
