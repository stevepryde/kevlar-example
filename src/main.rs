use kevlar::*;
use std::path::PathBuf;
use async_trait::async_trait;
use log::*;

#[tokio::main]
async fn main() {
    let harness = TestHarness::new(
        "kevlar_example",
        ConfigType::File(PathBuf::from("./config.json")),
    );
    harness.run_async::<MyTest>().await;
}

#[derive(Default)]
struct MyTest;

#[async_trait]
impl AsyncTestCase for MyTest {
    async fn run_async(&mut self, _test_config: TestConfig, _test_result: &mut TestRecord) -> TestResult {
        info!("Do something interesting");
        Err(TestEvent::new(TestStatus::Failed).with_description("Something went wrong"))
    }
}
