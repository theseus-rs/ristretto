//! Method lookup and receiver selection over linked class hierarchies.

use crate::{Class, Method, Value};
use ristretto_classfile::MethodAccessFlags;
use std::collections::HashSet;
use std::sync::Arc;

/// A method together with the class whose constant pool must be used to execute it.
#[derive(Clone, Debug)]
pub struct ResolvedMethod {
    /// The class that declares the method.
    pub declaring_class: Arc<Class>,
    /// The resolved or selected method.
    pub method: Arc<Method>,
}

/// Failures during method lookup or receiver selection.
///
/// The runtime translates these into Java linkage errors without requiring the classloader
/// to depend on the VM's error types.
#[derive(Debug, thiserror::Error)]
pub enum MethodResolutionError {
    /// The requested method does not exist.
    #[error("{0}")]
    NoSuchMethod(String),
    /// An instance call targets a static method, or interface defaults conflict.
    #[error("{0}")]
    IncompatibleClassChange(String),
    /// No concrete implementation exists for the receiver.
    #[error("{0}")]
    AbstractMethod(String),
    /// Class metadata could not be read.
    #[error(transparent)]
    ClassLoaderError(#[from] crate::Error),
}

impl Class {
    /// Resolve a method in this linked class hierarchy, preserving its declaring class.
    /// Unlike [`Class::method`], this searches ancestors and interface defaults.
    /// Constructors and class initializers are never inherited.
    ///
    /// # Errors
    /// Returns an error when no method exists or metadata is unavailable. Interface
    /// default conflicts are deferred to selection, just as for interface references.
    pub fn resolve_method(
        self: &Arc<Self>,
        name: &str,
        descriptor: &str,
    ) -> Result<ResolvedMethod, MethodResolutionError> {
        if self.is_interface() {
            return self.resolve_interface_method(name, descriptor);
        }
        let mut current = Some(self.clone());
        while let Some(candidate) = current {
            if let Some(method) = candidate.method(name, descriptor) {
                return Ok(ResolvedMethod {
                    declaring_class: candidate,
                    method,
                });
            }
            if name == "<init>" || name == "<clinit>" {
                break;
            }
            current = candidate.parent()?;
        }
        if name != "<init>"
            && name != "<clinit>"
            && let Some(target) = interface_resolution_candidate(self, name, descriptor)?
        {
            return Ok(target);
        }
        Err(MethodResolutionError::NoSuchMethod(format!(
            "Method {name}{descriptor} not found in class {}",
            self.name()
        )))
    }

    /// Resolve an interface method, including Object methods and maximally specific
    /// superinterfaces in the linked hierarchy. Conflicting defaults are deferred to
    /// receiver selection, where class implementations take precedence.
    ///
    /// # Errors
    /// Returns an error when no method exists or metadata is unavailable.
    pub fn resolve_interface_method(
        self: &Arc<Self>,
        name: &str,
        descriptor: &str,
    ) -> Result<ResolvedMethod, MethodResolutionError> {
        if let Some(method) = self.method(name, descriptor) {
            return Ok(ResolvedMethod {
                declaring_class: self.clone(),
                method,
            });
        }
        if let Some(object) = self.parent()?
            && let Some(method) = object.method(name, descriptor)
            && method.is_public()
            && !method.is_static()
        {
            return Ok(ResolvedMethod {
                declaring_class: object,
                method,
            });
        }
        if let Some(target) = interface_resolution_candidate(self, name, descriptor)? {
            return Ok(target);
        }
        Err(MethodResolutionError::NoSuchMethod(format!(
            "Method {name}{descriptor} not found in interface {}",
            self.name()
        )))
    }

