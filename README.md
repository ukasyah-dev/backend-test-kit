# backend-test-kit

Provides a set of tools and helpers for testing backend services in Rust.

## Quickstart

Add `backend-test-kit` as a dev dependency:

```bash
cargo add --dev backend-test-kit
```

Here's a simple example of how to use it:

```rust
use std::vec;

use backend_test_kit::http::{TestCase, TestSuite};
use serde_json::json;

#[tokio::test]
async fn create_post() {
    let test_cases = vec![
        TestCase {
            json: Some(json!({
                "title": "foo",
                "body": "bar",
                "userId": 1,
            })),
            expect_status: |s| assert!(s.is_success()),
            expect_result: |r| {
                assert_eq!(r["id"], 101);
                assert_eq!(r["title"], "foo");
                assert_eq!(r["body"], "bar");
            },
        },
    ];

    let test_suite = TestSuite {
        method: "POST".to_string(),
        url: "https://jsonplaceholder.typicode.com/posts".to_string(),
        test_cases,
    };

    test_suite.run().await;
}
```
