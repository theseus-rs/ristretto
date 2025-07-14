/// Enum representing the different types of class loaders in the JVM.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClassLoaderType {
    /// The bootstrap class loader, which is the parent of all class loaders.
    Bootstrap,
    /// The platform class loader, which is responsible for loading platform classes.
    Platform,
    /// The system class loader, which is the default class loader for the JVM.
    System,
}

impl ClassLoaderType {
    /// Returns the name of the class loader type.
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            ClassLoaderType::Bootstrap => "bootstrap",
            ClassLoaderType::Platform => "platform",
            ClassLoaderType::System => "system",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class_loader_type() {
        assert_eq!(ClassLoaderType::Bootstrap.name(), "bootstrap");
        assert_eq!(ClassLoaderType::Platform.name(), "platform");
        assert_eq!(ClassLoaderType::System.name(), "system");
    }
}