    /// Select a resolved instance method for this receiver class in its linked hierarchy.
    /// Private methods remain bound to their declaring class; static methods are not overrides.
    /// Abstract targets are returned so callers can check access before rejecting them.
    /// Callers without those checks can use [`Class::select_concrete_method`].
    ///
    /// # Errors
    /// Returns an error for static, missing, or conflicting targets, or unavailable metadata.
    pub fn select_method(
        self: &Arc<Self>,
        resolved: &ResolvedMethod,
    ) -> Result<ResolvedMethod, MethodResolutionError> {
        if resolved.method.is_static() {
            return Err(MethodResolutionError::IncompatibleClassChange(format!(
                "Method {}.{} is static",
                resolved.declaring_class.name(),
                resolved.method.name()
            )));
        }
        if resolved.method.is_private() {
            return Ok(resolved.clone());
        }
        let mut hierarchy = Vec::new();
        let mut current = Some(self.clone());
        while let Some(class) = current {
            current = class.parent()?;
            hierarchy.push(class);
        }
        // Keep all eligible declarations: an intervening package-private method must not
        // hide an earlier public override when checking transitive overriding.
        let mut overrides: Vec<ResolvedMethod> = Vec::new();
        for class in hierarchy.into_iter().rev() {
            let Some(method) = class.method(resolved.method.name(), resolved.method.descriptor())
            else {
                continue;
            };
            if method.is_static() || method.is_private() {
                continue;
            }
            let mut eligible = can_override(&class, &resolved.declaring_class, &resolved.method)?;
            if !eligible {
                for ancestor in &overrides {
                    if can_override(&class, &ancestor.declaring_class, &ancestor.method)? {
                        eligible = true;
                        break;
                    }
                }
            }
            if eligible {
                overrides.push(ResolvedMethod {
                    declaring_class: class,
                    method,
                });
            }
        }
        if let Some(target) = overrides.pop() {
            return Ok(target);
        }
        if let Some(target) =
            interface_method(self, resolved.method.name(), resolved.method.descriptor())?
        {
            return Ok(target);
        }
        Err(MethodResolutionError::AbstractMethod(format!(
            "Method {}{} has no implementation in {}",
            resolved.method.name(),
            resolved.method.descriptor(),
            self.name()
        )))
    }

    /// Select a concrete instance method for a caller with no further access checks.
    ///
    /// # Errors
    /// Returns an error for static, abstract, missing, or conflicting targets, or unavailable
    /// metadata.
    pub fn select_concrete_method(
        self: &Arc<Self>,
        resolved: &ResolvedMethod,
    ) -> Result<ResolvedMethod, MethodResolutionError> {
        concrete_method(self.select_method(resolved)?)
    }

    /// Select an `invokespecial` target from the symbolic class or interface.
    /// Superclass calls start at the caller's direct superclass under `ACC_SUPER`;
    /// receiver overrides never participate in this selection.
    ///
    /// # Errors
    /// Returns an error for static, abstract, missing, or conflicting targets, or
    /// unavailable metadata. The caller must check for a null receiver first.
    pub fn select_special_method(
        self: &Arc<Self>,
        caller: &Arc<Class>,
        resolved: &ResolvedMethod,
    ) -> Result<ResolvedMethod, MethodResolutionError> {
        let name = resolved.method.name();
        let descriptor = resolved.method.descriptor();
        if resolved.method.is_static() {
            return Err(MethodResolutionError::IncompatibleClassChange(format!(
                "Method {}.{name} is static",
                resolved.declaring_class.name()
            )));
        }
        if name == "<init>" {
            return concrete_method(resolved.clone());
        }
        let mut lookup = self.clone();
        // Java 8+ treats ACC_SUPER as set for every class, including older files.
        if !self.is_interface()
            && let Some(parent) = caller.parent()?
        {
            let mut current = Some(parent.clone());
            while let Some(ancestor) = current {
                if Arc::ptr_eq(&ancestor, self) {
                    lookup = parent;
                    break;
                }
                current = ancestor.parent()?;
            }
        }
        let mut current = Some(lookup.clone());
        while let Some(class) = current {
            if let Some(method) = class.method(name, descriptor)
                && !method.is_static()
                && (!lookup.is_interface() || Arc::ptr_eq(&class, &lookup) || method.is_public())
            {
                return concrete_method(ResolvedMethod {
                    declaring_class: class,
                    method,
                });
            }
            current = class.parent()?;
        }
        if let Some(target) = interface_method(&lookup, name, descriptor)? {
            return concrete_method(target);
        }
        Err(MethodResolutionError::AbstractMethod(format!(
            "Method {name}{descriptor} has no implementation in {}",
            lookup.name()
        )))
    }

