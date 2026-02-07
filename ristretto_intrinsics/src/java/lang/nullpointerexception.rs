use ristretto_classfile::Constant;
use ristretto_classfile::FieldType;
use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classfile::attributes::Instruction;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaObject;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

/// Convert a `FieldType` to a human-readable type name.
fn field_type_to_name(field_type: &FieldType) -> String {
    match field_type {
        FieldType::Base(base_type) => base_type.class_name().to_string(),
        FieldType::Object(class_name) => {
            // Get just the simple class name (after the last '/')
            class_name
                .rsplit('/')
                .next()
                .unwrap_or(class_name)
                .to_string()
        }
        FieldType::Array(component) => {
            format!("{} array", field_type_to_name(component))
        }
    }
}

#[intrinsic_method(
    "java/lang/NullPointerException.getExtendedNPEMessage()Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn get_extended_npe_message<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let Ok(exception_value) = parameters.pop() else {
        return Ok(Some(Value::Object(None)));
    };
    // Check if exception is actually an object
    if !exception_value.is_object() {
        return Ok(Some(Value::Object(None)));
    }
    // We need to access the "backtrace" field of the exception object.
    let backtrace_value = {
        let exception_ref = exception_value.as_object_ref()?;
        if let Ok(v) = exception_ref.value("backtrace") {
            v
        } else {
            return Ok(Some(Value::Object(None)));
        }
    };

    // The backtrace is an Object[] of Object[] (representing frames).
    let frames: Vec<Value> = if let Ok(v) = backtrace_value.try_into() {
        v
    } else {
        return Ok(Some(Value::Object(None)));
    };

    if frames.is_empty() {
        return Ok(Some(Value::Object(None)));
    }

    // Top frame is at index 0
    let top_frame_value = &frames[0];
    let top_frame_info: Vec<Value> = if let Ok(v) = top_frame_value.clone().try_into() {
        v
    } else {
        return Ok(Some(Value::Object(None)));
    };

    // [Class, MethodName, Descriptor, BCI]
    if top_frame_info.len() < 4 {
        return Ok(Some(Value::Object(None)));
    }

    // 1. Class
    // The first element is a Class object (java.lang.Class).
    let class_value = &top_frame_info[0];
    let class_name_str = {
        let class_obj_ref = class_value.as_object_ref()?;
        // java.lang.Class has a "name" field which is a String.
        let name_val = class_obj_ref.value("name")?;
        name_val.as_string()?
    };

    // The name in Class object is typically "java.lang.String". Internal names use slashes "java/lang/String".
    let internal_class_name = class_name_str.replace('.', "/");

    // 2. Method Name (String)
    let method_name = top_frame_info[1].as_string()?;

    // 3. Descriptor (String)
    let descriptor = top_frame_info[2].as_string()?;

    // 4. BCI (Integer)
    let bci_value = &top_frame_info[3];
    // This is an Integer object.
    let bci = usize::try_from(bci_value.as_i32()?).unwrap_or(0);

    // Load Class and Method
    let Ok(class) = thread.class(&internal_class_name).await else {
        return Ok(Some(Value::Object(None)));
    };

    let Some(method) = class.method(&method_name, &descriptor) else {
        return Ok(Some(Value::Object(None)));
    };

    // Analyze
    let analyzer = NpeAnalyzer::new(thread.clone(), class.clone(), method.clone());
    let message = analyzer.analyze(bci);

    if let Some(msg) = message {
        let msg_obj = msg.to_object(&thread).await?;
        Ok(Some(msg_obj))
    } else {
        Ok(Some(Value::Object(None)))
    }
}

// --- Analysis Logic ---

struct NpeAnalyzer<T: Send + Sync> {
    #[expect(dead_code)]
    thread: Arc<T>,
    class: Arc<ristretto_classloader::Class>,
    method: Arc<ristretto_classloader::Method>,
}

#[derive(Debug, Clone)]
enum Source {
    /// Local variable index
    Local(usize),
    /// Explicit "this" reference
    This,
    /// Field access, with optional receiver source
    Field(String, Option<Box<Source>>),
    /// Method return value (class name, method name)
    MethodReturn(String, String),
    /// Array element access with optional array source
    ArrayAccess(Option<Box<Source>>),
    /// Constant value (e.g., null literal)
    Constant,
    /// Unknown source
    Unknown,
}

impl<T: Send + Sync> NpeAnalyzer<T> {
    fn new(
        thread: Arc<T>,
        class: Arc<ristretto_classloader::Class>,
        method: Arc<ristretto_classloader::Method>,
    ) -> Self {
        Self {
            thread,
            class,
            method,
        }
    }

