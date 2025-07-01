use crate::error::Error::InvalidVersion;
use crate::error::Result;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::io::Cursor;

/// Constants representing the Java version 1.0.2.
pub const JAVA_1_0_2: Version = Version::Java1_0_2 { minor: 0 };
/// Constants representing the Java version 1.1.
pub const JAVA_1_1: Version = Version::Java1_1 { minor: 0 };
/// Constants representing the Java version 1.2.
pub const JAVA_1_2: Version = Version::Java1_2 { minor: 0 };
/// Constants representing the Java version 1.3.
pub const JAVA_1_3: Version = Version::Java1_3 { minor: 0 };
/// Constants representing the Java version 1.4.
pub const JAVA_1_4: Version = Version::Java1_4 { minor: 0 };
/// Constants representing the Java version 5.0.
pub const JAVA_5: Version = Version::Java5 { minor: 0 };
/// Constants representing the Java version 6.
pub const JAVA_6: Version = Version::Java6 { minor: 0 };
/// Constants representing the Java version 7.
pub const JAVA_7: Version = Version::Java7 { minor: 0 };
/// Constants representing the Java version 8.
pub const JAVA_8: Version = Version::Java8 { minor: 0 };
/// Constants representing the Java version 9.
pub const JAVA_9: Version = Version::Java9 { minor: 0 };
/// Constants representing the Java version 10.
pub const JAVA_10: Version = Version::Java10 { minor: 0 };
/// Constants representing the Java version 11.
pub const JAVA_11: Version = Version::Java11 { minor: 0 };
/// Constants representing the Java version 12.
pub const JAVA_12: Version = Version::Java12 { minor: 0 };
/// Constants representing the Java version 13.
pub const JAVA_13: Version = Version::Java13 { minor: 0 };
/// Constants representing the Java version 14.
pub const JAVA_14: Version = Version::Java14 { minor: 0 };
/// Constants representing the Java version 15.
pub const JAVA_15: Version = Version::Java15 { minor: 0 };
/// Constants representing the Java version 16.
pub const JAVA_16: Version = Version::Java16 { minor: 0 };
/// Constants representing the Java version 17.
pub const JAVA_17: Version = Version::Java17 { minor: 0 };
/// Constants representing the Java version 18.
pub const JAVA_18: Version = Version::Java18 { minor: 0 };
/// Constants representing the Java version 19.
pub const JAVA_19: Version = Version::Java19 { minor: 0 };
/// Constants representing the Java version 20.
pub const JAVA_20: Version = Version::Java20 { minor: 0 };
/// Constants representing the Java version 21.
pub const JAVA_21: Version = Version::Java21 { minor: 0 };
/// Constants representing the Java version 22.
pub const JAVA_22: Version = Version::Java22 { minor: 0 };
/// Constants representing the Java version 23.
pub const JAVA_23: Version = Version::Java23 { minor: 0 };
/// Constants representing the Java version 24.
pub const JAVA_24: Version = Version::Java24 { minor: 0 };
/// Constants representing the Java version 25.
pub const JAVA_25: Version = Version::Java25 { minor: 0 };

/// Minor version number that indicates a Java preview release.
///
/// The value 65535 (0xFFFF) is used to indicate a preview version of Java.
pub const JAVA_PREVIEW_MINOR_VERSION: u16 = 65535;