    /// Determine whether two classes have the same package name and defining loader.
    /// Java class mirror identities take precedence over Rust loader identities because
    /// user-defined Java loaders can share the same Rust loader.
    ///
    /// # Errors
    /// Returns an error if class metadata or a class mirror's loader cannot be read.
    pub fn same_runtime_package(&self, other: &Class) -> crate::Result<bool> {
        if package_name(self.name()) != package_name(other.name()) {
            return Ok(false);
        }
        if let (Some(left), Some(right)) = (self.object()?, other.object()?) {
            let left = left.as_object_ref()?.value("classLoader")?;
            let right = right.as_object_ref()?.value("classLoader")?;
            return Ok(match (&left, &right) {
                (Value::Object(None), Value::Object(None)) => true,
                (Value::Object(Some(left)), Value::Object(Some(right))) => left.ptr_eq(right),
                _ => false,
            });
        }
        Ok(match (self.class_loader()?, other.class_loader()?) {
            (None, None) => true,
            (Some(left), Some(right)) => Arc::ptr_eq(&left, &right),
            _ => false,
        })
    }
}

fn concrete_method(target: ResolvedMethod) -> Result<ResolvedMethod, MethodResolutionError> {
    if target.method.is_abstract() {
        return Err(MethodResolutionError::AbstractMethod(format!(
            "Method {}.{} is abstract",
            target.declaring_class.name(),
            target.method.name()
        )));
    }
    Ok(target)
}

fn package_name(name: &str) -> &str {
    let name = if name.starts_with('[') {
        name.trim_start_matches('[')
            .strip_prefix('L')
            .unwrap_or(name)
            .trim_end_matches(';')
    } else {
        name
    };
    name.rsplit_once('/').map_or("", |(package, _)| package)
}

fn can_override(
    class: &Class,
    ancestor: &Class,
    method: &Method,
) -> Result<bool, MethodResolutionError> {
    if method.is_public() || method.access_flags().contains(MethodAccessFlags::PROTECTED) {
        return Ok(true);
    }
    if method.is_private() {
        return Ok(false);
    }
    Ok(class.same_runtime_package(ancestor)?)
}

fn interface_resolution_candidate(
    class: &Arc<Class>,
    name: &str,
    descriptor: &str,
) -> Result<Option<ResolvedMethod>, MethodResolutionError> {
    let candidates = maximally_specific_interface_methods(class, name, descriptor)?;
    // JVMS 5.4.3.3 and 5.4.3.4 prefer the unique non-abstract candidate when one
    // exists. Otherwise any eligible declaration can be resolved, even when
    // defaults conflict. Invocation selection checks the actual call target.
    Ok(candidates
        .iter()
        .find(|candidate| !candidate.method.is_abstract())
        .or_else(|| candidates.first())
        .cloned())
}

fn interface_method(
    class: &Arc<Class>,
    name: &str,
    descriptor: &str,
) -> Result<Option<ResolvedMethod>, MethodResolutionError> {
    let candidates = maximally_specific_interface_methods(class, name, descriptor)?;
    let mut concrete = candidates
        .iter()
        .filter(|candidate| !candidate.method.is_abstract());
    if let Some(target) = concrete.next() {
        if concrete.next().is_some() {
            return Err(MethodResolutionError::IncompatibleClassChange(format!(
                "Conflicting interface defaults for {name}{descriptor} in {}",
                class.name()
            )));
        }
        return Ok(Some(target.clone()));
    }
    Ok(candidates.into_iter().next())
}

fn maximally_specific_interface_methods(
    class: &Arc<Class>,
    name: &str,
    descriptor: &str,
) -> Result<Vec<ResolvedMethod>, MethodResolutionError> {
    let mut interfaces = Vec::new();
    let mut current = Some(class.clone());
    while let Some(class) = current {
        interfaces.extend(class.interfaces()?);
        current = class.parent()?;
    }
    let mut visited = HashSet::new();
    let mut candidates = Vec::new();
    while let Some(interface) = interfaces.pop() {
        if !visited.insert(Arc::as_ptr(&interface)) {
            continue;
        }
        interfaces.extend(interface.interfaces()?);
        if let Some(method) = interface.method(name, descriptor)
            && !method.is_private()
            && !method.is_static()
        {
            candidates.push(ResolvedMethod {
                declaring_class: interface,
                method,
            });
        }
    }
    let mut shadowed = HashSet::new();
    for candidate in &candidates {
        let mut parents = candidate.declaring_class.interfaces()?;
        while let Some(parent) = parents.pop() {
            if shadowed.insert(Arc::as_ptr(&parent)) {
                parents.extend(parent.interfaces()?);
            }
        }
    }
    candidates.retain(|candidate| !shadowed.contains(&Arc::as_ptr(&candidate.declaring_class)));
    Ok(candidates)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ClassLoader, ClassPath};
    use ristretto_classfile::{ClassAccessFlags, ClassFile, ConstantPool};

