//! Loading constraints on reference types in resolved member descriptors (JVMS §5.3.4).
use crate::{Class, Error, Result};
use ahash::AHashMap;
use ristretto_classfile::{JavaStr, JavaString};
use ristretto_gc::sync::Mutex;
use std::sync::Arc;

#[derive(Debug)]
enum Binding {
    Loaded(Arc<Class>),
    Constrained(Arc<Group>),
}

#[derive(Debug, Default)]
pub(crate) struct LoaderBindings(Mutex<AHashMap<JavaString, Binding>>);

impl LoaderBindings {
    pub(crate) fn record(&self, class: &Arc<Class>) -> Result<()> {
        let name = class.java_name();
        let mut bindings = self.0.lock();
        match bindings.get_mut(name) {
            Some(Binding::Constrained(group)) => Group::bind(group.clone(), class),
            Some(Binding::Loaded(old)) => {
                old.clone_from(class);
                Ok(())
            }
            None => {
                bindings.insert(name.to_owned(), Binding::Loaded(class.clone()));
                Ok(())
            }
        }
    }

    fn group(&self, name: &JavaStr) -> Arc<Group> {
        let mut bindings = self.0.lock();
        let loaded = match bindings.get(name) {
            Some(Binding::Constrained(group)) => return group.clone(),
            Some(Binding::Loaded(class)) => Some(class.clone()),
            None => None,
        };
        let group = Arc::new(Group(Mutex::new(State::Root(loaded))));
        bindings.insert(name.to_owned(), Binding::Constrained(group.clone()));
        group
    }

    pub(crate) fn constrain(&self, other: &Self, name: &JavaStr) -> Result<()> {
        let mut left = self.group(name);
        let mut right = other.group(name);
        loop {
            left = Group::root(left);
            right = Group::root(right);
            if Arc::ptr_eq(&left, &right) {
                return Ok(());
            }
            // Merging roots takes both locks in identity order. Recheck in case another
            // thread joined either group while these locks were being acquired.
            if Arc::as_ptr(&left) > Arc::as_ptr(&right) {
                std::mem::swap(&mut left, &mut right);
            }
            let mut first = left.0.lock();
            let mut second = right.0.lock();
            if let (State::Root(a), State::Root(b)) = (&mut *first, &mut *second) {
                if let Some(class) = b {
                    bind(a, class)?;
                }
                *second = State::Joined(left.clone());
                return Ok(());
            }
        }
    }
}

#[derive(Debug)]
struct Group(Mutex<State>);

#[derive(Debug)]
enum State {
    Root(Option<Arc<Class>>),
    Joined(Arc<Group>),
}

impl Group {
    fn root(mut group: Arc<Self>) -> Arc<Self> {
        loop {
            let next = match &*group.0.lock() {
                State::Root(_) => None,
                State::Joined(next) => Some(next.clone()),
            };
            match next {
                Some(next) => group = next,
                None => return group,
            }
        }
    }

    fn bind(mut group: Arc<Self>, class: &Arc<Class>) -> Result<()> {
        loop {
            let next = match &mut *group.0.lock() {
                State::Root(current) => return bind(current, class),
                State::Joined(next) => next.clone(),
            };
            group = next;
        }
    }
}

fn bind(current: &mut Option<Arc<Class>>, class: &Arc<Class>) -> Result<()> {
    if let Some(existing) = current {
        if !Arc::ptr_eq(existing, class) {
            return Err(Error::LoadingConstraint(class.name().to_owned()));
        }
    } else {
        *current = Some(class.clone());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classfile::{ClassFile, ConstantPool};

    fn definition() -> Result<Arc<Class>> {
        let mut pool = ConstantPool::new();
        let this_class = pool.add_class("Value")?;
        Class::from(
            None,
            ClassFile {
                constant_pool: pool,
                this_class,
                ..Default::default()
            },
        )
    }

    #[test]
    fn constraints_bind_lazily_and_transitively() -> Result<()> {
        let value = definition()?;
        let other = definition()?;
        let (a, b, c, unrelated) = (
            LoaderBindings::default(),
            LoaderBindings::default(),
            LoaderBindings::default(),
            LoaderBindings::default(),
        );
        let name = value.java_name();
        a.constrain(&b, name)?;
        b.constrain(&c, name)?;
        c.record(&value)?;
        assert!(matches!(a.record(&other), Err(Error::LoadingConstraint(_))));
        unrelated.record(&other)?;
        assert!(matches!(
            unrelated.constrain(&b, name),
            Err(Error::LoadingConstraint(_))
        ));
        unrelated.record(&other)?;
        a.record(&value)?;
        b.record(&value)?;
        Ok(())
    }
}
