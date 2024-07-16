use anyhow::{Context, Result};
use clap::Parser;
use futures::TryStreamExt;
use reqwest::multipart;
use std::time::Instant;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    num_requests: u32,

    #[arg(short, long, default_value_t = 1)]
    concurrency: u32,

    #[arg(short, long, default_value = "POST")]
    method: String,

    #[arg(short = 'H', long)]
    headers: Vec<String>,

    #[arg(short, long)]
    data: Option<String>,

    #[arg(short, long)]
    file: Option<String>,

    url: String,

    #[arg(long, default_value = "audio_content")]
    audio_part_name: String,
}
#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let client = reqwest::Client::new();

    let mut tasks = Vec::new();
    let total_start = Instant::now();
    let mut latencies = Vec::new();

    for _ in 0..args.num_requests {
        let client = client.clone();
        let args = args.clone();

        let task = tokio::spawn(async move {
            let start = Instant::now();

            let mut request_builder = client.request(
                args.method.parse().context("Invalid HTTP method")?,
                &args.url,
            );

            // Add headers
            for header in &args.headers {
                let parts: Vec<&str> = header.splitn(2, ':').collect();
                if parts.len() == 2 {
                    request_builder = request_builder.header(parts[0].trim(), parts[1].trim());
                }
            }

            // Handle multipart or JSON data
            if let Some(file_path) = &args.file {
                let file = File::open(file_path).await?;
                let stream = FramedRead::new(file, BytesCodec::new());
                let file_body = reqwest::Body::wrap_stream(stream);

                let audio_part = multipart::Part::stream(file_body)
                    .file_name("audio.wav")
                    .mime_str("audio/wav")?;

                let form = multipart::Form::new()
                    .text(
                        "request_content",
                        args.data
                            .unwrap_or_else(|| "{\"language\":\"en\"}".to_string()),
                    )
                    .part(args.audio_part_name, audio_part);

                request_builder = request_builder.multipart(form);
            } else if let Some(data) = &args.data {
                request_builder = request_builder.body(data.clone());
            }

            let response = request_builder.send().await?;
            let status = response.status();
            let headers = response.headers().clone();
            let body = response.text().await?;

            let duration = start.elapsed();

            Ok::<_, anyhow::Error>((status, headers, body, duration))
        });

        tasks.push(task);

        if tasks.len() >= args.concurrency as usize {
            let (result, _, remaining) = futures::future::select_all(tasks).await;
            match result {
                Ok(Ok((status, headers, body, duration))) => {
                    println!("Status: {}", status);
                    println!("Headers: {:#?}", headers);
                    println!("Body: {}", body);
                    println!("Latency: {:?}", duration);
                    latencies.push(duration);
                }
                Ok(Err(e)) => eprintln!("Request error: {}", e),
                Err(e) => eprintln!("Task join error: {}", e),
            }
            tasks = remaining;
        }
    }

    // Handle any remaining tasks
    for task in tasks {
        match task.await {
            Ok(Ok((status, headers, body, duration))) => {
                println!("Status: {}", status);
                println!("Headers: {:#?}", headers);
                println!("Body: {}", body);
                println!("Latency: {:?}", duration);
                latencies.push(duration);
            }
            Ok(Err(e)) => eprintln!("Request error: {}", e),
            Err(e) => eprintln!("Task join error: {}", e),
        }
    }
    let total_duration = total_start.elapsed();
    // Calculate and print summary statistics
    if !latencies.is_empty() {
        let latencies_sum: std::time::Duration = latencies.iter().sum();
        //sum of latencies
        let latencies_sum = latencies
            .iter()
            .fold(std::time::Duration::new(0, 0), |acc, &x| acc + x);
        let avg_duration = latencies_sum / latencies.len() as u32;
        let min_duration = latencies.iter().min().unwrap();
        let max_duration = latencies.iter().max().unwrap();

        println!("\nSummary:");
        println!("Concurrent requests: {}", args.concurrency);
        println!("Total requests: {}", latencies.len());
        println!("Average latency: {:?}", avg_duration);
        println!("Min latency: {:?}", min_duration);
        println!("Max latency: {:?}", max_duration);
        println!("Total time: {:?}", total_duration);
        println!("Sum of latencies: {:?}", latencies_sum);
    }

    Ok(())
}