    fn class(
        name: &str,
        parent: Option<&Arc<Class>>,
        loader: Option<&Arc<ClassLoader>>,
        methods: &[(&str, MethodAccessFlags)],
    ) -> crate::Result<Arc<Class>> {
        let mut constant_pool = ConstantPool::new();
        let this_class = constant_pool.add_class(name)?;
        let super_class = parent.map_or(Ok(0), |parent| constant_pool.add_class(parent.name()))?;
        let methods = methods
            .iter()
            .map(|(name, flags)| {
                Ok(ristretto_classfile::Method {
                    access_flags: *flags,
                    name_index: constant_pool.add_utf8(name)?,
                    descriptor_index: constant_pool.add_utf8("()V")?,
                    ..Default::default()
                })
            })
            .collect::<crate::Result<Vec<_>>>()?;
        let class = Class::from(
            loader.map(Arc::downgrade),
            ClassFile {
                access_flags: ClassAccessFlags::PUBLIC,
                constant_pool,
                this_class,
                super_class,
                methods,
                ..Default::default()
            },
        )?;
        class.set_parent(parent.cloned())?;
        Ok(class)
    }

    #[test]
    fn resolution_preserves_owner_without_changing_declaration_lookup()
    -> Result<(), MethodResolutionError> {
        let base = class(
            "Base",
            None,
            None,
            &[
                ("run", MethodAccessFlags::PUBLIC),
                ("<init>", MethodAccessFlags::PUBLIC),
                ("<clinit>", MethodAccessFlags::STATIC),
            ],
        )?;
        let leaf = class("Leaf", Some(&base), None, &[])?;
        assert!(leaf.method("run", "()V").is_none());
        assert!(leaf.try_get_method("run", "()V").is_err());
        let resolved = leaf.resolve_method("run", "()V")?;
        assert!(Arc::ptr_eq(&resolved.declaring_class, &base));
        assert!(Arc::ptr_eq(
            &resolved.method,
            &base.try_get_method("run", "()V")?
        ));
        for name in ["<init>", "<clinit>", "missing"] {
            assert!(matches!(
                leaf.resolve_method(name, "()V"),
                Err(MethodResolutionError::NoSuchMethod(_))
            ));
        }
        Ok(())
    }

    #[test]
    fn selection_can_defer_abstract_target_rejection() -> Result<(), MethodResolutionError> {
        let base = class("Base", None, None, &[("run", MethodAccessFlags::PUBLIC)])?;
        let receiver = class(
            "AbstractReceiver",
            Some(&base),
            None,
            &[(
                "run",
                MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT,
            )],
        )?;
        let resolved = base.resolve_method("run", "()V")?;
        let selected = receiver.select_method(&resolved)?;
        assert!(Arc::ptr_eq(&selected.declaring_class, &receiver));
        assert!(selected.method.is_abstract());
        assert!(matches!(
            receiver.select_concrete_method(&resolved),
            Err(MethodResolutionError::AbstractMethod(_))
        ));
        Ok(())
    }

    fn interface(
        name: &str,
        parents: Vec<Arc<Class>>,
        methods: &[(&str, MethodAccessFlags)],
    ) -> crate::Result<Arc<Class>> {
        let mut class_file = class(name, None, None, methods)?.class_file().clone();
        class_file.access_flags |= ClassAccessFlags::INTERFACE | ClassAccessFlags::ABSTRACT;
        let interface = Class::from(None, class_file)?;
        interface.set_interfaces(parents)?;
        Ok(interface)
    }

    #[test]
    fn interface_resolution_defers_default_conflicts_to_selection()
    -> Result<(), MethodResolutionError> {
        let methods = &[("run", MethodAccessFlags::PUBLIC)];
        let first = interface("First", vec![], methods)?;
        let second = interface("Second", vec![], methods)?;
        let combined = interface("Combined", vec![first.clone(), second.clone()], &[])?;
        let resolved = combined.resolve_method("run", "()V")?;
        assert!(
            Arc::ptr_eq(&resolved.declaring_class, &first)
                || Arc::ptr_eq(&resolved.declaring_class, &second)
        );

        let direct = class("Direct", None, None, methods)?;
        direct.set_interfaces(vec![combined.clone()])?;
        let base = class("Base", None, None, methods)?;
        let inherited = class("Inherited", Some(&base), None, &[])?;
        inherited.set_interfaces(vec![combined.clone()])?;
        for (receiver, owner) in [(&direct, &direct), (&inherited, &base)] {
            let selected = receiver.select_concrete_method(&resolved)?;
            assert!(Arc::ptr_eq(&selected.declaring_class, owner));
        }

        let missing = class("Missing", None, None, &[])?;
        missing.set_interfaces(vec![combined])?;
        assert!(matches!(
            missing.select_method(&resolved),
            Err(MethodResolutionError::IncompatibleClassChange(_))
        ));
        Ok(())
    }

