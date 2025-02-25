use async_graphql::{Object, Schema, SimpleObject};
use lazy_static::lazy_static;
use std::sync::Mutex;
// use tch::{nn, Device, Kind, Tensor};
use validator::Validate;

// モデルの再定義
lazy_static! {
    static ref MODEL: Mutex<Option<TestModel>> = Mutex::new(None);
}

// より単純なモデル構造を定義
struct TestModel {
    // 推論に使用する単純な係数
    multiplier: f64,
}

impl TestModel {
    // 新しいモデルを作成
    fn new(multiplier: f64) -> Self {
        TestModel { multiplier }
    }

    // シンプルな推論関数
    fn forward(&self, input: f64) -> f64 {
        input * self.multiplier
    }
}

// GraphQLのQuery実装
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    // 既存のpostsクエリは保持
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
        Ok(posts)
    }

    // 推論関数を単純化
    async fn predict(&self, input: String) -> String {
        // モデルが初期化されているか確認
        let model_guard = MODEL.lock().unwrap();

        match &*model_guard {
            Some(model) => {
                // 入力値（単純に文字列の長さを使用）
                let input_value = input.len() as f64;

                // 単純なモデルで計算
                let result = model.forward(input_value);

                format!(
                    "Input: '{}' (length: {}), Prediction: {:.4}",
                    input, input_value, result
                )
            }
            None => "Model not loaded. Please initialize the model first.".to_string(),
        }
    }

    // モデル初期化用の追加クエリ
    async fn initialize_model(&self, multiplier: Option<f64>) -> String {
        let mut model_guard = MODEL.lock().unwrap();

        // デフォルト値または指定された値で初期化
        let mult = multiplier.unwrap_or(2.5);
        *model_guard = Some(TestModel::new(mult));

        format!("Model initialized with multiplier: {:.4}", mult)
    }

    // モデル状態確認用の追加クエリ
    async fn model_status(&self) -> String {
        let model_guard = MODEL.lock().unwrap();

        match &*model_guard {
            Some(model) => format!("Model is loaded with multiplier: {:.4}", model.multiplier),
            None => "Model is not loaded".to_string(),
        }
    }
}

// Posts構造体はそのまま保持
#[derive(SimpleObject, Validate)]
struct Posts {
    id: i32,
    title: String,
    body: String,
}

// モデルのロード関数も単純化
pub fn load_model() {
    let mut model_guard = MODEL.lock().unwrap();
    *model_guard = Some(TestModel::new(2.5)); // デフォルト係数2.5でモデル初期化
    println!("Default model loaded with multiplier 2.5");
}

// GraphQLスキーマ作成
pub type BlogSchema =
    Schema<QueryRoot, async_graphql::EmptyMutation, async_graphql::EmptySubscription>;

pub fn create_schema() -> BlogSchema {
    // スキーマ生成前にモデルを初期化
    load_model();

    Schema::build(
        QueryRoot,
        async_graphql::EmptyMutation,
        async_graphql::EmptySubscription,
    )
    .finish()
}
