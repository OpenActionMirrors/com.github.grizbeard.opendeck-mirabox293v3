use mirajazz::{
    device::DeviceQuery,
    types::{HidDeviceInfo, ImageFormat, ImageMirroring, ImageMode, ImageRotation},
};

// Must be unique between all the plugins, 2 characters long and match `DeviceNamespace` field in `manifest.json`
pub const DEVICE_NAMESPACE: &str = "93";

/// The 293 V3 is a 15-key device: 3 rows of 5, with no sixth column of small keys
/// like the AKP153-family devices have.
pub const ROW_COUNT: usize = 3;
pub const COL_COUNT: usize = 5;
pub const KEY_COUNT: usize = ROW_COUNT * COL_COUNT;
pub const ENCODER_COUNT: usize = 0;

#[derive(Debug, Clone)]
pub enum Kind {
    HSV293V3,
}

pub const MIRABOX_VID: u16 = 0x6603;
pub const HSV293V3_PID: u16 = 0x1006;

pub const HSV293V3_QUERY: DeviceQuery = DeviceQuery::new(65440, 1, MIRABOX_VID, HSV293V3_PID);

pub const QUERIES: [DeviceQuery; 1] = [HSV293V3_QUERY];

/// Every key on this device has the same 112x112 screen, so the format never varies.
///
/// The panel is mounted 180 degrees from the orientation the image arrives in,
/// so the image is rotated before it is sent.
pub fn get_image_format_for_key(_kind: &Kind, _key: u8) -> ImageFormat {
    ImageFormat {
        mode: ImageMode::JPEG,
        size: (112, 112),
        rotation: ImageRotation::Rot180,
        mirror: ImageMirroring::None,
    }
}

impl Kind {
    /// Matches devices VID+PID pairs to correct kinds
    pub fn from_vid_pid(vid: u16, pid: u16) -> Option<Self> {
        match vid {
            MIRABOX_VID => match pid {
                HSV293V3_PID => Some(Kind::HSV293V3),
                _ => None,
            },

            _ => None,
        }
    }

    /// Returns protocol version for device
    pub fn protocol_version(&self) -> usize {
        match self {
            Self::HSV293V3 => 3,
        }
    }

    /// There is no point relying on manufacturer/device names reported by the USB stack,
    /// so we return custom names for all the kinds of devices
    pub fn human_name(&self) -> String {
        match &self {
            Self::HSV293V3 => "Mirabox 293 V3",
        }
        .to_string()
    }

    /// Only "v1" devices need a suffix to disambiguate their shared serial number.
    /// This plugin supports a single "v3" device, so this is never called.
    pub fn id_suffix(&self) -> String {
        unreachable!()
    }
}

#[derive(Debug, Clone)]
pub struct CandidateDevice {
    pub id: String,
    pub dev: HidDeviceInfo,
    pub kind: Kind,
}