/// Implementation of Version based on `ClassFile` format for major/minor versions.
///
/// Represents the Java version that corresponds to a specific `ClassFile` format version. Each enum
/// variant corresponds to a specific Java version with its associated minor version.
///
/// # Examples
///
/// Creating and working with Version objects:
///
/// ```rust
/// use ristretto_classfile::{Version, JAVA_PREVIEW_MINOR_VERSION};
/// use std::io::Cursor;
///
/// // Create a Version from major and minor version numbers
/// let java11 = Version::from(55, 0)?;
/// let java17_preview = Version::from(61, JAVA_PREVIEW_MINOR_VERSION)?;
///
/// // Compare versions
/// assert!(java17_preview.supports(&java11)); // Java 17 supports Java 11 features
/// assert!(!java11.supports(&java17_preview)); // Java 11 doesn't support Java 17 features
///
/// // Check if a version is a preview release
/// assert!(java17_preview.is_preview());
/// assert!(!java11.is_preview());
///
/// // Get the display name of the version
/// assert_eq!(java11.to_string(), "Java 11");
///
/// // Serialize and deserialize a version
/// let mut bytes = Vec::new();
/// java11.to_bytes(&mut bytes)?;
///
/// let mut cursor = Cursor::new(bytes);
/// let deserialized = Version::from_bytes(&mut cursor)?;
/// assert_eq!(deserialized, java11);
/// # Ok::<(), ristretto_classfile::Error>(())
/// ```
///
/// #  Reference
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.1>
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd)]
pub enum Version {
    Java1_0_2 { minor: u16 },
    Java1_1 { minor: u16 },
    Java1_2 { minor: u16 },
    Java1_3 { minor: u16 },
    Java1_4 { minor: u16 },
    Java5 { minor: u16 },
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
    /// The major version determines the Java version, while the minor version typically
    /// indicates incremental updates or preview status.
    ///
    /// # Errors
    ///
    /// Returns an error if the major and minor version are invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{JAVA_PREVIEW_MINOR_VERSION, Version};
    ///
    /// // Create Java 17 version
    /// let java17 = Version::from(61, 0)?;
    /// assert_eq!(java17.major(), 61);
    /// assert_eq!(java17.minor(), 0);
    ///
    /// // Create Java 21 preview version
    /// let java21_preview = Version::from(65, JAVA_PREVIEW_MINOR_VERSION)?;
    /// assert!(java21_preview.is_preview());
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn from(major: u16, minor: u16) -> Result<Self> {
        if major >= 56 && minor != 0 && minor != JAVA_PREVIEW_MINOR_VERSION {
            return Err(InvalidVersion { major, minor });
        }

        let version = match major {
            45 => Version::Java1_0_2 { minor },
            46 => Version::Java1_2 { minor },
            47 => Version::Java1_3 { minor },
            48 => Version::Java1_4 { minor },
            49 => Version::Java5 { minor },
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
    ///
    /// The major version corresponds to the Java version according to the `ClassFile` format.
    /// For example, Java 8 has a major version of 52.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::Version;
    ///
    /// let version = Version::from(52, 0)?; // Java 8
    /// assert_eq!(version.major(), 52);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    #[must_use]
    pub fn major(&self) -> u16 {
        match self {
            Version::Java1_0_2 { .. } | Version::Java1_1 { .. } => 45,
            Version::Java1_2 { .. } => 46,
            Version::Java1_3 { .. } => 47,
            Version::Java1_4 { .. } => 48,
            Version::Java5 { .. } => 49,
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
    ///
    /// The minor version is typically 0 for standard releases or 65535 for preview releases.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{Version, JAVA_PREVIEW_MINOR_VERSION};
    ///
    /// let standard = Version::from(61, 0)?; // Java 17
    /// assert_eq!(standard.minor(), 0);
    ///
    /// let preview = Version::from(61, JAVA_PREVIEW_MINOR_VERSION)?; // Java 17 preview
    /// assert_eq!(preview.minor(), JAVA_PREVIEW_MINOR_VERSION);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    #[must_use]
    pub fn minor(&self) -> u16 {
        match self {
            Version::Java1_0_2 { minor, .. }
            | Version::Java1_1 { minor, .. }
            | Version::Java1_2 { minor, .. }
            | Version::Java1_3 { minor, .. }
            | Version::Java1_4 { minor, .. }
            | Version::Java5 { minor, .. }
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
    ///
    /// This converts the internal major version number to the more commonly used
    /// Java version number by subtracting 44.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::Version;
    ///
    /// let version = Version::from(52, 0)?; // Java 8
    /// assert_eq!(version.java(), 8);
    ///
    /// let version = Version::from(61, 0)?; // Java 17
    /// assert_eq!(version.java(), 17);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    #[must_use]
    pub fn java(&self) -> u16 {
        self.major() - 44
    }

    /// Returns true if the current major version supports the given version.
    ///
    /// A Java version supports all earlier versions but not later ones.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::Version;
    ///
    /// let java11 = Version::from(55, 0)?; // Java 11
    /// let java17 = Version::from(61, 0)?; // Java 17
    /// let java21 = Version::from(65, 0)?; // Java 21
    ///
    /// // Java 17 supports Java 11 features
    /// assert!(java17.supports(&java11));
    ///
    /// // Java 17 does not support Java 21 features
    /// assert!(!java17.supports(&java21));
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    #[must_use]
    pub fn supports(&self, version: &Version) -> bool {
        self.major() >= version.major()
    }

    /// Returns true if the current major version is >= Java 12 (`56`) is a preview minor
    /// version (`65535`).
    ///
    /// Preview versions are indicated by a minor version of 65535 (0xFFFF).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{Version, JAVA_PREVIEW_MINOR_VERSION};
    ///
    /// let standard = Version::from(61, 0)?; // Java 17
    /// assert!(!standard.is_preview());
    ///
    /// let preview = Version::from(61, JAVA_PREVIEW_MINOR_VERSION)?; // Java 17 preview
    /// assert!(preview.is_preview());
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    #[must_use]
    pub fn is_preview(&self) -> bool {
        self.major() >= 56 && self.minor() == JAVA_PREVIEW_MINOR_VERSION
    }

    /// Deserialize the major and minor version bytes.
    ///
    /// Reads the version information from a cursor pointing to the binary data
    /// representing the class file's minor and major version.
    ///
    /// ```text
    /// |--------------------- u32 ---------------------|
    /// |--------- u16 ---------|--------- u16 ---------|
    /// |      Minor Ver.       |      Major Ver.       |
    /// |     31 30 29 .. 16    |     15 14 13 .. 0     |
    /// ```
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::Version;
    /// use std::io::Cursor;
    /// use byteorder::{BigEndian, WriteBytesExt};
    ///
    /// // Create a binary representation of Java 11 (major: 55, minor: 0)
    /// let mut buffer = Vec::new();
    /// buffer.write_u16::<BigEndian>(0)?; // minor version
    /// buffer.write_u16::<BigEndian>(55)?; // major version
    ///
    /// let mut cursor = Cursor::new(buffer);
    /// let version = Version::from_bytes(&mut cursor)?;
    ///
    /// assert_eq!(version.major(), 55);
    /// assert_eq!(version.minor(), 0);
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    ///
    /// # Errors
    /// Returns an error if reading from the byte cursor fails or if the version is invalid.
    pub fn from_bytes(bytes: &mut Cursor<Vec<u8>>) -> Result<Version> {
        let minor = bytes.read_u16::<BigEndian>()?;
        let major = bytes.read_u16::<BigEndian>()?;
        Version::from(major, minor)
    }

    /// Serialize the major and minor version to bytes.
    ///
    /// Writes the version information to a vector of bytes according to the `ClassFile` format.
    ///
    /// ```text
    /// |--------------------- u32 ---------------------|
    /// |--------- u16 ---------|--------- u16 ---------|
    /// |      Minor Ver.       |      Major Ver.       |
    /// |     31 30 29 .. 16    |     15 14 13 .. 0     |
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if writing to the byte vector fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::Version;
    /// use std::io::Cursor;
    /// use byteorder::{BigEndian, ReadBytesExt};
    ///
    /// let version = Version::from(55, 0)?; // Java 11
    /// let mut bytes = Vec::new();
    /// version.to_bytes(&mut bytes)?;
    ///
    /// // The bytes should represent minor version (0) followed by major version (55)
    /// let mut cursor = Cursor::new(bytes);
    /// assert_eq!(cursor.read_u16::<BigEndian>()?, 0); // minor version
    /// assert_eq!(cursor.read_u16::<BigEndian>()?, 55); // major version
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        bytes.write_u16::<BigEndian>(self.minor())?;
        bytes.write_u16::<BigEndian>(self.major())?;
        Ok(())
    }
}

impl Default for Version {
    /// Returns the default version, which is Java 1.0.2 with minor version 0.
    ///
    /// This is useful when you need to initialize a `Version` with the earliest supported Java
    /// version.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::Version;
    ///
    /// // Create a default Version
    /// let version = Version::default();
    ///
    /// // The default version is Java 1.0.2
    /// assert_eq!(version.to_string(), "Java 1.0.2");
    /// assert_eq!(version.major(), 45);
    /// assert_eq!(version.minor(), 0);
    /// ```
    fn default() -> Self {
        Version::Java1_0_2 { minor: 0 }
    }
}

impl fmt::Display for Version {
    /// Formats the Version as a human-readable string.
    ///
    /// The version is displayed as "Java X" where X is the Java version number.
    /// For older versions (1.0.2 through 5.0), the format follows the historical
    /// naming convention with decimals.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::Version;
    ///
    /// let java1_2 = Version::from(46, 0)?;
    /// assert_eq!(java1_2.to_string(), "Java 1.2");
    ///
    /// let java5 = Version::from(49, 0)?;
    /// assert_eq!(java5.to_string(), "Java 5");
    ///
    /// let java8 = Version::from(52, 0)?;
    /// assert_eq!(java8.to_string(), "Java 8");
    ///
    /// # Ok::<(), ristretto_classfile::Error>(())
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Version::Java1_0_2 { .. } => write!(f, "Java 1.0.2"),
            Version::Java1_1 { .. } => write!(f, "Java 1.1"),
            Version::Java1_2 { .. } => write!(f, "Java 1.2"),
            Version::Java1_3 { .. } => write!(f, "Java 1.3"),
            Version::Java1_4 { .. } => write!(f, "Java 1.4"),
            Version::Java5 { .. } => write!(f, "Java 5"),
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

/// Specifies a rule for matching Java versions.
///
/// This enum allows defining conditions for comparing a runtime Java version's major number against
/// specific criteria.
///
/// # Examples
///
/// ```rust
/// use ristretto_classfile::{Version, VersionSpecification, JAVA_17, JAVA_21};
///
/// // Matches any Java version
/// assert!(VersionSpecification::Any.matches(&JAVA_21));
///
/// // Matches if the runtime Java version's major is equal to 11
/// assert!(VersionSpecification::Equal(JAVA_21.clone()).matches(&JAVA_21));
///
/// // Matches if the runtime Java version's major is less than 21
/// assert!(VersionSpecification::LessThan(JAVA_21.clone()).matches(&JAVA_17));
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum VersionSpecification {
    /// Matches any Java version.
    Any,
    /// Matches if the runtime Java version's major is equal to the specified version's major.
    Equal(Version),
    /// Matches if the runtime Java version's major is not equal to the specified version's major.
    NotEqual(Version),
    /// Matches if the runtime Java version's major is less than the specified version's major.
    LessThan(Version),
    /// Matches if the runtime Java version's major is less than or equal to the specified version's
    /// major.
    LessThanOrEqual(Version),
    /// Matches if the runtime Java version's major is greater than the specified version's major.
    GreaterThan(Version),
    /// Matches if the runtime Java version's major is greater than or equal to the specified
    /// version's major.
    GreaterThanOrEqual(Version),
    /// Matches if the runtime Java version's major is between the specified versions' majors
    /// (inclusive).
    Between(Version, Version),
    /// Matches if the runtime Java version's major is in the specified list of versions.
    In(&'static [Version]),
}

impl VersionSpecification {
    /// Checks if a given runtime `Version` matches the specification.
    ///
    /// Comparisons are based on the major version number.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ristretto_classfile::{Version, VersionSpecification, JAVA_17, JAVA_21};
    ///
    /// // Matches any Java version
    /// assert!(VersionSpecification::Any.matches(&JAVA_21));
    ///
    /// // Matches if the runtime Java version's major is equal to 11
    /// assert!(VersionSpecification::Equal(JAVA_21.clone()).matches(&JAVA_21));
    ///
    /// // Matches if the runtime Java version's major is less than 21
    /// assert!(VersionSpecification::LessThan(JAVA_21.clone()).matches(&JAVA_17));
    /// ```
    #[must_use]
    pub fn matches(&self, version: &Version) -> bool {
        let major = version.major();
        match self {
            VersionSpecification::Any => true,
            VersionSpecification::Equal(specification) => major == specification.major(),
            VersionSpecification::NotEqual(specification) => major != specification.major(),
            VersionSpecification::LessThan(specification) => major < specification.major(),
            VersionSpecification::LessThanOrEqual(specification) => major <= specification.major(),
            VersionSpecification::GreaterThan(specification) => major > specification.major(),
            VersionSpecification::GreaterThanOrEqual(specification) => {
                major >= specification.major()
            }
            VersionSpecification::Between(start_version, end_version) => {
                major >= start_version.major() && major <= end_version.major()
            }
            VersionSpecification::In(versions) => versions.iter().any(|v| v.major() == major),
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
            JAVA_1_0_2, JAVA_1_1, JAVA_1_2, JAVA_1_3, JAVA_1_4, JAVA_5, JAVA_6, JAVA_7, JAVA_8,
            JAVA_9, JAVA_10, JAVA_11, JAVA_12, JAVA_13, JAVA_14, JAVA_15, JAVA_16, JAVA_17,
            JAVA_18, JAVA_19, JAVA_20, JAVA_21, JAVA_22, JAVA_23, JAVA_24, JAVA_25,
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
        assert_eq!(JAVA_21.major(), 65);
    }

    #[test]
    fn test_minor() {
        let minor = 3;
        let version = Version::Java11 { minor };
        assert_eq!(version.minor(), minor);
    }

    #[test]
    fn test_supports() {
        assert!(JAVA_11.supports(&JAVA_5));
        assert!(!JAVA_5.supports(&JAVA_11));
    }

    #[test]
    fn test_is_preview() {
        assert!(!JAVA_11.is_preview());
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
        assert_eq!(version, JAVA_1_0_2);
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

    #[test]
    fn test_version_specification() {
        assert!(VersionSpecification::Any.matches(&JAVA_11));

        assert!(VersionSpecification::Equal(JAVA_11.clone()).matches(&JAVA_11));
        assert!(!VersionSpecification::Equal(JAVA_17.clone()).matches(&JAVA_11));

        assert!(!VersionSpecification::NotEqual(JAVA_11.clone()).matches(&JAVA_11));
        assert!(VersionSpecification::NotEqual(JAVA_17.clone()).matches(&JAVA_11));

        assert!(VersionSpecification::LessThan(JAVA_17.clone()).matches(&JAVA_11));
        assert!(!VersionSpecification::LessThan(JAVA_11.clone()).matches(&JAVA_11));

        assert!(VersionSpecification::LessThanOrEqual(JAVA_11.clone()).matches(&JAVA_11));
        assert!(VersionSpecification::LessThanOrEqual(JAVA_17.clone()).matches(&JAVA_11));
        assert!(!VersionSpecification::LessThanOrEqual(JAVA_17.clone()).matches(&JAVA_21));

        assert!(VersionSpecification::GreaterThan(JAVA_11.clone()).matches(&JAVA_17));
        assert!(!VersionSpecification::GreaterThan(JAVA_17.clone()).matches(&JAVA_17));

        assert!(VersionSpecification::GreaterThanOrEqual(JAVA_11.clone()).matches(&JAVA_11));
        assert!(VersionSpecification::GreaterThanOrEqual(JAVA_17.clone()).matches(&JAVA_21));
        assert!(!VersionSpecification::GreaterThanOrEqual(JAVA_21.clone()).matches(&JAVA_17));

        let between = VersionSpecification::Between(JAVA_11, JAVA_17);
        assert!(between.matches(&JAVA_11));
        assert!(between.matches(&JAVA_17));
        assert!(!between.matches(&JAVA_21));

        let versions_in = VersionSpecification::In(&[JAVA_11, JAVA_21]);
        assert!(versions_in.matches(&JAVA_11));
        assert!(versions_in.matches(&JAVA_21));
        assert!(!versions_in.matches(&JAVA_17));
    }
}