    #[expect(clippy::too_many_lines)]
    #[expect(clippy::match_same_arms)]
    fn analyze(&self, target_bci: usize) -> Option<String> {
        let code = self.method.code();
        let mut stack: Vec<Source> = Vec::new();
        let mut pc = 0;
        let is_instance_method = !self.method.is_static();

        while pc < target_bci {
            let Some(instruction) = code.get(pc) else {
                break;
            };

            match instruction {
                Instruction::Aconst_null
                | Instruction::Iconst_m1
                | Instruction::Iconst_0
                | Instruction::Iconst_1
                | Instruction::Iconst_2
                | Instruction::Iconst_3
                | Instruction::Iconst_4
                | Instruction::Iconst_5
                | Instruction::Lconst_0
                | Instruction::Lconst_1
                | Instruction::Fconst_0
                | Instruction::Fconst_1
                | Instruction::Fconst_2
                | Instruction::Dconst_0
                | Instruction::Dconst_1
                | Instruction::Bipush(_)
                | Instruction::Sipush(_)
                | Instruction::Ldc(_)
                | Instruction::Ldc_w(_)
                | Instruction::Ldc2_w(_) => {
                    stack.push(Source::Constant);
                }

                Instruction::Aload(idx) => {
                    let idx = *idx as usize;
                    if is_instance_method && idx == 0 {
                        stack.push(Source::This);
                    } else {
                        stack.push(Source::Local(idx));
                    }
                }
                Instruction::Aload_0 => {
                    if is_instance_method {
                        stack.push(Source::This);
                    } else {
                        stack.push(Source::Local(0));
                    }
                }
                Instruction::Aload_1 => stack.push(Source::Local(1)),
                Instruction::Aload_2 => stack.push(Source::Local(2)),
                Instruction::Aload_3 => stack.push(Source::Local(3)),

                Instruction::Iload(_)
                | Instruction::Fload(_)
                | Instruction::Iload_0
                | Instruction::Iload_1
                | Instruction::Iload_2
                | Instruction::Iload_3
                | Instruction::Fload_0
                | Instruction::Fload_1
                | Instruction::Fload_2
                | Instruction::Fload_3 => {
                    stack.push(Source::Unknown);
                }
                Instruction::Lload(_)
                | Instruction::Dload(_)
                | Instruction::Lload_0
                | Instruction::Lload_1
                | Instruction::Lload_2
                | Instruction::Lload_3
                | Instruction::Dload_0
                | Instruction::Dload_1
                | Instruction::Dload_2
                | Instruction::Dload_3 => {
                    stack.push(Source::Unknown);
                    stack.push(Source::Unknown);
                }

                Instruction::Getfield(idx) => {
                    let receiver = stack.pop();
                    let (_class_name, field_name, descriptor) = self.resolve_field(*idx);
                    let source = Source::Field(field_name.clone(), receiver.map(Box::new));
                    if descriptor == "D" || descriptor == "J" {
                        stack.push(source.clone());
                        stack.push(Source::Unknown);
                    } else {
                        stack.push(source);
                    }
                }
                Instruction::Getstatic(idx) => {
                    let (_, field_name, descriptor) = self.resolve_field(*idx);
                    if descriptor == "D" || descriptor == "J" {
                        stack.push(Source::Field(field_name.clone(), None));
                        stack.push(Source::Unknown);
                    } else {
                        stack.push(Source::Field(field_name, None));
                    }
                }

                Instruction::Invokevirtual(idx)
                | Instruction::Invokeinterface(idx, _)
                | Instruction::Invokespecial(idx)
                | Instruction::Invokestatic(idx) => {
                    let (class_name, method_name, descriptor) = self.resolve_method_ref(*idx);
                    let args_count = self.count_slots(&descriptor);
                    for _ in 0..args_count {
                        stack.pop();
                    }
                    if !matches!(instruction, Instruction::Invokestatic(_)) {
                        stack.pop(); // objectref
                    }

                    let return_type = descriptor.split(')').nth(1).unwrap_or("V");
                    if return_type != "V" {
                        stack.push(Source::MethodReturn(
                            class_name.clone(),
                            method_name.clone(),
                        ));
                        if return_type == "D" || return_type == "J" {
                            stack.push(Source::Unknown);
                        }
                    }
                }

                Instruction::Aaload => {
                    stack.pop(); // index
                    let array_source = stack.pop();
                    stack.push(Source::ArrayAccess(array_source.map(Box::new)));
                }

                Instruction::Iaload
                | Instruction::Laload
                | Instruction::Faload
                | Instruction::Daload
                | Instruction::Baload
                | Instruction::Caload
                | Instruction::Saload => {
                    stack.pop(); // index
                    stack.pop(); // arrayref
                    stack.push(Source::Unknown);
                }

                Instruction::Astore(_)
                | Instruction::Astore_0
                | Instruction::Astore_1
                | Instruction::Astore_2
                | Instruction::Astore_3 => {
                    stack.pop();
                }

                Instruction::Istore(_)
                | Instruction::Fstore(_)
                | Instruction::Istore_0
                | Instruction::Istore_1
                | Instruction::Istore_2
                | Instruction::Istore_3
                | Instruction::Fstore_0
                | Instruction::Fstore_1
                | Instruction::Fstore_2
                | Instruction::Fstore_3 => {
                    stack.pop();
                }

                Instruction::Lstore(_)
                | Instruction::Dstore(_)
                | Instruction::Lstore_0
                | Instruction::Lstore_1
                | Instruction::Lstore_2
                | Instruction::Lstore_3
                | Instruction::Dstore_0
                | Instruction::Dstore_1
                | Instruction::Dstore_2
                | Instruction::Dstore_3 => {
                    stack.pop();
                    stack.pop();
                }

                Instruction::Pop => {
                    stack.pop();
                }
                Instruction::Pop2 => {
                    stack.pop();
                    stack.pop();
                }
                Instruction::Dup => {
                    if let Some(last) = stack.last() {
                        stack.push(last.clone());
                    }
                }
                Instruction::Dup_x1 => {
                    if stack.len() >= 2 {
                        let v1 = stack.pop().expect("stack checked");
                        let v2 = stack.pop().expect("stack checked");
                        stack.push(v1.clone());
                        stack.push(v2);
                        stack.push(v1);
                    }
                }
                Instruction::Dup_x2 => {
                    if stack.len() >= 3 {
                        let v1 = stack.pop().expect("stack checked");
                        let v2 = stack.pop().expect("stack checked");
                        let v3 = stack.pop().expect("stack checked");
                        stack.push(v1.clone());
                        stack.push(v3);
                        stack.push(v2);
                        stack.push(v1);
                    }
                }
                Instruction::Dup2 => {
                    if stack.len() >= 2 {
                        let len = stack.len();
                        stack.push(stack[len - 2].clone());
                        stack.push(stack[len - 1].clone());
                    }
                }
                Instruction::Swap => {
                    if stack.len() >= 2 {
                        let len = stack.len();
                        stack.swap(len - 1, len - 2);
                    }
                }

                // Arithmetic operations that consume and produce values
                Instruction::Iadd
                | Instruction::Isub
                | Instruction::Imul
                | Instruction::Idiv
                | Instruction::Irem
                | Instruction::Iand
                | Instruction::Ior
                | Instruction::Ixor
                | Instruction::Ishl
                | Instruction::Ishr
                | Instruction::Iushr
                | Instruction::Fadd
                | Instruction::Fsub
                | Instruction::Fmul
                | Instruction::Fdiv
                | Instruction::Frem
                | Instruction::Fcmpl
                | Instruction::Fcmpg => {
                    stack.pop();
                    stack.pop();
                    stack.push(Source::Unknown);
                }

                Instruction::Ladd
                | Instruction::Lsub
                | Instruction::Lmul
                | Instruction::Ldiv
                | Instruction::Lrem
                | Instruction::Land
                | Instruction::Lor
                | Instruction::Lxor
                | Instruction::Dadd
                | Instruction::Dsub
                | Instruction::Dmul
                | Instruction::Ddiv
                | Instruction::Drem
                | Instruction::Lcmp
                | Instruction::Dcmpl
                | Instruction::Dcmpg => {
                    stack.pop();
                    stack.pop();
                    stack.pop();
                    stack.pop();
                    stack.push(Source::Unknown);
                }

                Instruction::Lshl | Instruction::Lshr | Instruction::Lushr => {
                    stack.pop(); // shift amount (int, 1 slot)
                    stack.pop();
                    stack.pop(); // long (2 slots)
                    stack.push(Source::Unknown);
                    stack.push(Source::Unknown);
                }

                Instruction::Ineg | Instruction::Fneg => {
                    stack.pop();
                    stack.push(Source::Unknown);
                }

                Instruction::Lneg | Instruction::Dneg => {
                    stack.pop();
                    stack.pop();
                    stack.push(Source::Unknown);
                    stack.push(Source::Unknown);
                }

                // Type conversions
                Instruction::I2l | Instruction::I2d | Instruction::F2l | Instruction::F2d => {
                    stack.pop();
                    stack.push(Source::Unknown);
                    stack.push(Source::Unknown);
                }

                Instruction::L2i | Instruction::L2f | Instruction::D2i | Instruction::D2f => {
                    stack.pop();
                    stack.pop();
                    stack.push(Source::Unknown);
                }

                Instruction::I2f
                | Instruction::I2b
                | Instruction::I2c
                | Instruction::I2s
                | Instruction::F2i => {
                    stack.pop();
                    stack.push(Source::Unknown);
                }

                Instruction::L2d | Instruction::D2l => {
                    stack.pop();
                    stack.pop();
                    stack.push(Source::Unknown);
                    stack.push(Source::Unknown);
                }

                Instruction::New(_) => {
                    stack.push(Source::Unknown);
                }

                Instruction::Newarray(_) | Instruction::Anewarray(_) => {
                    stack.pop(); // count
                    stack.push(Source::Unknown);
                }

                Instruction::Checkcast(_) => {
                    // Doesn't change the stack, just checks type
                }

                Instruction::Instanceof(_) => {
                    stack.pop();
                    stack.push(Source::Unknown);
                }

                Instruction::Return
                | Instruction::Ireturn
                | Instruction::Lreturn
                | Instruction::Freturn
                | Instruction::Dreturn
                | Instruction::Areturn => {
                    // Method returns, stack state doesn't matter for our analysis
                }

                _ => {}
            }

            pc += 1;
        }

        let instruction = code.get(target_bci)?;

        match instruction {
            Instruction::Getfield(idx) => {
                if let Some(source) = stack.last() {
                    let (_, field_name, _) = self.resolve_field(*idx);
                    let reason = self.describe_source(source);
                    return Some(format!(
                        "Cannot read field \"{field_name}\" because {reason} is null"
                    ));
                }
            }
            Instruction::Putfield(idx) => {
                let (_, _, descriptor) = self.resolve_field(*idx);
                let offset = if descriptor == "D" || descriptor == "J" {
                    2
                } else {
                    1
                };
                if stack.len() > offset {
                    let source = &stack[stack.len() - 1 - offset];
                    let (_, field_name, _) = self.resolve_field(*idx);
                    let reason = self.describe_source(source);
                    return Some(format!(
                        "Cannot assign field \"{field_name}\" because {reason} is null"
                    ));
                }
            }
            Instruction::Invokevirtual(idx) | Instruction::Invokeinterface(idx, _) => {
                let (class_name, method_name, descriptor) = self.resolve_method_ref(*idx);
                let args_count = self.count_slots(&descriptor);
                if stack.len() > args_count {
                    let source = &stack[stack.len() - 1 - args_count];
                    let reason = self.describe_source(source);
                    // Use simple class name for the message
                    let simple_class = class_name.rsplit('/').next().unwrap_or(&class_name);
                    return Some(format!(
                        "Cannot invoke \"{simple_class}.{method_name}()\" because {reason} is null"
                    ));
                }
            }
            Instruction::Arraylength => {
                if let Some(source) = stack.last() {
                    let reason = self.describe_source(source);
                    return Some(format!(
                        "Cannot read the array length because {reason} is null"
                    ));
                }
            }
            Instruction::Iaload => {
                if stack.len() > 1 {
                    let source = &stack[stack.len() - 2];
                    let reason = self.describe_source(source);
                    return Some(format!(
                        "Cannot load from int array because {reason} is null"
                    ));
                }
            }
            Instruction::Laload => {
                if stack.len() > 1 {
                    let source = &stack[stack.len() - 2];
                    let reason = self.describe_source(source);
                    return Some(format!(
                        "Cannot load from long array because {reason} is null"
                    ));
                }
            }
            Instruction::Faload => {
                if stack.len() > 1 {
                    let source = &stack[stack.len() - 2];
                    let reason = self.describe_source(source);
                    return Some(format!(
                        "Cannot load from float array because {reason} is null"
                    ));
                }
            }
            Instruction::Daload => {
                if stack.len() > 1 {
                    let source = &stack[stack.len() - 2];
                    let reason = self.describe_source(source);
                    return Some(format!(
                        "Cannot load from double array because {reason} is null"
                    ));
                }
            }
            Instruction::Aaload => {
                if stack.len() > 1 {
                    let source = &stack[stack.len() - 2];
                    let reason = self.describe_source(source);
                    return Some(format!(
                        "Cannot load from object array because {reason} is null"
                    ));
                }
            }
            Instruction::Baload => {
                if stack.len() > 1 {
                    let source = &stack[stack.len() - 2];
                    let reason = self.describe_source(source);
                    return Some(format!(
                        "Cannot load from byte/boolean array because {reason} is null"
                    ));
                }
            }
            Instruction::Caload => {
                if stack.len() > 1 {
                    let source = &stack[stack.len() - 2];
                    let reason = self.describe_source(source);
                    return Some(format!(
                        "Cannot load from char array because {reason} is null"
                    ));
                }
            }
            Instruction::Saload => {
                if stack.len() > 1 {
                    let source = &stack[stack.len() - 2];
                    let reason = self.describe_source(source);
                    return Some(format!(
                        "Cannot load from short array because {reason} is null"
                    ));
                }
            }
            Instruction::Athrow => {
                if let Some(source) = stack.last() {
                    let reason = self.describe_source(source);
                    return Some(format!("Cannot throw exception because {reason} is null"));
                }
            }
            Instruction::Monitorenter | Instruction::Monitorexit => {
                if let Some(source) = stack.last() {
                    let reason = self.describe_source(source);
                    let op = if matches!(instruction, Instruction::Monitorenter) {
                        "enter"
                    } else {
                        "exit"
                    };
                    return Some(format!(
                        "Cannot {op} synchronized block because {reason} is null"
                    ));
                }
            }
            _ => {}
        }

        None
    }

