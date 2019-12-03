use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use log::*;

use async_trait::async_trait;
use kevlar::*;
use thirtyfour::remote::command::{Command, SessionId};
use thirtyfour::remote::connection_async::RemoteConnectionAsync;
use thirtyfour::remote::connection_common::CommandError;

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

        self.webtest().await.map_err(|e| TestEvent::new(TestStatus::Failed).with_description(&format!("CommandError: {:?}", e)))?;
        Ok(())
    }
}

impl MyTest {
    async fn webtest(&self) -> Result<(), CommandError> {
        let conn = RemoteConnectionAsync::new("http://localhost:4444/wd/hub")?;
        let caps = serde_json::json!({
            "browserName": "chrome",
            "version": "",
            "platform": "any"
        });

        info!("Launching new browser session");
        let v = conn.execute(Command::NewSession(caps)).await?;
        let session_id = SessionId::from(v["sessionId"].as_str().unwrap());
        info!("Navigate to Google");
        conn.execute(Command::NavigateTo(
            &session_id,
            "https://google.com.au".to_owned(),
        )).await?;
        thread::sleep(Duration::new(3, 0));
        info!("Closing browser");
        conn.execute(Command::DeleteSession(&session_id)).await?;

        Ok(())
    }
}
