//! Windows machine-wide print-dialog defaults, not enforced job policy.

use super::spool;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct PrintDefaults {
    pub grayscale: bool,
    pub duplex: Option<spool::Duplex>,
}

/// Missing, unreadable, wrongly typed or unknown values retain the deployment defaults.
/// Decode independently so one bad value does not discard the other valid setting.
#[cfg(any(target_os = "windows", test))]
fn decode(grayscale: Option<u32>, duplex: Option<u32>) -> PrintDefaults {
    PrintDefaults {
        grayscale: grayscale != Some(0),
        duplex: match duplex {
            Some(0) => None,
            Some(1) => Some(spool::Duplex::Off),
            Some(3) => Some(spool::Duplex::ShortEdge),
            _ => Some(spool::Duplex::LongEdge),
        },
    }
}

/// Read the 64-bit registry view for x86, x64 and ARM64 builds alike. No HKCU fallback,
/// environment override, registry writes, elevation or subprocess is needed by the app.
#[cfg(target_os = "windows")]
pub(super) fn machine() -> PrintDefaults {
    use winsafe::{HKEY, RegistryValue, co};

    let key = HKEY::LOCAL_MACHINE.RegOpenKeyEx(
        Some("SOFTWARE\\PdfCraft\\PrintDefaults"),
        co::REG_OPTION::default(),
        co::KEY::QUERY_VALUE | co::KEY::WOW64_64KEY,
    );
    let Ok(key) = key else { return decode(None, None) };
    let read = |name: &str| match key.RegGetValue(None, Some(name), co::RRF::RT_REG_DWORD).ok() {
        Some(RegistryValue::Dword(value)) => Some(value),
        _ => None,
    };
    decode(read("Grayscale"), read("Duplex"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_and_invalid_values_use_grayscale_and_long_edge() {
        let wanted = PrintDefaults { grayscale: true, duplex: Some(spool::Duplex::LongEdge) };
        assert_eq!(decode(None, None), wanted);
        assert_eq!(decode(Some(u32::MAX), Some(u32::MAX)), wanted);
        assert_eq!(decode(Some(1), Some(2)), wanted);
    }

    #[test]
    fn machine_values_are_independent_and_allow_every_dialog_choice() {
        for (value, wanted) in [(0, None), (1, Some(spool::Duplex::Off)), (2, Some(spool::Duplex::LongEdge)), (3, Some(spool::Duplex::ShortEdge))] {
            assert_eq!(decode(Some(0), Some(value)), PrintDefaults { grayscale: false, duplex: wanted });
            assert_eq!(decode(None, Some(value)), PrintDefaults { grayscale: true, duplex: wanted });
        }
        assert!(!decode(Some(0), Some(99)).grayscale);
        assert_eq!(decode(Some(99), Some(1)).duplex, Some(spool::Duplex::Off));
    }
}