    fn describe_source(&self, source: &Source) -> String {
        match source {
            Source::Local(idx) => {
                if let Some(name) = self.get_local_name(*idx) {
                    format!("\"{name}\"")
                } else {
                    format!("\"<local{idx}>\"")
                }
            }
            Source::This => "\"this\"".to_string(),
            Source::Field(name, receiver) => {
                if let Some(recv) = receiver {
                    match recv.as_ref() {
                        Source::Local(idx) => {
                            if let Some(local_name) = self.get_local_name(*idx) {
                                format!("\"{local_name}.{name}\"")
                            } else {
                                format!("\"<local{idx}>.{name}\"")
                            }
                        }
                        Source::This => format!("\"this.{name}\""),
                        _ => format!("\"{name}\""),
                    }
                } else {
                    format!("\"{name}\"")
                }
            }
            Source::MethodReturn(class_name, method_name) => {
                // Use simple class name (just the part after the last /)
                let simple_class = class_name.rsplit('/').next().unwrap_or(class_name);
                format!("the return value of \"{simple_class}.{method_name}()\"")
            }
            Source::ArrayAccess(_) => "an array element".to_string(),
            Source::Constant => "\"null\"".to_string(),
            Source::Unknown => "\"<unknown>\"".to_string(),
        }
    }

