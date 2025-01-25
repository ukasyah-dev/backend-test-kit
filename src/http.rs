use std::str::FromStr;

use reqwest::{Client, Method};
use serde_json::Value;

pub struct TestSuite {
    pub method: String,
    pub url: String,
    pub test_cases: Vec<TestCase>,
}

pub struct TestCase {
    pub json: Option<Value>,
    pub expect_status: fn(reqwest::StatusCode),
    pub expect_result: fn(Value),
}

impl TestSuite {
    pub async fn run(self) {
        let client = Client::new();
        let method = Method::from_str(&self.method).unwrap_or(Method::GET);

        for test_case in self.test_cases {
            let request = client
                .request(method.clone(), self.url.clone())
                .json(&test_case.json)
                .send();

            let response = request.await.unwrap();
            let status = response.status();
            let result = response.json::<Value>().await.unwrap();

            (test_case.expect_status)(status);
            (test_case.expect_result)(result);
        }
    }
}
