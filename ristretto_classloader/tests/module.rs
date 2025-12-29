//! Integration tests for the module system.

use ristretto_classloader::module::{
    AccessCheckResult, Exports, ModuleDescriptor, ModuleFlags, Opens, Provides, Requires,
    RequiresFlags,
};
use std::collections::BTreeSet;

#[test]
fn test_module_descriptor_new() {
    let desc = ModuleDescriptor::new("com.example.mymodule".to_string());
    assert_eq!(desc.name, "com.example.mymodule");
    assert!(!desc.is_open());
    assert!(desc.requires.is_empty());
    assert!(desc.exports.is_empty());
}

#[test]
fn test_module_descriptor_open() {
    let mut desc = ModuleDescriptor::new("com.example.open".to_string());
    desc.flags = ModuleFlags::OPEN;
    assert!(desc.is_open());
}

#[test]
fn test_requires_transitive() {
    let req = Requires {
        name: "java.logging".to_string(),
        flags: RequiresFlags::TRANSITIVE,
        version: None,
    };
    assert!(req.is_transitive());
    assert!(!req.is_static());
    assert!(!req.is_mandated());
}

#[test]
fn test_requires_static() {
    let req = Requires {
        name: "java.compiler".to_string(),
        flags: RequiresFlags::STATIC_PHASE,
        version: None,
    };
    assert!(!req.is_transitive());
    assert!(req.is_static());
}

#[test]
fn test_exports_unqualified() {
    let exp = Exports {
        package: "com/example/api".to_string(),
        targets: None,
    };
    assert!(!exp.is_qualified());
    assert!(exp.exports_to("any.module"));
    assert!(exp.exports_to("other.module"));
}

#[test]
fn test_exports_qualified() {
    let exp = Exports {
        package: "com/example/internal".to_string(),
        targets: Some(vec!["com.example.impl".to_string()]),
    };
    assert!(exp.is_qualified());
    assert!(exp.exports_to("com.example.impl"));
    assert!(!exp.exports_to("com.example.other"));
}

#[test]
fn test_opens_unqualified() {
    let opens = Opens {
        package: "com/example/reflect".to_string(),
        targets: None,
    };
    assert!(!opens.is_qualified());
    assert!(opens.opens_to("any.module"));
}

#[test]
fn test_opens_qualified() {
    let opens = Opens {
        package: "com/example/internal".to_string(),
        targets: Some(vec!["com.example.test".to_string()]),
    };
    assert!(opens.is_qualified());
    assert!(opens.opens_to("com.example.test"));
    assert!(!opens.opens_to("com.example.other"));
}

#[test]
fn test_provides() {
    let provides = Provides {
        service: "java/util/spi/LocaleServiceProvider".to_string(),
        implementations: vec!["com/example/MyLocaleProvider".to_string()],
    };
    assert_eq!(provides.service, "java/util/spi/LocaleServiceProvider");
    assert_eq!(provides.implementations.len(), 1);
}

#[test]
fn test_module_descriptor_exports_package() {
    let mut desc = ModuleDescriptor::new("com.example".to_string());
    desc.exports.push(Exports {
        package: "com/example/api".to_string(),
        targets: None,
    });
    desc.exports.push(Exports {
        package: "com/example/internal".to_string(),
        targets: Some(vec!["com.example.impl".to_string()]),
    });

    // Unqualified export
    assert!(desc.exports_package("com/example/api", None));
    assert!(desc.exports_package("com/example/api", Some("any.module")));

    // Qualified export
    assert!(!desc.exports_package("com/example/internal", None));
    assert!(desc.exports_package("com/example/internal", Some("com.example.impl")));
    assert!(!desc.exports_package("com/example/internal", Some("other.module")));

    // Non-exported package
    assert!(!desc.exports_package("com/example/private", None));
}

