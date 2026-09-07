//! Resolved field storage shared by frames through per-class constant-pool tables.

use crate::Error::{InternalError, PoisonedLock};
use crate::JavaError::{IllegalAccessError, IncompatibleClassChangeError, NoSuchFieldError};
use crate::Result;
use crate::frame::Frame;
use crate::instruction::method_resolver::check_jpms_access;
use crate::reference_cache::ReferenceCache;
use dashmap::DashMap;
use ristretto_classfile::attributes::Attribute;
use ristretto_classfile::{BaseType, ClassAccessFlags, FieldAccessFlags, FieldType};
use ristretto_classloader::{Class, Field, Object, Value};
use std::sync::{Arc, OnceLock, RwLock};

pub(crate) type FieldRefEntry = OnceLock<Arc<ResolvedFieldRef>>;
pub(crate) type FieldRefCache = ReferenceCache<FieldRefEntry>;

#[derive(Debug)]
enum FieldStorage {
    Instance(usize),
    Static(Arc<RwLock<Value>>),
}

/// The common receiver needs only a pointer comparison. Additional receiver classes are
/// checked once and retained so class-loader identity cannot be confused by address reuse.
#[derive(Debug, Default)]
struct ReceiverChecks {
    first: OnceLock<Arc<Class>>,
    others: OnceLock<DashMap<usize, Arc<Class>>>,
}

impl ReceiverChecks {
    fn contains(&self, class: &Arc<Class>) -> bool {
        self.first
            .get()
            .is_some_and(|first| Arc::ptr_eq(first, class))
            || self
                .others
                .get()
                .is_some_and(|others| others.contains_key(&(Arc::as_ptr(class) as usize)))
    }

    fn insert(&self, class: &Arc<Class>) {
        let first = self.first.get_or_init(|| class.clone());
        if !Arc::ptr_eq(first, class) {
            self.others
                .get_or_init(DashMap::new)
                .entry(Arc::as_ptr(class) as usize)
                .or_insert_with(|| class.clone());
        }
    }
}

#[derive(Debug)]
pub(crate) struct ResolvedFieldRef {
    pub(crate) declaring_class: Arc<Class>,
    field: Arc<Field>,
    storage: FieldStorage,
    /// Cross-package protected access also constrains the receiver at execution time.
    protected_receiver: Option<Arc<Class>>,
    receivers: ReceiverChecks,
}

impl ResolvedFieldRef {
    pub(crate) fn is_static(&self) -> bool {
        matches!(self.storage, FieldStorage::Static(_))
    }

    pub(crate) fn check_kind(&self, is_static: bool) -> Result<()> {
        if self.is_static() != is_static {
            return Err(IncompatibleClassChangeError(format!(
                "Field {}.{} {} static",
                self.declaring_class.name(),
                self.field.name(),
                if self.is_static() { "is" } else { "is not" }
            ))
            .into());
        }
        Ok(())
    }

    /// Final writes are checked for each instruction, since frames share resolved references.
    pub(crate) fn check_write(&self, frame: &Frame) -> Result<()> {
        if self.field.access_flags().contains(FieldAccessFlags::FINAL)
            && (!Arc::ptr_eq(frame.class(), &self.declaring_class)
                || frame.method().name()
                    != if self.is_static() {
                        "<clinit>"
                    } else {
                        "<init>"
                    })
        {
            return Err(IllegalAccessError(format!(
                "Cannot write final field {}.{} from {}.{}",
                self.declaring_class.name(),
                self.field.name(),
                frame.class().name(),
                frame.method().name()
            ))
            .into());
        }
        Ok(())
    }

    fn instance_slot(&self, object: &Object) -> Result<usize> {
        self.check_kind(false)?;
        let FieldStorage::Instance(slot) = self.storage else {
            return Err(InternalError("Expected instance field storage".to_owned()));
        };
        let receiver = object.class();
        if self.receivers.contains(receiver) {
            return Ok(slot);
        }
        if !is_subclass_or_same(receiver, &self.declaring_class)? {
            return Err(IncompatibleClassChangeError(format!(
                "{} is not an instance of {}",
                receiver.name(),
                self.declaring_class.name()
            ))
            .into());
        }
        if let Some(caller) = &self.protected_receiver
            && !is_subclass_or_same(receiver, caller)?
        {
            return Err(IllegalAccessError(format!(
                "Cannot access protected field {}.{} on {} from {}",
                self.declaring_class.name(),
                self.field.name(),
                receiver.name(),
                caller.name()
            ))
            .into());
        }
        // Standalone classloader users can still change unlinked hierarchies. Only publish
        // successes once all dependencies are immutable; never cache a failed access.
        if receiver.is_linked()
            && self.declaring_class.is_linked()
            && self
                .protected_receiver
                .as_ref()
                .is_none_or(|class| class.is_linked())
        {
            self.receivers.insert(receiver);
        }
        Ok(slot)
    }

