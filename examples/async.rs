use rascam::*;
use std::time::Duration;
use tokio::fs::File;
use tokio::prelude::*;
use tokio::time::delay_for;

#[tokio::main]
async fn main() {
    // Set up logging to stdout
    tracing_subscriber::fmt::init();

    let info = info().unwrap();
    if info.cameras.len() < 1 {
        tracing::error!("Found 0 cameras. Exiting");
        // note that this doesn't run destructors
        ::std::process::exit(1);
    }
    tracing::info!("{}", info);

    let result = simple_async(&info.cameras[0]).await;
    match result {
        Ok(_) => tracing::info!("Saved image as image.jpg"),
        Err(err) => {
            tracing::error!("error: {}", err);
            ::std::process::exit(1);
        }
    }
}

async fn simple_async(info: &CameraInfo) -> Result<(), Box<dyn std::error::Error>> {
    let mut camera = SimpleCamera::new(info.clone())?;
    camera.activate()?;

    delay_for(Duration::from_millis(2000)).await;

    let b = camera.take_one_async().await?;
    let mut file = File::create("image.jpg").await?;
    file.write_all(&b).await?;

    Ok(())
}