    /// Get the local variable name for a given index.
    ///
    /// According to the JVM specification, when a local variable is a method parameter
    /// and no debug information is available, it should be named `<parameterN>` where N
    /// is the 1-based parameter index.
    fn get_local_name(&self, index: usize) -> Option<String> {
        // For instance methods, local 0 is 'this', so parameters start at index 1.
        // For static methods, parameters start at index 0.
        let param_start_index = usize::from(!self.method.is_static());
        let local_offset = index.checked_sub(param_start_index)?;

        let mut current_slot = 0;
        for (param_number, param_type) in self.method.parameters().iter().enumerate() {
            let slot_count = match param_type {
                FieldType::Base(
                    ristretto_classfile::BaseType::Long | ristretto_classfile::BaseType::Double,
                ) => 2,
                _ => 1,
            };

            if local_offset < current_slot + slot_count {
                return Some(format!("<parameter{}>", param_number + 1));
            }
            current_slot += slot_count;
        }
        None
    }

    fn resolve_field(&self, index: u16) -> (String, String, String) {
        let constant_pool = self.class.constant_pool();
        let (class_index, name_and_type_index) = match constant_pool.get(index) {
            Some(Constant::FieldRef {
                class_index,
                name_and_type_index,
            }) => (*class_index, *name_and_type_index),
            _ => return ("?".to_string(), "?".to_string(), "?".to_string()),
        };

        let (name_index, descriptor_index) = match constant_pool.get(name_and_type_index) {
            Some(Constant::NameAndType {
                name_index,
                descriptor_index,
            }) => (*name_index, *descriptor_index),
            _ => return ("?".to_string(), "?".to_string(), "?".to_string()),
        };

        let field_name = constant_pool
            .try_get_utf8(name_index)
            .unwrap_or("?")
            .to_string();
        let descriptor = constant_pool
            .try_get_utf8(descriptor_index)
            .unwrap_or("?")
            .to_string();

        let class_name_index = match constant_pool.get(class_index) {
            Some(Constant::Class(name_index)) => *name_index,
            _ => 0,
        };
        let class_name = constant_pool
            .try_get_utf8(class_name_index)
            .unwrap_or("?")
            .to_string();

        (class_name, field_name, descriptor)
    }