    pub(crate) fn get(&self, object: &Object) -> Result<Value> {
        Ok(object.value_at_slot(self.instance_slot(object)?)?)
    }

    pub(crate) fn put(&self, object: &mut Object, value: Value) -> Result<()> {
        let slot = self.instance_slot(object)?;
        let value = self.checked_value(value)?;
        Ok(object.set_value_at_slot(slot, value)?)
    }

    pub(crate) fn get_static(&self) -> Result<Value> {
        self.check_kind(true)?;
        let FieldStorage::Static(storage) = &self.storage else {
            return Err(InternalError("Expected static field storage".to_owned()));
        };
        Ok(storage
            .read()
            .map_err(|error| PoisonedLock(error.to_string()))?
            .clone())
    }

    pub(crate) fn put_static(&self, value: Value) -> Result<()> {
        self.check_kind(true)?;
        let FieldStorage::Static(storage) = &self.storage else {
            return Err(InternalError("Expected static field storage".to_owned()));
        };
        let value = self.checked_value(value)?;
        *storage
            .write()
            .map_err(|error| PoisonedLock(error.to_string()))? = value;
        Ok(())
    }

    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "JVM field stores narrow integer values to the declared field type"
    )]
    fn checked_value(&self, value: Value) -> Result<Value> {
        self.field.check_value(&value)?;
        Ok(match (self.field.field_type(), value) {
            (FieldType::Base(BaseType::Boolean), Value::Int(value)) => Value::Int(value & 1),
            (FieldType::Base(BaseType::Byte), Value::Int(value)) => {
                Value::Int(i32::from(value as i8))
            }
            (FieldType::Base(BaseType::Char), Value::Int(value)) => {
                Value::Int(i32::from(value as u16))
            }
            (FieldType::Base(BaseType::Short), Value::Int(value)) => {
                Value::Int(i32::from(value as i16))
            }
            (_, value) => value,
        })
    }
}

pub(crate) async fn resolve_field_ref(frame: &Frame, index: u16) -> Result<Arc<ResolvedFieldRef>> {
    let entry = frame
        .field_refs()?
        .get(index)
        .ok_or(ristretto_classfile::Error::InvalidConstantPoolIndex(index))?;
    if let Some(resolved) = entry.get() {
        return Ok(resolved.clone());
    }

    let pool = frame.class().constant_pool();
    let (class_index, name_and_type) = pool.try_get_field_ref(index)?;
    let (name_index, descriptor_index) = pool.try_get_name_and_type(*name_and_type)?;
    let name = pool.try_get_utf8(*name_index)?.to_string();
    let descriptor = pool.try_get_utf8(*descriptor_index)?.to_string();
    let field_type = FieldType::parse(&descriptor)?;
    let target = frame
        .thread()?
        .load_referenced_class(frame.class(), pool.try_get_class(*class_index)?)
        .await?;
    check_jpms_access(frame, &target)?;
    if !target
        .class_file()
        .access_flags
        .contains(ClassAccessFlags::PUBLIC)
        && !same_runtime_package(frame.class(), &target)?
    {
        return Err(IllegalAccessError(format!(
            "{} cannot access {}",
            frame.class().name(),
            target.name()
        ))
        .into());
    }
    let (declaring_class, field) = lookup_field(&target, &name, &field_type)?
        .ok_or_else(|| NoSuchFieldError(format!("{}.{name}:{descriptor}", target.name())))?;
    let protected_receiver = check_access(frame, &declaring_class, &field).await?;
    let storage = if field.access_flags().contains(FieldAccessFlags::STATIC) {
        let index = declaring_class
            .static_fields()
            .iter()
            .position(|candidate| Arc::ptr_eq(candidate, &field))
            .ok_or_else(|| InternalError("Missing resolved static field".to_owned()))?;
        FieldStorage::Static(declaring_class.static_storage(index)?)
    } else {
        let slot = declaring_class
            .object_field_layout()?
            .iter()
            .position(|candidate| Arc::ptr_eq(candidate, &field))
            .ok_or_else(|| InternalError("Missing resolved instance field".to_owned()))?;
        FieldStorage::Instance(slot)
    };
    let resolved = Arc::new(ResolvedFieldRef {
        declaring_class,
        field,
        storage,
        protected_receiver,
        receivers: ReceiverChecks::default(),
    });
    Ok(entry.get_or_init(|| resolved).clone())
}

