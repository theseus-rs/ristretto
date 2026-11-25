//! Syntax for one explicit, opt-in application native bridge.
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Binding {
    pub bridge_class: String,
    pub bridge_method: String,
    pub target_class: String,
    pub target_method: String,
}

pub(crate) fn class_name(value: &str) -> Result<String, String> {
    if value.is_empty()
        || value.split(['.', '/']).any(str::is_empty)
        || value
            .chars()
            .any(|c| c.is_whitespace() || "[;()=:<>\\".contains(c))
    {
        return Err("expected a binary class name, using dots or slashes".to_owned());
    }
    Ok(value.to_owned())
}

fn method(value: &str) -> Result<(String, String), String> {
    let (class, method) = value.rsplit_once('.').ok_or_else(|| {
        "expected CLASS.METHOD (the bridge signature is fixed to (I)I)".to_owned()
    })?;
    let class = class_name(class)?;
    if method.is_empty()
        || method
            .chars()
            .any(|c| c.is_whitespace() || "./[;()=:<>\\".contains(c))
    {
        return Err("expected a non-constructor method name without a descriptor".to_owned());
    }
    Ok((class, method.to_owned()))
}

impl Binding {
    pub(crate) async fn install(
        &self,
        vm: &ristretto_vm::VM,
        companions: &[String],
    ) -> ristretto_vm::Result<()> {
        vm.bind_canonical_native(
            &self.bridge_class,
            &self.bridge_method,
            &self.target_class,
            &self.target_method,
            companions,
        )
        .await
    }
}

impl FromStr for Binding {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (bridge, target) = value
            .split_once('=')
            .ok_or_else(|| "expected CLASS.METHOD=TARGET.METHOD".to_owned())?;
        let (bridge_class, bridge_method) = method(bridge)?;
        let (target_class, target_method) = method(target)?;
        Ok(Self {
            bridge_class,
            bridge_method,
            target_class,
            target_method,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_binary_names_and_splits_the_final_method_separator() -> Result<(), String> {
        let binding: Binding = "example.Caller.enter=example/Target.compute".parse()?;
        assert_eq!(binding.bridge_class, "example.Caller");
        assert_eq!(binding.bridge_method, "enter");
        assert_eq!(binding.target_class, "example/Target");
        assert_eq!(binding.target_method, "compute");
        assert_eq!(class_name("example.Target$Node")?, "example.Target$Node");
        assert!("例.入口.呼ぶ=例/目標.実行".parse::<Binding>().is_ok());
        Ok(())
    }

    #[test]
    fn rejects_ambiguous_bindings_descriptors_and_invalid_binary_names() {
        for invalid in [
            "",
            "Caller.enter",
            "=Target.run",
            "Caller.enter=",
            "Caller.enter=Target.run=Other.run",
            "Caller.enter(I)I=Target.run",
            "Caller.<init>=Target.run",
            "Caller.=Target.run",
            ".Caller.enter=Target.run",
            "a..Caller.enter=Target.run",
            "Caller.enter=Target/running",
            "Caller.enter=Target. run",
            "Caller.enter=[Target.run",
            "a\\Caller.enter=Target.run",
        ] {
            assert!(invalid.parse::<Binding>().is_err(), "accepted {invalid}");
        }
        for invalid in [
            "",
            "/Target",
            "Target/",
            "a//Target",
            "Target;",
            "[I",
            "a b",
        ] {
            assert!(class_name(invalid).is_err(), "accepted {invalid}");
        }
    }
}
