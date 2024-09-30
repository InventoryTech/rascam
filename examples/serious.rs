use rascam::{ISO, *};
use std::fs::File;
use std::io::Write;
use std::{thread, time};

fn main() {
    // Set up logging to stdout
    simple_logger::SimpleLogger::new().env().init().unwrap();

    let info = info().unwrap();
    if info.cameras.len() < 1 {
        log::error!("Found 0 cameras. Exiting");
        // note that this doesn't run destructors
        ::std::process::exit(1);
    }
    log::info!("{}", info);

    serious(&info.cameras[0]);
}

fn serious(info: &CameraInfo) {
    let mut camera = SeriousCamera::new().unwrap();
    log::info!("camera created");
    camera.set_camera_num(0).unwrap();
    log::info!("camera number set");
    camera.create_encoder().unwrap();
    log::info!("encoder created");
    camera.enable_control_port(true).unwrap();
    log::info!("camera control port enabled");
    camera.set_camera_params(info).unwrap();
    log::info!("camera params set");

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
    log::info!("set camera format");
    camera.enable().unwrap();
    log::info!("camera enabled");
    camera.create_pool().unwrap();
    log::info!("pool created");

    camera.create_preview().unwrap();
    log::info!("preview created");
    camera.connect_preview().unwrap();
    log::info!("preview connected");
    camera.enable_preview().unwrap();
    log::info!("preview enabled");

    log::info!("taking photo");

    let sleep_duration = time::Duration::from_millis(2000);
    thread::sleep(sleep_duration);

    let receiver = camera.take().unwrap();

    let buffer = receiver.recv().unwrap().unwrap();

    File::create("image.rgb")
        .unwrap()
        .write_all(&buffer.get_bytes())
        .unwrap();

    log::info!("Raw rgb bytes written to image.rgb");
    log::info!("Try: convert -size 96x96 -depth 8 -colorspace RGB rgb:image.rgb image.png");
    // If imagemagick gives something like:
    //   convert-im6.q16: unexpected end-of-file `image.rgb': No such file or directory @ error/rgb.c/ReadRGBImage/239.
    // There is probably padding in the image. Check the width.
}
