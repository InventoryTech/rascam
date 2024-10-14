/// Setting for MMAL camera
///
/// Not supported:
/// 1) Anything video related
/// 2) Flash info, modes and types
/// 3) Red Eye
/// 4) Image FX
/// 5) Autofocus
/// 6) Face detect
/// 7) Dynamic range compression (DRC)
/// 8) Time Stamp modes
///
use libc::c_uint;
use mmal_sys as ffi;
use std::fmt;
use strum::{Display, EnumString};

#[derive(Debug, Clone, Copy)]
// use the strum crate to derive FromStr and Display
#[derive(EnumString, Display)]
pub enum SensorMode {
    ModeAuto = 0,
    Mode1080pCropped = 1,
    Mode5MPix,
    Mode5MPixPerSecond,
    Mode2x2Binned,
    Mode2x2Binned16to9,
    ModeVGA60fps,
    ModeVGA60fps90fps,
}

impl SensorMode {
    pub fn to_u32(&self) -> u32 {
        *self as u32
    }
}

#[derive(Debug, Clone, Copy)]
// use the strum crate to derive FromStr and Display
#[derive(EnumString, Display)]
pub enum ISO {
    IsoAuto = 0,
    Iso125 = 125,
    Iso160 = 160,
    Iso200 = 200,
    Iso250 = 250,
    Iso320 = 320,
    Iso400 = 400,
    Iso500 = 500,
    Iso640 = 640,
    Iso800 = 800,
    Iso1000 = 1000,
    Iso1250 = 1250,
    Iso1600 = 1600,
    Iso2000 = 2000,
    Iso2500 = 2500,
    Iso3200 = 3200,
}

impl ISO {
    pub fn to_u32(&self) -> u32 {
        *self as u32
    }
}

#[derive(Debug, Clone, Copy)]
// use the strum crate to derive FromStr and Display
#[derive(EnumString, Display)]
pub enum MeteringMode {
    // values from MMAL_PARAM_EXPOSUREMETERINGMODE_T in https://github.com/raspberrypi/userland/blob/master/interface/mmal/mmal_parameters_camera.h
    Average =
        ffi::MMAL_PARAM_EXPOSUREMETERINGMODE_T_MMAL_PARAM_EXPOSUREMETERINGMODE_AVERAGE as isize,
    Spot = ffi::MMAL_PARAM_EXPOSUREMETERINGMODE_T_MMAL_PARAM_EXPOSUREMETERINGMODE_SPOT as isize,
    Backlit =
        ffi::MMAL_PARAM_EXPOSUREMETERINGMODE_T_MMAL_PARAM_EXPOSUREMETERINGMODE_BACKLIT as isize,
    Matrix = ffi::MMAL_PARAM_EXPOSUREMETERINGMODE_T_MMAL_PARAM_EXPOSUREMETERINGMODE_MATRIX as isize,
}

impl MeteringMode {
    pub fn to_i32(&self) -> i32 {
        *self as i32
    }
}

#[derive(Debug, Clone, Copy)]
// use the strum crate to derive FromStr and Display
#[derive(EnumString, Display)]
// values from MMAL_PARAM_EXPOSUREMODE_T in https://github.com/raspberrypi/userland/blob/master/interface/mmal/mmal_parameters_camera.h
pub enum ExposureMode {
    Off = ffi::MMAL_PARAM_EXPOSUREMODE_T_MMAL_PARAM_EXPOSUREMODE_OFF as isize,
    Auto = ffi::MMAL_PARAM_EXPOSUREMODE_T_MMAL_PARAM_EXPOSUREMODE_AUTO as isize,
    Night = ffi::MMAL_PARAM_EXPOSUREMODE_T_MMAL_PARAM_EXPOSUREMODE_NIGHT as isize,
    NightPreview = ffi::MMAL_PARAM_EXPOSUREMODE_T_MMAL_PARAM_EXPOSUREMODE_NIGHTPREVIEW as isize,
    Backlight = ffi::MMAL_PARAM_EXPOSUREMODE_T_MMAL_PARAM_EXPOSUREMODE_BACKLIGHT as isize,
    Spotlight = ffi::MMAL_PARAM_EXPOSUREMODE_T_MMAL_PARAM_EXPOSUREMODE_SPOTLIGHT as isize,
    Sports = ffi::MMAL_PARAM_EXPOSUREMODE_T_MMAL_PARAM_EXPOSUREMODE_SPORTS as isize,
    Snow = ffi::MMAL_PARAM_EXPOSUREMODE_T_MMAL_PARAM_EXPOSUREMODE_SNOW as isize,
    Beach = ffi::MMAL_PARAM_EXPOSUREMODE_T_MMAL_PARAM_EXPOSUREMODE_BEACH as isize,
    VeryLong = ffi::MMAL_PARAM_EXPOSUREMODE_T_MMAL_PARAM_EXPOSUREMODE_VERYLONG as isize,
    FixedFPS = ffi::MMAL_PARAM_EXPOSUREMODE_T_MMAL_PARAM_EXPOSUREMODE_FIXEDFPS as isize,
    AntiShake = ffi::MMAL_PARAM_EXPOSUREMODE_T_MMAL_PARAM_EXPOSUREMODE_ANTISHAKE as isize,
    FireWorks = ffi::MMAL_PARAM_EXPOSUREMODE_T_MMAL_PARAM_EXPOSUREMODE_FIREWORKS as isize,
}

