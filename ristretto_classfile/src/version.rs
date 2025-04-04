use crate::error::Error::InvalidVersion;
use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

pub const JAVA_PREVIEW_MINOR_VERSION: u16 = 65535;

/// Implementation of Version based on `ClassFile` format for major/minor versions.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.1>
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Version {
    Java1_0_2 { minor: u16 },
    Java1_1 { minor: u16 },
    Java1_2 { minor: u16 },
    Java1_3 { minor: u16 },
    Java1_4 { minor: u16 },
    Java5_0 { minor: u16 },
    Java6 { minor: u16 },
    Java7 { minor: u16 },
    Java8 { minor: u16 },
    Java9 { minor: u16 },
    Java10 { minor: u16 },
    Java11 { minor: u16 },
    Java12 { minor: u16 },
    Java13 { minor: u16 },
    Java14 { minor: u16 },
    Java15 { minor: u16 },
    Java16 { minor: u16 },
    Java17 { minor: u16 },
    Java18 { minor: u16 },
    Java19 { minor: u16 },
    Java20 { minor: u16 },
    Java21 { minor: u16 },
    Java22 { minor: u16 },
    Java23 { minor: u16 },
    Java24 { minor: u16 },
    Java25 { minor: u16 },
}

impl Version {
    /// Create a new version from a major and minor version.
    ///
    /// # Errors
    /// Returns an error if the major and minor version are invalid.
    pub fn from(major: u16, minor: u16) -> Result<Self> {
        if major >= 56 && minor != 0 && minor != JAVA_PREVIEW_MINOR_VERSION {
            return Err(InvalidVersion { major, minor });
        }

        let version = match major {
            45 => Version::Java1_0_2 { minor },
            46 => Version::Java1_2 { minor },
            47 => Version::Java1_3 { minor },
            48 => Version::Java1_4 { minor },
            49 => Version::Java5_0 { minor },
            50 => Version::Java6 { minor },
            51 => Version::Java7 { minor },
            52 => Version::Java8 { minor },
            53 => Version::Java9 { minor },
            54 => Version::Java10 { minor },
            55 => Version::Java11 { minor },
            56 => Version::Java12 { minor },
            57 => Version::Java13 { minor },
            58 => Version::Java14 { minor },
            59 => Version::Java15 { minor },
            60 => Version::Java16 { minor },
            61 => Version::Java17 { minor },
            62 => Version::Java18 { minor },
            63 => Version::Java19 { minor },
            64 => Version::Java20 { minor },
            65 => Version::Java21 { minor },
            66 => Version::Java22 { minor },
            67 => Version::Java23 { minor },
            68 => Version::Java24 { minor },
            69 => Version::Java25 { minor },
            _ => return Err(InvalidVersion { major, minor }),
        };

        Ok(version)
    }

    /// Returns the major version.
    #[must_use]
    pub fn major(&self) -> u16 {
        match self {
            Version::Java1_0_2 { .. } | Version::Java1_1 { .. } => 45,
            Version::Java1_2 { .. } => 46,
            Version::Java1_3 { .. } => 47,
            Version::Java1_4 { .. } => 48,
            Version::Java5_0 { .. } => 49,
            Version::Java6 { .. } => 50,
            Version::Java7 { .. } => 51,
            Version::Java8 { .. } => 52,
            Version::Java9 { .. } => 53,
            Version::Java10 { .. } => 54,
            Version::Java11 { .. } => 55,
            Version::Java12 { .. } => 56,
            Version::Java13 { .. } => 57,
            Version::Java14 { .. } => 58,
            Version::Java15 { .. } => 59,
            Version::Java16 { .. } => 60,
            Version::Java17 { .. } => 61,
            Version::Java18 { .. } => 62,
            Version::Java19 { .. } => 63,
            Version::Java20 { .. } => 64,
            Version::Java21 { .. } => 65,
            Version::Java22 { .. } => 66,
            Version::Java23 { .. } => 67,
            Version::Java24 { .. } => 68,
            Version::Java25 { .. } => 69,
        }
    }