/// Match both name and descriptor, visiting interfaces before the superclass (JVMS 5.4.3.2).
fn lookup_field(
    class: &Arc<Class>,
    name: &str,
    field_type: &FieldType,
) -> Result<Option<(Arc<Class>, Arc<Field>)>> {
    for field in class.declared_fields() {
        if field.name() == name && field.field_type() == field_type {
            return Ok(Some((class.clone(), field)));
        }
    }
    for interface in class.interfaces()? {
        if let Some(found) = lookup_field(&interface, name, field_type)? {
            return Ok(Some(found));
        }
    }
    if let Some(parent) = class.parent()? {
        return lookup_field(&parent, name, field_type);
    }
    Ok(None)
}

fn same_runtime_package(first: &Arc<Class>, second: &Arc<Class>) -> Result<bool> {
    let first_package = first
        .name()
        .rsplit_once('/')
        .map_or("", |(package, _)| package);
    let second_package = second
        .name()
        .rsplit_once('/')
        .map_or("", |(package, _)| package);
    if first_package != second_package {
        return Ok(false);
    }
    Ok(match (first.class_loader()?, second.class_loader()?) {
        (Some(first), Some(second)) => Arc::ptr_eq(&first, &second),
        (None, None) => true,
        _ => false,
    })
}

fn is_subclass_or_same(class: &Arc<Class>, parent: &Arc<Class>) -> Result<bool> {
    let mut current = Some(class.clone());
    while let Some(class) = current {
        if Arc::ptr_eq(&class, parent) {
            return Ok(true);
        }
        current = class.parent()?;
    }
    Ok(false)
}

async fn check_access(
    frame: &Frame,
    class: &Arc<Class>,
    field: &Field,
) -> Result<Option<Arc<Class>>> {
    let caller = frame.class();
    let flags = field.access_flags();
    if Arc::ptr_eq(caller, class) || flags.contains(FieldAccessFlags::PUBLIC) {
        return Ok(None);
    }
    if flags.contains(FieldAccessFlags::PRIVATE) {
        if same_runtime_package(caller, class)? {
            let caller_host = nest_host(frame, caller).await?;
            let field_host = nest_host(frame, class).await?;
            if Arc::ptr_eq(&caller_host, &field_host) {
                return Ok(None);
            }
        }
    } else if same_runtime_package(caller, class)? {
        return Ok(None);
    } else if flags.contains(FieldAccessFlags::PROTECTED) && is_subclass_or_same(caller, class)? {
        return Ok((!flags.contains(FieldAccessFlags::STATIC)).then(|| caller.clone()));
    }
    Err(IllegalAccessError(format!(
        "{} cannot access field {}.{}",
        caller.name(),
        class.name(),
        field.name()
    ))
    .into())
}

async fn nest_host(frame: &Frame, class: &Arc<Class>) -> Result<Arc<Class>> {
    for attribute in &class.class_file().attributes {
        if let Attribute::NestHost {
            host_class_index, ..
        } = attribute
        {
            let host = frame
                .thread()?
                .load_referenced_class(
                    class,
                    class.constant_pool().try_get_class(*host_class_index)?,
                )
                .await?;
            if same_runtime_package(class, &host)? {
                for attribute in &host.class_file().attributes {
                    if let Attribute::NestMembers { class_indexes, .. } = attribute {
                        for index in class_indexes {
                            if host.constant_pool().try_get_class(*index)? == class.java_name() {
                                return Ok(host);
                            }
                        }
                    }
                }
            }
            return Err(
                IllegalAccessError(format!("Invalid nest host for {}", class.name())).into(),
            );
        }
    }
    Ok(class.clone())
}

#[cfg(test)]
mod tests;