impl ExposureMode {
    pub fn to_i32(&self) -> i32 {
        *self as i32
    }
}

#[derive(Debug, Clone, Copy)]
// use the strum crate to derive FromStr and Display
#[derive(EnumString, Display)]
/// Auto White Balance Mode
// values from MMAL_PARAM_AWBMODE_T in https://github.com/raspberrypi/userland/blob/master/interface/mmal/mmal_parameters_camera.h
pub enum AwbMode {
    Auto = ffi::MMAL_PARAM_AWBMODE_T_MMAL_PARAM_AWBMODE_AUTO as isize,
    Sunlight = ffi::MMAL_PARAM_AWBMODE_T_MMAL_PARAM_AWBMODE_SUNLIGHT as isize,
    Cloud = ffi::MMAL_PARAM_AWBMODE_T_MMAL_PARAM_AWBMODE_CLOUDY as isize,
    Shade = ffi::MMAL_PARAM_AWBMODE_T_MMAL_PARAM_AWBMODE_SHADE as isize,
    Tungsten = ffi::MMAL_PARAM_AWBMODE_T_MMAL_PARAM_AWBMODE_TUNGSTEN as isize,
    Fluorescent = ffi::MMAL_PARAM_AWBMODE_T_MMAL_PARAM_AWBMODE_FLUORESCENT as isize,
    Incandescent = ffi::MMAL_PARAM_AWBMODE_T_MMAL_PARAM_AWBMODE_INCANDESCENT as isize,
    Flash = ffi::MMAL_PARAM_AWBMODE_T_MMAL_PARAM_AWBMODE_FLASH as isize,
    Horizon = ffi::MMAL_PARAM_AWBMODE_T_MMAL_PARAM_AWBMODE_HORIZON as isize,
    Greyworld = ffi::MMAL_PARAM_AWBMODE_T_MMAL_PARAM_AWBMODE_GREYWORLD as isize,
    Off = ffi::MMAL_PARAM_AWBMODE_T_MMAL_PARAM_AWBMODE_OFF as isize,
}

impl AwbMode {
    pub fn to_u32(&self) -> u32 {
        *self as u32
    }
}

#[derive(Debug, Clone, Copy)]
// use the strum crate to derive FromStr and Display
#[derive(EnumString, Display)]
/// Flicker reduction mode
// values from MMAL_PARAM_FLICKERAVOID_T in https://github.com/raspberrypi/userland/blob/master/interface/mmal/mmal_parameters_camera.h
pub enum FlickerAvoidMode {
    Off = ffi::MMAL_PARAM_FLICKERAVOID_T_MMAL_PARAM_FLICKERAVOID_OFF as isize,
    Auto = ffi::MMAL_PARAM_FLICKERAVOID_T_MMAL_PARAM_FLICKERAVOID_AUTO as isize,
    Avoid50Hz = ffi::MMAL_PARAM_FLICKERAVOID_T_MMAL_PARAM_FLICKERAVOID_50HZ as isize,
    Avoid60Hz = ffi::MMAL_PARAM_FLICKERAVOID_T_MMAL_PARAM_FLICKERAVOID_60HZ as isize,
}

impl FlickerAvoidMode {
    pub fn to_i32(&self) -> i32 {
        *self as i32
    }
}

#[derive(Debug, Clone, Copy)]
// use the strum crate to derive FromStr and Display
#[derive(EnumString, Display)]
pub enum MirrorMode {
    None = ffi::MMAL_PARAM_MIRROR_T_MMAL_PARAM_MIRROR_NONE as isize,
    Vertical = ffi::MMAL_PARAM_MIRROR_T_MMAL_PARAM_MIRROR_VERTICAL as isize,
    Horizontal = ffi::MMAL_PARAM_MIRROR_T_MMAL_PARAM_MIRROR_HORIZONTAL as isize,
    Both = ffi::MMAL_PARAM_MIRROR_T_MMAL_PARAM_MIRROR_BOTH as isize,
}

impl MirrorMode {
    pub fn to_i32(&self) -> i32 {
        *self as i32
    }
}

#[derive(Debug, Clone, Copy)]
// use the strum crate to derive FromStr and Display
#[derive(EnumString, Display)]
/// Image rotation
pub enum Rotation {
    Rotate0 = 0,
    Rotate90 = 90,
    Rotate180 = 180,
    Rotate270 = 270,
}

impl Rotation {
    pub fn to_i32(&self) -> i32 {
        *self as i32
    }
}