#[test]
fn test_module_descriptor_opens_package() {
    let mut desc = ModuleDescriptor::new("com.example".to_string());
    desc.packages.insert("com/example/reflect".to_string());
    desc.packages.insert("com/example/internal".to_string());

    desc.opens.push(Opens {
        package: "com/example/reflect".to_string(),
        targets: None,
    });
    desc.opens.push(Opens {
        package: "com/example/internal".to_string(),
        targets: Some(vec!["com.example.test".to_string()]),
    });

    // Unqualified open
    assert!(desc.opens_package("com/example/reflect", None));
    assert!(desc.opens_package("com/example/reflect", Some("any.module")));

    // Qualified open
    assert!(!desc.opens_package("com/example/internal", None));
    assert!(desc.opens_package("com/example/internal", Some("com.example.test")));
}

#[test]
fn test_open_module_opens_all_packages() {
    let mut desc = ModuleDescriptor::new("com.example.open".to_string());
    desc.flags = ModuleFlags::OPEN;
    desc.packages.insert("com/example/open/pkg1".to_string());
    desc.packages.insert("com/example/open/pkg2".to_string());

    // Open modules implicitly open all packages
    assert!(desc.opens_package("com/example/open/pkg1", None));
    assert!(desc.opens_package("com/example/open/pkg2", Some("any.module")));

    // But non-existent packages are not opened
    assert!(!desc.opens_package("com/example/open/nonexistent", None));
}

#[test]
fn test_automatic_module_from_jar_name() {
    let packages: BTreeSet<String> = ["com/example/lib".to_string()].into_iter().collect();

    let desc =
        ModuleDescriptor::automatic_from_jar_name("my-library-1.0.0.jar", None, packages.clone())
            .expect("should create automatic module");

    assert_eq!(desc.name, "my.library");

    // Automatic modules export all packages
    assert!(desc.exports_package("com/example/lib", None));

    // Automatic modules open all packages
    assert!(desc.opens_package("com/example/lib", None));

    // Automatic modules require java.base
    assert!(desc.requires_module("java.base"));
}

#[test]
fn test_automatic_module_with_manifest_name() {
    let packages: BTreeSet<String> = ["com/example/lib".to_string()].into_iter().collect();

    let desc = ModuleDescriptor::automatic_from_jar_name(
        "ugly-jar-name.jar",
        Some("com.example.beautiful.name"),
        packages,
    )
    .expect("should create automatic module");

    // Manifest name takes precedence
    assert_eq!(desc.name, "com.example.beautiful.name");
}

#[test]
fn test_automatic_module_name_derivation() {
    // Test various JAR name patterns
    let test_cases = [
        ("foo.jar", "foo"),
        ("foo-bar.jar", "foo.bar"),
        ("foo-bar-1.0.jar", "foo.bar"),
        ("foo-bar-1.0.0.jar", "foo.bar"),
        ("foo_bar.jar", "foo.bar"),
        ("foo-bar-baz.jar", "foo.bar.baz"),
    ];

    for (jar_name, expected) in test_cases {
        let packages = BTreeSet::new();
        let desc = ModuleDescriptor::automatic_from_jar_name(jar_name, None, packages)
            .unwrap_or_else(|_| panic!("should create automatic module from {jar_name}"));
        assert_eq!(
            desc.name, expected,
            "JAR {jar_name} should derive module name {expected}"
        );
    }
}

#[test]
fn test_access_check_result() {
    assert!(AccessCheckResult::Allowed.is_allowed());
    assert!(!AccessCheckResult::Allowed.is_denied());

    assert!(!AccessCheckResult::NotReadable.is_allowed());
    assert!(AccessCheckResult::NotReadable.is_denied());

    assert!(!AccessCheckResult::NotExported.is_allowed());
    assert!(AccessCheckResult::NotExported.is_denied());

    assert!(!AccessCheckResult::NotOpened.is_allowed());
    assert!(AccessCheckResult::NotOpened.is_denied());
}
