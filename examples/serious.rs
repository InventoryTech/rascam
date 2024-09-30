use rascam::{ISO, *};
use std::fs::File;
use std::io::Write;
use std::{thread, time};

fn main() {
    // Set up logging to stdout
    tracing_subscriber::fmt::init();

    let info = info().unwrap();
    if info.cameras.len() < 1 {
        tracing::error!("Found 0 cameras. Exiting");
        // note that this doesn't run destructors
        ::std::process::exit(1);
    }
    tracing::info!("{}", info);

    serious(&info.cameras[0]);
}

fn serious(info: &CameraInfo) {
    let mut camera = SeriousCamera::new().unwrap();
    tracing::info!("camera created");
    camera.set_camera_num(0).unwrap();
    tracing::info!("camera number set");
    camera.create_encoder().unwrap();
    tracing::info!("encoder created");
    camera.enable_control_port(true).unwrap();
    tracing::info!("camera control port enabled");
    camera.set_camera_params(info).unwrap();
    tracing::info!("camera params set");

    let settings = CameraSettings {
        encoding: MMAL_ENCODING_RGB24,
        width: 96, // 96px will not require padding
        height: 96,
        iso: ISO::IsoAuto,
        zero_copy: true,
        use_encoder: false,
        ..Default::default()
    };

    camera.set_camera_format(&settings).unwrap();
    tracing::info!("set camera format");
    camera.enable().unwrap();
    tracing::info!("camera enabled");
    camera.create_pool().unwrap();
    tracing::info!("pool created");

    camera.create_preview().unwrap();
    tracing::info!("preview created");
    camera.connect_preview().unwrap();
    tracing::info!("preview connected");
    camera.enable_preview().unwrap();
    tracing::info!("preview enabled");

    tracing::info!("taking photo");

    let sleep_duration = time::Duration::from_millis(2000);
    thread::sleep(sleep_duration);

    let receiver = camera.take().unwrap();

    let buffer = receiver.recv().unwrap().unwrap();

    File::create("image.rgb")
        .unwrap()
        .write_all(&buffer.get_bytes())
        .unwrap();

    tracing::info!("Raw rgb bytes written to image.rgb");
    tracing::info!("Try: convert -size 96x96 -depth 8 -colorspace RGB rgb:image.rgb image.png");
    // If imagemagick gives something like:
    //   convert-im6.q16: unexpected end-of-file `image.rgb': No such file or directory @ error/rgb.c/ReadRGBImage/239.
    // There is probably padding in the image. Check the width.
}
