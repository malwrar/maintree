use std::os::raw::c_int;
use x11::xdamage;
use x11::xlib;
use crate::{
    error::{
        Error,
        Result
    },
    window_system::{
        Display,
        Window,
        WindowRef,
    },
};

pub enum DamageWatcherLevel {
    RawRectangles,
    DeltaRectangles,
    BoundingBox,
    NonEmpty
}

/// Manages the lifecycle using xdamage.
#[derive(Debug)]
pub struct DamageWatcher {
    display: Display,
    damage: xdamage::Damage,
}

impl DamageWatcher {
    pub fn watch(
        window: &WindowRef,
        level: DamageWatcherLevel
    ) -> Result<Self> {
        let level = match level {
            DamageWatcherLevel::RawRectangles => xdamage::XDamageReportRawRectangles,
            DamageWatcherLevel::DeltaRectangles => xdamage::XDamageReportDeltaRectangles,
            DamageWatcherLevel::BoundingBox => xdamage::XDamageReportBoundingBox,
            DamageWatcherLevel::NonEmpty => xdamage::XDamageReportNonEmpty,
        };

        let damage = unsafe {
            xdamage::XDamageCreate(window.native_display(), window.native(),
                    level)
        };
        // TODO: check for error

        Ok(Self {
            display: window.display(),
            damage,
        })
    }
}

impl Drop for DamageWatcher {
    fn drop(&mut self) {
        unsafe {
            xdamage::XDamageDestroy(self.display.native(), self.damage);
        }
        // TODO: check for error
    }
}

/// Contains information about the glx extension.
pub struct DamageExtensionInfo {
    pub version_major: c_int,
    pub version_minor: c_int,
    pub event_base: c_int,
    pub error_base: c_int,
}

impl DamageExtensionInfo {
    pub fn query(display: &Display) -> Option<Self> {
        let mut info = Self {
            version_major: 0,
            version_minor: 0,
            event_base: 0,
            error_base: 0,
        };

        // Verify the extension is around before continuing
        let status = unsafe {
            xdamage::XDamageQueryExtension(display.native(),
                    &mut info.event_base, &mut info.error_base)
        };
        // TODO: check for error

        if status == xlib::False {
            return None;
        }

        unsafe {
            xdamage::XDamageQueryVersion(display.native(), &mut info.version_major,
                    &mut info.version_minor);
        }
        // TODO: check for error
        
        Some(info)
    }
}

/// Verify damage is available.
pub fn check_damage_extension(
    display: &Display
) -> Result<DamageExtensionInfo> {
    match DamageExtensionInfo::query(display) {
        Some(info) => Ok(info),
        None => Err(Error::new("Xdamage not installed on display."))
    }
}
