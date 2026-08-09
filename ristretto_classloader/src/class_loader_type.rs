/// The role of a built-in class loader in the JVM.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClassLoaderType {
    /// The bootstrap class loader, which is the parent of all class loaders.
    Bootstrap,
    /// The platform class loader, which is responsible for loading platform classes.
    Platform,
    /// The system/application class loader, which is the default class loader for the JVM.
    System,
}

impl ClassLoaderType {
    /// Returns the Java visible name of the built-in class loader.
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            ClassLoaderType::Bootstrap => "bootstrap",
            ClassLoaderType::Platform => "platform",
            ClassLoaderType::System => "app",
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
        assert_eq!(ClassLoaderType::System.name(), "app");
    }
}