    #[expect(clippy::match_same_arms)]
    fn resolve_method_ref(&self, index: u16) -> (String, String, String) {
        let constant_pool = self.class.constant_pool();
        let (class_index, name_and_type_index) = match constant_pool.get(index) {
            Some(Constant::MethodRef {
                class_index,
                name_and_type_index,
            }) => (*class_index, *name_and_type_index),
            Some(Constant::InterfaceMethodRef {
                class_index,
                name_and_type_index,
            }) => (*class_index, *name_and_type_index),
            _ => return ("?".to_string(), "?".to_string(), "?".to_string()),
        };

        let (name_index, descriptor_index) = match constant_pool.get(name_and_type_index) {
            Some(Constant::NameAndType {
                name_index,
                descriptor_index,
            }) => (*name_index, *descriptor_index),
            _ => return ("?".to_string(), "?".to_string(), "?".to_string()),
        };

        let method_name = constant_pool
            .try_get_utf8(name_index)
            .unwrap_or("?")
            .to_string();
        let descriptor = constant_pool
            .try_get_utf8(descriptor_index)
            .unwrap_or("?")
            .to_string();

        let class_name_index = match constant_pool.get(class_index) {
            Some(Constant::Class(name_index)) => *name_index,
            _ => 0,
        };
        let class_name = constant_pool
            .try_get_utf8(class_name_index)
            .unwrap_or("?")
            .to_string();

        (class_name, method_name, descriptor)
    }

    #[expect(clippy::unused_self)]
    fn count_slots(&self, descriptor: &str) -> usize {
        let mut slot_count = 0;
        let mut chars = descriptor.chars();
        if chars.next() != Some('(') {
            return 0;
        }
        let mut array_depth = 0;

        while let Some(c) = chars.next() {
            match c {
                ')' => break,
                '[' => {
                    array_depth += 1;
                    continue;
                }
                'L' => {
                    for n in chars.by_ref() {
                        if n == ';' {
                            break;
                        }
                    }
                    slot_count += 1;
                }
                'D' | 'J' => {
                    if array_depth == 0 {
                        slot_count += 2;
                    } else {
                        slot_count += 1;
                    }
                }
                _ => {
                    slot_count += 1;
                }
            }
            array_depth = 0;
        }
        slot_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_describe_source_local_without_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("java/lang/Object").await.expect("class");
        let method = class.method("<init>", "()V").expect("method");
        let analyzer = NpeAnalyzer::new(thread, class, method);

        let source = Source::Local(5);
        assert_eq!(analyzer.describe_source(&source), "\"<local5>\"");
    }