    #[test]
    fn interface_resolution_prefers_the_unique_default() -> Result<(), MethodResolutionError> {
        let concrete = interface("Concrete", vec![], &[("run", MethodAccessFlags::PUBLIC)])?;
        let abstract_interface = interface(
            "Abstract",
            vec![],
            &[(
                "run",
                MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT,
            )],
        )?;
        let combined = interface("Combined", vec![concrete.clone(), abstract_interface], &[])?;
        let resolved = combined.resolve_interface_method("run", "()V")?;
        assert!(Arc::ptr_eq(&resolved.declaring_class, &concrete));
        Ok(())
    }

    #[test]
    fn class_resolution_defers_default_conflicts_to_selection() -> Result<(), MethodResolutionError>
    {
        let methods = &[("run", MethodAccessFlags::PUBLIC)];
        let first = interface("First", vec![], methods)?;
        let second = interface("Second", vec![], methods)?;
        let base = class("Base", None, None, &[])?;
        base.set_interfaces(vec![first, second])?;
        let child = class("Child", Some(&base), None, methods)?;
        let resolved = base.resolve_method("run", "()V")?;
        let selected = child.select_concrete_method(&resolved)?;
        assert!(Arc::ptr_eq(&selected.declaring_class, &child));
        assert!(matches!(
            base.select_concrete_method(&resolved),
            Err(MethodResolutionError::IncompatibleClassChange(_))
        ));
        assert!(matches!(
            base.select_special_method(&child, &resolved),
            Err(MethodResolutionError::IncompatibleClassChange(_))
        ));
        Ok(())
    }

    #[test]
    fn special_selection_uses_the_direct_superclass() -> Result<(), MethodResolutionError> {
        let methods = &[("run", MethodAccessFlags::PUBLIC)];
        let grandparent = class("Grandparent", None, None, methods)?;
        let parent = class("Parent", Some(&grandparent), None, methods)?;
        let resolved = grandparent.resolve_method("run", "()V")?;
        for (version, flags) in [
            (ristretto_classfile::JAVA_1_1, ClassAccessFlags::empty()),
            (ristretto_classfile::JAVA_1_1, ClassAccessFlags::SUPER),
            (ristretto_classfile::JAVA_8, ClassAccessFlags::empty()),
        ] {
            let mut definition = class("Caller", None, None, methods)?.class_file().clone();
            definition.access_flags |= flags;
            definition.version = version;
            let caller = Class::from(None, definition)?;
            caller.set_parent(Some(parent.clone()))?;
            let selected = grandparent.select_special_method(&caller, &resolved)?;
            assert!(Arc::ptr_eq(&selected.declaring_class, &parent));
        }
        Ok(())
    }

    #[test]
    fn special_selection_checks_interface_conflicts_and_abstract_redeclarations()
    -> Result<(), MethodResolutionError> {
        let methods = &[("run", MethodAccessFlags::PUBLIC)];
        let first = interface("First", vec![], methods)?;
        let second = interface("Second", vec![], methods)?;
        let combined = interface("Combined", vec![first.clone(), second], &[])?;
        let caller = class("Caller", None, None, methods)?;
        caller.set_interfaces(vec![combined.clone()])?;
        let resolved = combined.resolve_method("run", "()V")?;
        assert!(matches!(
            combined.select_special_method(&caller, &resolved),
            Err(MethodResolutionError::IncompatibleClassChange(_))
        ));
        let single = interface("Single", vec![first.clone()], &[])?;
        let resolved = single.resolve_method("run", "()V")?;
        let selected = single.select_special_method(&caller, &resolved)?;
        assert!(Arc::ptr_eq(&selected.declaring_class, &first));

        let abstract_interface = interface(
            "Abstract",
            vec![first],
            &[(
                "run",
                MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT,
            )],
        )?;
        let resolved = abstract_interface.resolve_method("run", "()V")?;
        assert!(matches!(
            abstract_interface.select_special_method(&caller, &resolved),
            Err(MethodResolutionError::AbstractMethod(_))
        ));
        Ok(())
    }

    #[test]
    fn runtime_packages_use_loader_identity_and_complete_package_names() -> crate::Result<()> {
        let first_loader = ClassLoader::new("same name", ClassPath::new(Vec::new()));
        let second_loader = ClassLoader::new("same name", ClassPath::new(Vec::new()));
        let first = class("pkg/First", None, Some(&first_loader), &[])?;
        let same = class("pkg/Other", None, Some(&first_loader), &[])?;
        let foreign = class("pkg/Other", None, Some(&second_loader), &[])?;
        let different = class("Lpkg/Other", None, Some(&first_loader), &[])?;
        assert!(first.same_runtime_package(&same)?);
        assert!(!first.same_runtime_package(&foreign)?);
        assert!(!first.same_runtime_package(&different)?);
        Ok(())
    }
}