    /// Returns the minor version.
    #[must_use]
    pub fn minor(&self) -> u16 {
        match self {
            Version::Java1_0_2 { minor, .. }
            | Version::Java1_1 { minor, .. }
            | Version::Java1_2 { minor, .. }
            | Version::Java1_3 { minor, .. }
            | Version::Java1_4 { minor, .. }
            | Version::Java5_0 { minor, .. }
            | Version::Java6 { minor, .. }
            | Version::Java7 { minor, .. }
            | Version::Java8 { minor, .. }
            | Version::Java9 { minor, .. }
            | Version::Java10 { minor, .. }
            | Version::Java11 { minor, .. }
            | Version::Java12 { minor, .. }
            | Version::Java13 { minor, .. }
            | Version::Java14 { minor, .. }
            | Version::Java15 { minor, .. }
            | Version::Java16 { minor, .. }
            | Version::Java17 { minor, .. }
            | Version::Java18 { minor, .. }
            | Version::Java19 { minor, .. }
            | Version::Java20 { minor, .. }
            | Version::Java21 { minor, .. }
            | Version::Java22 { minor, .. }
            | Version::Java23 { minor, .. }
            | Version::Java24 { minor, .. }
            | Version::Java25 { minor, .. } => *minor,
        }
    }

    /// Returns the major version for Java (e.g. 8 for Java 8).
    #[must_use]
    pub fn java(&self) -> u16 {
        self.major() - 44
    }

    /// Returns true if the current major version supports the given version.
    #[must_use]
    pub fn supports(self, version: &Version) -> bool {
        self.major() >= version.major()
    }

    /// Returns true if the current major version is >= Java 12 (`56`) is a preview minor
    /// version (`65535`).
    #[must_use]
    pub fn is_preview(&self) -> bool {
        self.major() >= 56 && self.minor() == JAVA_PREVIEW_MINOR_VERSION
    }

    /// Deserialize the major and minor version bytes.
    ///
    /// ```text
    /// |--------------------- u32 ---------------------|
    /// |--------- u16 ---------|--------- u16 ---------|
    /// |      Minor Ver.       |      Major Ver.       |
    /// |     31 30 29 .. 16    |     15 14 13 .. 0     |
    /// ```
    ///
    /// # Errors
    /// Returns an error if the bytes do not represent a valid version.
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<Version> {
        let minor = bytes.read_u16::<BigEndian>()?;
        let major = bytes.read_u16::<BigEndian>()?;
        Version::from(major, minor)
    }

    /// Serialize the major and minor version to bytes.
    ///
    /// ```text
    /// |--------------------- u32 ---------------------|
    /// |--------- u16 ---------|--------- u16 ---------|
    /// |      Minor Ver.       |      Major Ver.       |
    /// |     31 30 29 .. 16    |     15 14 13 .. 0     |
    /// ```
    ///
    /// # Errors
    /// Should not occur; reserved for future use.
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.minor())?;
        bytes.write_u16::<BigEndian>(self.major())?;
        Ok(())
    }
}