// JPEG Quality
pub const DEFAULT_JPEG_QUALITY: u32 = 95;

/// Settings for the camera.
///
/// ```
/// # use rascam::{CameraError, CameraSettings, SimpleCamera};
/// #
/// # let info = rascam::info().unwrap().cameras[0].clone();
/// # let mut camera = SimpleCamera::new(info.clone()).unwrap();
/// #
/// let settings = CameraSettings{
///     width: info.max_width,
///     height: info.max_height,
///     ..CameraSettings::default()
/// };
/// camera.configure(settings);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct CameraSettings {
    pub encoding: c_uint,
    /// image width in pixels, 0 = maximum
    pub width: u32,
    /// image height in pixels, 0 = maximum
    pub height: u32,
    /// ISO. Default is Auto
    pub iso: ISO,
    // shutter_speed: 0 = auto, otherwise the shutter speed in microseconds
    pub shutter_speed: u32,
    /// Exposure mode
    pub exposure_mode: ExposureMode,
    /// Metering Mode
    pub metering_mode: MeteringMode,
    /// White Balance
    pub awb_mode: AwbMode,
    /// EV compensation in steps of 1/6 stop (-25 to +25)
    pub exposure_compensation: i32,
    /// Brightness 0% to 100%, default = 50%
    pub brightness: u32,
    /// Contrast -100% to +100%, default = 0%
    pub contrast: i32,
    // Saturation  -100% to 100%, default = 0%
    pub saturation: i32,
    // Sharpness  -100% to 100%, default = 0%
    pub sharpness: i32,
    // rotation (0, 90, 180, or 270), default = 0
    pub rotation: Rotation,
    // H&V flip , default = false
    pub horizontal_flip: bool,
    pub vertical_flip: bool,
    // Sensor mode
    pub sensor_mode: SensorMode,
    // Image quality. range 0..100
    pub quality: u32,
    // flicker avoidance mode  (Off, Auto, 50Hz, 60Hz), default = Auto
    pub flicker_avoid: FlickerAvoidMode,

    pub zero_copy: bool,
    /// `use_encoder` will go away
    pub use_encoder: bool,
}

impl Default for CameraSettings {
    fn default() -> Self {
        CameraSettings {
            encoding: ffi::MMAL_ENCODING_JPEG,
            width: 0,
            height: 0,
            iso: ISO::IsoAuto,
            shutter_speed: 0,
            exposure_mode: ExposureMode::Auto,
            metering_mode: MeteringMode::Average,
            awb_mode: AwbMode::Auto,
            exposure_compensation: 0,
            brightness: 50,
            contrast: 0,
            saturation: 0,
            sharpness: 0,
            rotation: Rotation::Rotate0,
            horizontal_flip: false,
            vertical_flip: false,
            flicker_avoid: FlickerAvoidMode::Auto,
            sensor_mode: SensorMode::ModeAuto,
            quality: DEFAULT_JPEG_QUALITY,
            zero_copy: false,
            use_encoder: true,
        }
    }
}

impl fmt::Display for CameraSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        writeln!(f)?;
        // Modes
        writeln!(f, "Modes:")?;
        writeln!(f, "  Exposure Mode: {}", self.exposure_mode)?;
        writeln!(f, "  Metering Mode: {}", self.metering_mode)?;
        writeln!(f, "  Flicker Mode:  {}", self.flicker_avoid)?;
        writeln!(f, "  AWB Mode:      {}", self.awb_mode)?;
        // Exposure settings
        writeln!(f, "Exposure:")?;
        writeln!(f, "  ISO:            {}", self.iso)?;
        writeln!(f, "  Shutter Speed:  {}", self.shutter_speed)?;
        writeln!(f, "  Compensation:   {}", self.exposure_compensation)?;
        // image output settings
        writeln!(f, "Image Output:")?;
        writeln!(f, "  Encoding:       {}", u32_to_string(self.encoding))?;
        writeln!(f, "  Brightness:     {}", self.brightness)?;
        writeln!(f, "  Contrast:       {}", self.contrast)?;
        writeln!(f, "  Saturation:     {}", self.saturation)?;
        writeln!(f, "  Sharpness:      {}", self.sharpness)?;
        writeln!(f, "  Width:          {}", self.width)?;
        writeln!(f, "  Height:         {}", self.height)?;
        writeln!(f, "  Rotation:       {}", self.rotation)?;
        writeln!(f, "  V Flip:         {}", self.vertical_flip)?;
        writeln!(f, "  H Flip:         {}", self.horizontal_flip)
    }
}

fn u32_to_string(input: u32) -> String {
    let bytes = [
        (input & 0xFF) as u8,
        ((input >> 8) & 0xFF) as u8,
        ((input >> 16) & 0xFF) as u8,
        ((input >> 24) & 0xFF) as u8,
    ];
    bytes
        .iter()
        .filter(|&&b| b != 0) // Filter out null characters
        .map(|&b| b as char) // Convert remaining bytes to chars
        .collect()
}
