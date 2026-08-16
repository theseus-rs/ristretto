/// A named Java source file supplied directly from memory.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JavaSource {
    /// The binary name of the top-level class, such as `com.example.Main`.
    pub class_name: String,
    /// The Java source text.
    pub source: String,
}

impl JavaSource {
    /// Create a named in-memory Java source file.
    pub fn new<N, S>(class_name: N, source: S) -> Self
    where
        N: Into<String>,
        S: Into<String>,
    {
        Self {
            class_name: class_name.into(),
            source: source.into(),
        }
    }
}