impl Default for Version {
    fn default() -> Self {
        Version::Java1_0_2 { minor: 0 }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Version::Java1_0_2 { .. } => write!(f, "Java 1.0.2"),
            Version::Java1_1 { .. } => write!(f, "Java 1.1"),
            Version::Java1_2 { .. } => write!(f, "Java 1.2"),
            Version::Java1_3 { .. } => write!(f, "Java 1.3"),
            Version::Java1_4 { .. } => write!(f, "Java 1.4"),
            Version::Java5_0 { .. } => write!(f, "Java 5.0"),
            Version::Java6 { .. } => write!(f, "Java 6"),
            Version::Java7 { .. } => write!(f, "Java 7"),
            Version::Java8 { .. } => write!(f, "Java 8"),
            Version::Java9 { .. } => write!(f, "Java 9"),
            Version::Java10 { .. } => write!(f, "Java 10"),
            Version::Java11 { .. } => write!(f, "Java 11"),
            Version::Java12 { .. } => write!(f, "Java 12"),
            Version::Java13 { .. } => write!(f, "Java 13"),
            Version::Java14 { .. } => write!(f, "Java 14"),
            Version::Java15 { .. } => write!(f, "Java 15"),
            Version::Java16 { .. } => write!(f, "Java 16"),
            Version::Java17 { .. } => write!(f, "Java 17"),
            Version::Java18 { .. } => write!(f, "Java 18"),
            Version::Java19 { .. } => write!(f, "Java 19"),
            Version::Java20 { .. } => write!(f, "Java 20"),
            Version::Java21 { .. } => write!(f, "Java 21"),
            Version::Java22 { .. } => write!(f, "Java 22"),
            Version::Java23 { .. } => write!(f, "Java 23"),
            Version::Java24 { .. } => write!(f, "Java 24"),
            Version::Java25 { .. } => write!(f, "Java 25"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io;

    const MIN_MAJOR: u16 = 45;
    const MAX_MAJOR: u16 = 69;

    #[test]
    fn all_known_versions() -> Result<()> {
        let versions = [
            Version::Java1_0_2 { minor: 0 },
            Version::Java1_1 { minor: 0 },
            Version::Java1_2 { minor: 0 },
            Version::Java1_3 { minor: 0 },
            Version::Java1_4 { minor: 0 },
            Version::Java5_0 { minor: 0 },
            Version::Java6 { minor: 0 },
            Version::Java7 { minor: 0 },
            Version::Java8 { minor: 0 },
            Version::Java9 { minor: 0 },
            Version::Java10 { minor: 0 },
            Version::Java11 { minor: 0 },
            Version::Java12 { minor: 0 },
            Version::Java13 { minor: 0 },
            Version::Java14 { minor: 0 },
            Version::Java15 { minor: 0 },
            Version::Java16 { minor: 0 },
            Version::Java17 { minor: 0 },
            Version::Java18 { minor: 0 },
            Version::Java19 { minor: 0 },
            Version::Java20 { minor: 0 },
            Version::Java21 { minor: 0 },
            Version::Java22 { minor: 0 },
            Version::Java23 { minor: 0 },
            Version::Java24 { minor: 0 },
            Version::Java25 { minor: 0 },
        ];

        for (index, version) in versions.iter().enumerate() {
            let mut index = u16::try_from(index)?;
            let major = version.major();
            if major == MIN_MAJOR {
                index = 0;
            } else {
                index -= 1;
            }
            assert!(version.to_string().starts_with("Java "));
            assert_eq!(major, MIN_MAJOR + index);
            assert_eq!(version.minor(), 0);
            assert_eq!(version.java(), version.major() - 44);
        }

        Ok(())
    }

    #[test]
    fn test_from() -> Result<()> {
        for major in MIN_MAJOR..=MAX_MAJOR {
            // Test with minor version 0
            let version = Version::from(major, 0)?;
            assert_eq!(version.major(), major);
            // Test with preview minor version
            let version = Version::from(major, JAVA_PREVIEW_MINOR_VERSION)?;
            assert_eq!(version.major(), major);
        }
        Ok(())
    }

    #[test]
    fn test_from_invalid_version() {
        assert_eq!(
            Err(InvalidVersion {
                major: MIN_MAJOR - 1,
                minor: 0
            }),
            Version::from(MIN_MAJOR - 1, 0)
        );
        assert_eq!(
            Err(InvalidVersion {
                major: 56,
                minor: 42
            }),
            Version::from(56, 42)
        );
    }

    #[test]
    fn test_major() {
        let version = Version::Java21 { minor: 0 };
        assert_eq!(version.major(), 65);
    }

    #[test]
    fn test_minor() {
        let minor = 3;
        let version = Version::Java11 { minor };
        assert_eq!(version.minor(), minor);
    }

    #[test]
    fn test_supports() {
        assert!(Version::Java11 { minor: 0 }.supports(&Version::Java5_0 { minor: 0 }));
        assert!(!Version::Java5_0 { minor: 0 }.supports(&Version::Java11 { minor: 0 }));
    }

    #[test]
    fn test_is_preview() {
        assert!(!Version::Java11 { minor: 0 }.is_preview());
        assert!(
            Version::Java21 {
                minor: JAVA_PREVIEW_MINOR_VERSION
            }
            .is_preview()
        );
    }

    #[test]
    fn test_default() {
        let version = Version::default();
        assert_eq!(version, Version::Java1_0_2 { minor: 0 });
    }

    #[test]
    fn test_serialization() -> Result<()> {
        let version = Version::Java21 {
            minor: JAVA_PREVIEW_MINOR_VERSION,
        };
        let expected_value: u32 = 4_294_901_825;
        let mut bytes = Vec::new();
        version.clone().to_bytes(&mut bytes)?;
        let mut cursor = io::Cursor::new(bytes);
        assert_eq!(expected_value, cursor.read_u32::<BigEndian>()?);

        let mut bytes = Cursor::new(expected_value.to_be_bytes().to_vec());
        assert_eq!(Ok(version), Version::from_bytes(&mut bytes));
        Ok(())
    }
}
