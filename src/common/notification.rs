use crate::command_utils::CommandOutput;
use minio::s3::builders::ObjectContent;
use minio::s3::client::{Client, ClientBuilder};
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;
use minio::s3::multimap::Multimap;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct Notification {
    task_id: String,
    command_name: String,
    command_args: Vec<String>,
    status: i32,
    stdout: Option<String>,
    stderr: Option<String>,
}

pub fn send_notification(
    command_output: &CommandOutput,
    command_name: &str,
    command_args: &[&str],
) {
    let task_id = env::var("TK_TASK_ID").unwrap();
    // Create a notification object
    let notification = Notification {
        task_id: task_id.clone(),
        command_name: command_name.to_string(),
        command_args: command_args.iter().map(|s| s.to_string()).collect(),
        status: command_output.status.code().unwrap_or(0),
        stdout: command_output.stdout.clone(),
        stderr: command_output.stderr.clone(),
    };
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        // save to OSS
        if env::var("S3_BUCKET").is_ok() {
            save_oss(&notification).await.unwrap_or_else(|err| {
                eprintln!("Failed to save to OSS: {}", err);
            });
        }
        // Send the notification asynchronously
        if let Ok(nats_url) = env::var("NATS_URL") {
            send_nats_message(&nats_url, &notification)
                .await
                .unwrap_or_else(|err| {
                    eprintln!("Failed to send NATS message: {}", err);
                });
        }
    })
}

async fn send_nats_message(nats_url: &str, notification: &Notification) -> anyhow::Result<()> {
    if let Ok(client) = async_nats::connect(nats_url).await {
        client
            .publish(
                "task-keeper",
                serde_json::to_vec(notification).unwrap().into(),
            )
            .await?;
        client.flush().await?;
    }
    Ok(())
}

async fn save_oss(notification: &Notification) -> anyhow::Result<()> {
    let s3_bucket = env::var("S3_BUCKET")?;
    let object_name = &notification.task_id;
    if let Ok(minio_client) = create_oss_client() {
        let mut text = String::new();
        if let Some(stdout) = &notification.stdout {
            text.push_str(stdout);
        }
        if let Some(stderr) = &notification.stderr {
            text.push_str(stderr);
        }
        let content = ObjectContent::from(text);
        let mut user_metadata = Multimap::new();
        user_metadata.insert("status".to_string(), notification.status.to_string());
        user_metadata.insert("command".to_string(), notification.command_name.to_string());
        minio_client
            .put_object_content(&s3_bucket, object_name, content)
            .content_type("text/plain".to_string())
            .user_metadata(Some(user_metadata))
            .send()
            .await?;
    }
    Ok(())
}

pub fn create_oss_client() -> Result<Client, Box<dyn std::error::Error + Send + Sync>> {
    let s3_endpoint_url = env::var("S3_ENDPOINT_URL")?;
    let s3_access_key = env::var("S3_ACCESS_KEY")?;
    let s3_secret_key = env::var("S3_SECRET_KEY")?;
    let s3_region = env::var("S3_REGION");
    let s3_virtual_style = env::var("S3_VIRTUAL_STYLE").unwrap_or("0".to_string());

    let mut base_url = s3_endpoint_url.parse::<BaseUrl>()?;
    if let Ok(region) = s3_region {
        base_url.region = region;
    }
    if s3_virtual_style == "1" {
        base_url.virtual_style = true;
    }

    let static_provider = StaticProvider::new(&s3_access_key, &s3_secret_key, None);

    let client = ClientBuilder::new(base_url.clone())
        .provider(Some(Box::new(static_provider)))
        .build()?;
    Ok(client)
}