    #[tokio::test]
    async fn test_describe_source_this() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("java/lang/Object").await.expect("class");
        let method = class.method("<init>", "()V").expect("method");
        let analyzer = NpeAnalyzer::new(thread, class, method);

        let source = Source::This;
        assert_eq!(analyzer.describe_source(&source), "\"this\"");
    }

    #[tokio::test]
    async fn test_describe_source_field_no_receiver() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("java/lang/Object").await.expect("class");
        let method = class.method("<init>", "()V").expect("method");
        let analyzer = NpeAnalyzer::new(thread, class, method);

        let source = Source::Field("myField".to_string(), None);
        assert_eq!(analyzer.describe_source(&source), "\"myField\"");
    }

    #[tokio::test]
    async fn test_describe_source_field_with_local_receiver() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("java/lang/Object").await.expect("class");
        let method = class.method("<init>", "()V").expect("method");
        let analyzer = NpeAnalyzer::new(thread, class, method);

        let source = Source::Field("myField".to_string(), Some(Box::new(Source::Local(2))));
        assert_eq!(analyzer.describe_source(&source), "\"<local2>.myField\"");
    }

    #[tokio::test]
    async fn test_describe_source_field_with_this_receiver() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("java/lang/Object").await.expect("class");
        let method = class.method("<init>", "()V").expect("method");
        let analyzer = NpeAnalyzer::new(thread, class, method);

        let source = Source::Field("myField".to_string(), Some(Box::new(Source::This)));
        assert_eq!(analyzer.describe_source(&source), "\"this.myField\"");
    }

    #[tokio::test]
    async fn test_describe_source_field_with_unknown_receiver() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("java/lang/Object").await.expect("class");
        let method = class.method("<init>", "()V").expect("method");
        let analyzer = NpeAnalyzer::new(thread, class, method);

        let source = Source::Field("myField".to_string(), Some(Box::new(Source::Unknown)));
        assert_eq!(analyzer.describe_source(&source), "\"myField\"");
    }

    #[tokio::test]
    async fn test_describe_source_method_return() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("java/lang/Object").await.expect("class");
        let method = class.method("<init>", "()V").expect("method");
        let analyzer = NpeAnalyzer::new(thread, class, method);

        let source = Source::MethodReturn("java/lang/String".to_string(), "getValue".to_string());
        assert_eq!(
            analyzer.describe_source(&source),
            "the return value of \"String.getValue()\""
        );
    }

    #[tokio::test]
    async fn test_describe_source_method_return_simple_class() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("java/lang/Object").await.expect("class");
        let method = class.method("<init>", "()V").expect("method");
        let analyzer = NpeAnalyzer::new(thread, class, method);

        let source = Source::MethodReturn("MyClass".to_string(), "doSomething".to_string());
        assert_eq!(
            analyzer.describe_source(&source),
            "the return value of \"MyClass.doSomething()\""
        );
    }

    #[tokio::test]
    async fn test_describe_source_array_access() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("java/lang/Object").await.expect("class");
        let method = class.method("<init>", "()V").expect("method");
        let analyzer = NpeAnalyzer::new(thread, class, method);

        let source = Source::ArrayAccess(None);
        assert_eq!(analyzer.describe_source(&source), "an array element");
    }

    #[tokio::test]
    async fn test_describe_source_constant() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("java/lang/Object").await.expect("class");
        let method = class.method("<init>", "()V").expect("method");
        let analyzer = NpeAnalyzer::new(thread, class, method);

        let source = Source::Constant;
        assert_eq!(analyzer.describe_source(&source), "\"null\"");
    }

    #[tokio::test]
    async fn test_describe_source_unknown() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("java/lang/Object").await.expect("class");
        let method = class.method("<init>", "()V").expect("method");
        let analyzer = NpeAnalyzer::new(thread, class, method);

        let source = Source::Unknown;
        assert_eq!(analyzer.describe_source(&source), "\"<unknown>\"");
    }

    #[tokio::test]
    async fn test_count_slots_no_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("java/lang/Object").await.expect("class");
        let method = class.method("<init>", "()V").expect("method");
        let analyzer = NpeAnalyzer::new(thread, class, method);

        assert_eq!(analyzer.count_slots("()V"), 0);
    }

    #[tokio::test]
    async fn test_count_slots_single_int() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("java/lang/Object").await.expect("class");
        let method = class.method("<init>", "()V").expect("method");
        let analyzer = NpeAnalyzer::new(thread, class, method);

        assert_eq!(analyzer.count_slots("(I)V"), 1);
    }

    #[tokio::test]
    async fn test_count_slots_single_long() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("java/lang/Object").await.expect("class");
        let method = class.method("<init>", "()V").expect("method");
        let analyzer = NpeAnalyzer::new(thread, class, method);

        assert_eq!(analyzer.count_slots("(J)V"), 2);
    }

    #[tokio::test]
    async fn test_count_slots_single_double() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("java/lang/Object").await.expect("class");
        let method = class.method("<init>", "()V").expect("method");
        let analyzer = NpeAnalyzer::new(thread, class, method);

        assert_eq!(analyzer.count_slots("(D)V"), 2);
    }

    #[tokio::test]
    async fn test_count_slots_object_param() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("java/lang/Object").await.expect("class");
        let method = class.method("<init>", "()V").expect("method");
        let analyzer = NpeAnalyzer::new(thread, class, method);

        assert_eq!(analyzer.count_slots("(Ljava/lang/String;)V"), 1);
    }

    #[tokio::test]
    async fn test_count_slots_array_param() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("java/lang/Object").await.expect("class");
        let method = class.method("<init>", "()V").expect("method");
        let analyzer = NpeAnalyzer::new(thread, class, method);

        assert_eq!(analyzer.count_slots("([I)V"), 1);
    }

    #[tokio::test]
    async fn test_count_slots_array_of_long() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("java/lang/Object").await.expect("class");
        let method = class.method("<init>", "()V").expect("method");
        let analyzer = NpeAnalyzer::new(thread, class, method);

        // An array of long takes 1 slot (the array reference)
        assert_eq!(analyzer.count_slots("([J)V"), 1);
    }

    #[tokio::test]
    async fn test_count_slots_multiple_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("java/lang/Object").await.expect("class");
        let method = class.method("<init>", "()V").expect("method");
        let analyzer = NpeAnalyzer::new(thread, class, method);

        // int, long, Object = 1 + 2 + 1 = 4
        assert_eq!(analyzer.count_slots("(IJLjava/lang/Object;)V"), 4);
    }

    #[tokio::test]
    async fn test_count_slots_complex() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("java/lang/Object").await.expect("class");
        let method = class.method("<init>", "()V").expect("method");
        let analyzer = NpeAnalyzer::new(thread, class, method);

        // byte, char, double, float, int, long, short, boolean, Object
        // 1 + 1 + 2 + 1 + 1 + 2 + 1 + 1 + 1 = 11
        assert_eq!(analyzer.count_slots("(BCDFIJSZLjava/lang/Object;)V"), 11);
    }

    #[tokio::test]
    async fn test_count_slots_invalid_descriptor() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("java/lang/Object").await.expect("class");
        let method = class.method("<init>", "()V").expect("method");
        let analyzer = NpeAnalyzer::new(thread, class, method);

        // Invalid descriptor without opening paren
        assert_eq!(analyzer.count_slots("V"), 0);
    }

    #[tokio::test]
    async fn test_count_slots_2d_array() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("java/lang/Object").await.expect("class");
        let method = class.method("<init>", "()V").expect("method");
        let analyzer = NpeAnalyzer::new(thread, class, method);

        // 2D array of int takes 1 slot
        assert_eq!(analyzer.count_slots("([[I)V"), 1);
    }

    #[tokio::test]
    async fn test_get_extended_npe_message_empty_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_extended_npe_message(thread, Parameters::default()).await;
        assert!(result.is_ok());
        let value = result.unwrap();
        assert!(matches!(value, Some(Value::Object(None))));
    }

    #[tokio::test]
    async fn test_get_extended_npe_message_non_object() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Int(42));
        let result = get_extended_npe_message(thread, params).await;
        assert!(result.is_ok());
        let value = result.unwrap();
        assert!(matches!(value, Some(Value::Object(None))));
    }

    #[tokio::test]
    async fn test_get_extended_npe_message_null_object() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push(Value::Object(None));
        let result = get_extended_npe_message(thread, params).await;
        assert!(result.is_ok());
        let value = result.unwrap();
        assert!(matches!(value, Some(Value::Object(None))));
    }

    #[test]
    fn test_source_clone_local() {
        let source = Source::Local(5);
        let cloned = source.clone();
        assert!(matches!(cloned, Source::Local(5)));
    }

    #[test]
    fn test_source_clone_this() {
        let source = Source::This;
        let cloned = source.clone();
        assert!(matches!(cloned, Source::This));
    }

    #[test]
    fn test_source_clone_field() {
        let source = Source::Field("test".to_string(), Some(Box::new(Source::This)));
        let cloned = source.clone();
        if let Source::Field(name, receiver) = cloned {
            assert_eq!(name, "test");
            assert!(receiver.is_some());
        } else {
            panic!("Expected Source::Field");
        }
    }

    #[test]
    fn test_source_clone_method_return() {
        let source = Source::MethodReturn("Class".to_string(), "method".to_string());
        let cloned = source.clone();
        if let Source::MethodReturn(class_name, method_name) = cloned {
            assert_eq!(class_name, "Class");
            assert_eq!(method_name, "method");
        } else {
            panic!("Expected Source::MethodReturn");
        }
    }

    #[test]
    fn test_source_clone_array_access() {
        let source = Source::ArrayAccess(Some(Box::new(Source::Local(1))));
        let cloned = source.clone();
        assert!(matches!(cloned, Source::ArrayAccess(Some(_))));
    }

    #[test]
    fn test_source_clone_constant() {
        let source = Source::Constant;
        let cloned = source.clone();
        assert!(matches!(cloned, Source::Constant));
    }

    #[test]
    fn test_source_clone_unknown() {
        let source = Source::Unknown;
        let cloned = source.clone();
        assert!(matches!(cloned, Source::Unknown));
    }

    #[test]
    fn test_source_debug_local() {
        let source = Source::Local(3);
        let debug_str = format!("{source:?}");
        assert!(debug_str.contains("Local"));
        assert!(debug_str.contains('3'));
    }

    #[test]
    fn test_source_debug_this() {
        let source = Source::This;
        let debug_str = format!("{source:?}");
        assert_eq!(debug_str, "This");
    }

    #[test]
    fn test_source_debug_field() {
        let source = Source::Field("fieldName".to_string(), None);
        let debug_str = format!("{source:?}");
        assert!(debug_str.contains("Field"));
        assert!(debug_str.contains("fieldName"));
    }

    #[test]
    fn test_source_debug_method_return() {
        let source = Source::MethodReturn("ClassName".to_string(), "methodName".to_string());
        let debug_str = format!("{source:?}");
        assert!(debug_str.contains("MethodReturn"));
        assert!(debug_str.contains("ClassName"));
        assert!(debug_str.contains("methodName"));
    }

    #[test]
    fn test_source_debug_array_access() {
        let source = Source::ArrayAccess(None);
        let debug_str = format!("{source:?}");
        assert!(debug_str.contains("ArrayAccess"));
    }

    #[test]
    fn test_source_debug_constant() {
        let source = Source::Constant;
        let debug_str = format!("{source:?}");
        assert_eq!(debug_str, "Constant");
    }

    #[test]
    fn test_source_debug_unknown() {
        let source = Source::Unknown;
        let debug_str = format!("{source:?}");
        assert_eq!(debug_str, "Unknown");
    }

    #[tokio::test]
    async fn test_get_local_name_returns_none() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("java/lang/Object").await.expect("class");
        let method = class.method("<init>", "()V").expect("method");
        let analyzer = NpeAnalyzer::new(thread, class, method);

        assert!(analyzer.get_local_name(0).is_none());
        assert!(analyzer.get_local_name(1).is_none());
        assert!(analyzer.get_local_name(100).is_none());
    }

    #[tokio::test]
    async fn test_npe_analyzer_new() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("java/lang/Object").await.expect("class");
        let method = class.method("<init>", "()V").expect("method");

        // Just verify it can be created without panicking
        let _analyzer = NpeAnalyzer::new(thread, class, method);
    }

    #[tokio::test]
    async fn test_analyze_returns_none_for_invalid_bci() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("java/lang/Object").await.expect("class");
        let method = class.method("<init>", "()V").expect("method");
        let analyzer = NpeAnalyzer::new(thread, class, method);

        // Very large BCI that doesn't exist
        let result = analyzer.analyze(999_999);
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_describe_source_field_with_field_receiver() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("java/lang/Object").await.expect("class");
        let method = class.method("<init>", "()V").expect("method");
        let analyzer = NpeAnalyzer::new(thread, class, method);

        // Field accessed through another field
        let inner_field = Source::Field("inner".to_string(), None);
        let source = Source::Field("outer".to_string(), Some(Box::new(inner_field)));
        // Since inner is not Local or This, it just returns the field name
        assert_eq!(analyzer.describe_source(&source), "\"outer\"");
    }

    #[tokio::test]
    async fn test_describe_source_field_with_method_return_receiver() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("java/lang/Object").await.expect("class");
        let method = class.method("<init>", "()V").expect("method");
        let analyzer = NpeAnalyzer::new(thread, class, method);

        let method_return = Source::MethodReturn("Class".to_string(), "getObj".to_string());
        let source = Source::Field("field".to_string(), Some(Box::new(method_return)));
        // Since receiver is not Local, it just returns the field name
        assert_eq!(analyzer.describe_source(&source), "\"field\"");
    }

    #[tokio::test]
    async fn test_describe_source_field_with_array_access_receiver() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("java/lang/Object").await.expect("class");
        let method = class.method("<init>", "()V").expect("method");
        let analyzer = NpeAnalyzer::new(thread, class, method);

        let array_access = Source::ArrayAccess(None);
        let source = Source::Field("field".to_string(), Some(Box::new(array_access)));
        assert_eq!(analyzer.describe_source(&source), "\"field\"");
    }

    #[tokio::test]
    async fn test_describe_source_field_with_constant_receiver() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("java/lang/Object").await.expect("class");
        let method = class.method("<init>", "()V").expect("method");
        let analyzer = NpeAnalyzer::new(thread, class, method);

        let constant = Source::Constant;
        let source = Source::Field("field".to_string(), Some(Box::new(constant)));
        assert_eq!(analyzer.describe_source(&source), "\"field\"");
    }
}
