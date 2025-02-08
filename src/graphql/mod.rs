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
