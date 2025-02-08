# Pure Rust Dev Blog (PRDV) - Backend Setup

## 🚀 概要

本プロジェクトは **Rust (Actix-web + async-graphql)** を使用して構築するブログシステムのバックエンドです。

- **フレームワーク:** Actix-web
- **GraphQL:** async-graphql
- **認証:** (未実装)
- **データベース:** Supabase / PostgreSQL (後日導入)
- **バリデーション:** validator

本 README では **バックエンド開発の初期セットアップとバリデーションの実装** について説明します。

---

## ✅ フォルダ構成

```sh
src/
 ├── graphql/
 │   ├── mod.rs
 ├── validation/
 │   ├── mod.rs
 ├── lib.rs
 ├── main.rs
```

---

## **🔹 1️⃣ **``** に必要な依存関係を追加**

```toml
[dependencies]
actix-web = "4"
async-graphql = "6"
async-graphql-actix-web = "6"
validator = { version = "0.16", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
thiserror = "1.0"
```

---

## **🔹 2️⃣ **``**（GraphQL Resolver）**

```rust
use crate::validation;
use async_graphql::{Object, Schema, SimpleObject};
use validator::Validate;

#[derive(SimpleObject, Validate)]
struct Posts {
    #[validate(custom = "validation::validate_id")]
    id: i32,
    title: String,
    body: String,
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn posts(&self) -> Result<Vec<Posts>, String> {
        let posts = vec![
            Posts {
                id: 1,
                title: "First Post".to_string(),
                body: "This is the body of the first post".to_string(),
            },
            Posts {
                id: 2,
                title: "Second Post".to_string(),
                body: "This is the body of the second post".to_string(),
            },
        ];

        for post in &posts {
            if let Err(err) = post.validate() {
                return Err(format!("Validation Error: {:?}", err));
            }
        }
        Ok(posts)
    }
}

pub type BlogSchema =
    Schema<QueryRoot, async_graphql::EmptyMutation, async_graphql::EmptySubscription>;

pub fn create_schema() -> BlogSchema {
    Schema::build(
        QueryRoot,
        async_graphql::EmptyMutation,
        async_graphql::EmptySubscription,
    )
    .finish()
}
```

✅ **GraphQL の **``** クエリで **``** のデータがエラーになる！**

---

## **🔹 3️⃣ **``**（バリデーション）**

```rust
use validator::ValidationError;

pub fn validate_id(id: i32) -> Result<(), ValidationError> {
    if id < 1 {
        return Err(ValidationError::new("id must be greater than 0"));
    }
    Ok(())
}
```

✅ **ID が **``** の場合にバリデーションエラーを発生させる！**

---

## **🔹 4️⃣ **``**（モジュール定義）**

```rust
pub mod graphql;
pub mod validation;
```

✅ **GraphQL と Validation のモジュールを公開！**

---

## **🔹 5️⃣ **``**（Actix-web サーバー）**

```rust
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
            .route("/graphql", web::get().to(graphql_playground))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

✅ **GraphQL API (**``**) を提供し、GraphQL Playground (**``**) を起動！**

---

## **🚀 動作確認**

📌 **サーバー起動**

```sh
cargo run
```

📌 **ブラウザで **``** にアクセス** ➡ **GraphQL Playground が開く！**

📌 **GraphQL クエリテスト**

```graphql
{
  posts {
    id
    title
    body
  }
}
```

📌 **期待されるレスポンス**

```json
{
  "data": {
    "posts": [
      { "id": 1, "title": "First Post", "body": "This is the body of the first post" },
      { "id": 2, "title": "Second Post", "body": "This is the body of the second post" }
    ]
  }
}
```

✅ **GraphQL の動作確認完了！**

