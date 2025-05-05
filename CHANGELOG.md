# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## `ristretto_cli` - [0.18.0](https://github.com/theseus-rs/ristretto/compare/v0.17.0...v0.18.0) - 2025-05-05

### Fixed
- improve jit error handling
- update JIT compiler to retain state between invocations and define unique function names
- correct long and double jit locals

### Other
- add java compatibility tests
- update invokeinterface test to expect interface method
- Fix bug where Instruction::stack_delta panics when processing an InvokeInterface because it calls the wrong try_get*ref function ([#319](https://github.com/theseus-rs/ristretto/pull/319))
- add javac -parameters compiler flag to include parameter metadata in class files
- update jit to track JVM locals as cranelift variables

## `ristretto_vm` - [0.18.0](https://github.com/theseus-rs/ristretto/compare/ristretto_vm-v0.17.0...ristretto_vm-v0.18.0) - 2025-05-05

### Fixed
- update JIT compiler to retain state between invocations and define unique function names
- improve jit error handling
- correct long and double jit locals

### Other
- add javac -parameters compiler flag to include parameter metadata in class files
- update jit to track JVM locals as cranelift variables
- add java compatibility tests

## `ristretto_jit` - [0.18.0](https://github.com/theseus-rs/ristretto/compare/ristretto_jit-v0.17.0...ristretto_jit-v0.18.0) - 2025-05-05

### Added
- implement jit dcmpl, dcmpg, fcmpl, fcmpg
- implement jit lcmp

### Fixed
- update JIT compiler to retain state between invocations and define unique function names
- improve jit error handling
- correct long and double jit locals

### Other
- update jit to track JVM locals as cranelift variables

## `ristretto_classfile` - [0.18.0](https://github.com/theseus-rs/ristretto/compare/ristretto_classfile-v0.17.0...ristretto_classfile-v0.18.0) - 2025-05-05

### Other
- update invokeinterface test to expect interface method
- Fix bug where Instruction::stack_delta panics when processing an InvokeInterface because it calls the wrong try_get*ref function ([#319](https://github.com/theseus-rs/ristretto/pull/319))

## `ristretto_cli` - [0.17.0](https://github.com/theseus-rs/ristretto/compare/v0.16.0...v0.17.0) - 2025-04-10

### Added
- add initial jit compilier

### Other
- remove custom method optimizations

## `ristretto_vm` - [0.17.0](https://github.com/theseus-rs/ristretto/compare/ristretto_vm-v0.16.0...ristretto_vm-v0.17.0) - 2025-04-10

### Added
- add initial jit compilier

### Other
- remove custom method optimizations

## `ristretto_jit` - [0.17.0](https://github.com/theseus-rs/ristretto/releases/tag/ristretto_jit-v0.17.0) - 2025-04-10

### Added
- add initial jit compilier

## `ristretto_classloader` - [0.17.0](https://github.com/theseus-rs/ristretto/compare/ristretto_classloader-v0.16.0...ristretto_classloader-v0.17.0) - 2025-04-10

### Added
- add initial jit compilier

## `ristretto_classfile` - [0.17.0](https://github.com/theseus-rs/ristretto/compare/ristretto_classfile-v0.16.0...ristretto_classfile-v0.17.0) - 2025-04-10

### Added
- add initial jit compilier

## `ristretto_cli` - [0.16.0](https://github.com/theseus-rs/ristretto/compare/v0.15.0...v0.16.0) - 2025-04-04

### Added
- add MaxLocals trait for calculating method max_locals values

### Other
- update Cargo.lock dependencies
- [**breaking**] create MaxStack trait and refactor Instruction::stack_utilization() to Instruction::stack_delta(&ConstantPool) to more accurately calculate the maximum stack size

## `ristretto_vm` - [0.16.0](https://github.com/theseus-rs/ristretto/compare/ristretto_vm-v0.15.0...ristretto_vm-v0.16.0) - 2025-04-04

### Other
- [**breaking**] create MaxStack trait and refactor Instruction::stack_utilization() to Instruction::stack_delta(&ConstantPool) to more accurately calculate the maximum stack size

## `ristretto_classloader` - [0.16.0](https://github.com/theseus-rs/ristretto/compare/ristretto_classloader-v0.15.0...ristretto_classloader-v0.16.0) - 2025-04-04

### Other
- [**breaking**] create MaxStack trait and refactor Instruction::stack_utilization() to Instruction::stack_delta(&ConstantPool) to more accurately calculate the maximum stack size

## `ristretto_classfile` - [0.16.0](https://github.com/theseus-rs/ristretto/compare/ristretto_classfile-v0.15.0...ristretto_classfile-v0.16.0) - 2025-04-04

### Added
- add MaxLocals trait for calculating method max_locals values

### Other
- [**breaking**] create MaxStack trait and refactor Instruction::stack_utilization() to Instruction::stack_delta(&ConstantPool) to more accurately calculate the maximum stack size

## `ristretto_cli` - [0.15.0](https://github.com/theseus-rs/ristretto/compare/v0.14.0...v0.15.0) - 2025-04-04

### Added
- add Instruction::stack_utilization()
- add memory usage in debug output
- optimize obtaining invoke parameters from the stack

### Fixed
- remove unnecessary Reference.class_name() string

### Other
- update JVM specification links to point to Java 24
- correct wasm build configuration
- update to Rust 1.86.0
- encapsulate use of ConcurrentVec
- correct sysinfo physical_core_count() call

## `ristretto_vm` - [0.15.0](https://github.com/theseus-rs/ristretto/compare/ristretto_vm-v0.14.0...ristretto_vm-v0.15.0) - 2025-04-04

### Added
- add memory usage in debug output
- optimize obtaining invoke parameters from the stack

### Fixed
- remove unnecessary Reference.class_name() string

### Other
- update JVM specification links to point to Java 24
- correct sysinfo physical_core_count() call
- correct wasm build configuration
- encapsulate use of ConcurrentVec

## `ristretto_classloader` - [0.15.0](https://github.com/theseus-rs/ristretto/compare/ristretto_classloader-v0.14.0...ristretto_classloader-v0.15.0) - 2025-04-04

### Fixed
- remove unnecessary Reference.class_name() string

### Other
- update JVM specification links to point to Java 24
- update to Rust 1.86.0
- correct wasm build configuration
- encapsulate use of ConcurrentVec

## `ristretto_classfile` - [0.15.0](https://github.com/theseus-rs/ristretto/compare/ristretto_classfile-v0.14.0...ristretto_classfile-v0.15.0) - 2025-04-04

### Added
- add Instruction::stack_utilization()

### Other
- update JVM specification links to point to Java 24
- update to Rust 1.86.0

## `ristretto_cli` - [0.14.0](https://github.com/theseus-rs/ristretto/compare/v0.13.1...v0.14.0) - 2025-03-04

### Added
- implement jdk.internal.reflect.ConstantPool
- implement reflection constructors and methods
- add java.lang.Class reflection annotations for fields, constructors and methods
- implement java.io.Console encoding() and istty()
- add java.lang.Class reflection signatures for fields, constructors and methods
- implement sun.reflect.ConstantPool
- implement jdk.internal.reflect.ConstantPool
- implement java.lang.Class.getConstantPool()
- implement java.lang.Class getRawAnnotations()[B and getRawTypeAnnotations()[B

### Fixed
- correct jdk.internal.misc.Unsafe get_reference_type() byte array types
- udpate java.lang.Class.getSuperclass() to return null
- correct field and method annotation length encoding
- return stub boot ModuleLayer
- correct invokespecial to handle interface methods
- correct handling of null annotations
- correct class annotation modifier
- correct java.lang.Class.getModifiers() to return ENUM flag
- correct bugs in java.lang.Class.getSuperclass()

### Other
- update Cargo.toml dependencies

## `ristretto_vm` - [0.14.0](https://github.com/theseus-rs/ristretto/compare/ristretto_vm-v0.13.1...ristretto_vm-v0.14.0) - 2025-03-04

### Added
- implement reflection constructors and methods
- implement java.io.Console encoding() and istty()
- add java.lang.Class reflection signatures for fields, constructors and methods
- add java.lang.Class reflection annotations for fields, constructors and methods
- implement sun.reflect.ConstantPool
- implement jdk.internal.reflect.ConstantPool
- implement jdk.internal.reflect.ConstantPool
- implement java.lang.Class.getConstantPool()
- implement java.lang.Class getRawAnnotations()[B and getRawTypeAnnotations()[B

### Fixed
- correct field and method annotation length encoding
- return stub boot ModuleLayer
- correct invokespecial to handle interface methods
- correct handling of null annotations
- correct class annotation modifier
- correct jdk.internal.misc.Unsafe get_reference_type() byte array types
- correct java.lang.Class.getModifiers() to return ENUM flag
- udpate java.lang.Class.getSuperclass() to return null
- correct bugs in java.lang.Class.getSuperclass()

## `ristretto_classloader` - [0.14.0](https://github.com/theseus-rs/ristretto/compare/ristretto_classloader-v0.13.1...ristretto_classloader-v0.14.0) - 2025-03-04

### Added
- implement reflection constructors and methods
- add java.lang.Class reflection annotations for fields, constructors and methods

## `ristretto_classfile` - [0.14.0](https://github.com/theseus-rs/ristretto/compare/ristretto_classfile-v0.13.1...ristretto_classfile-v0.14.0) - 2025-03-04

### Added
- implement jdk.internal.reflect.ConstantPool

### Fixed
- correct jdk.internal.misc.Unsafe get_reference_type() byte array types
- udpate java.lang.Class.getSuperclass() to return null

## `ristretto_vm` - [0.13.1](https://github.com/theseus-rs/ristretto/compare/ristretto_vm-v0.13.0...ristretto_vm-v0.13.1) - 2025-02-20

### Added
- update to Rust 2024 edition
- add bool support to parameters
- add Java 24 support

### Fixed
- update java.lang.invoke.MethodHandleNatives.resolve() to support fields
- make class object singleton
- correct if_acmpeq and if_acmpne instructions when comparing class references
- remove unnecessary clone in values returned from frame

### Other
- add internal thread print_stack_trace() function for debugging
- update to getrandom=0.3.1
- refactor Class constructor functions to return Arc<Class>
- update dependencies

## `ristretto_cli` - [0.13.1](https://github.com/theseus-rs/ristretto/compare/v0.13.0...v0.13.1) - 2025-02-20

### Added
- update to Rust 2024 edition
- add Java 24 support
- add bool support to parameters

### Fixed
- make class object singleton
- update java.lang.invoke.MethodHandleNatives.resolve() to support fields
- correct if_acmpeq and if_acmpne instructions when comparing class references
- remove unnecessary clone in values returned from frame

### Other
- refactor Class constructor functions to return Arc<Class>
- add internal thread print_stack_trace() function for debugging
- update to getrandom=0.3.1
- update dependencies

## `ristretto_classfile` - [0.13.1](https://github.com/theseus-rs/ristretto/compare/ristretto_classfile-v0.13.0...ristretto_classfile-v0.13.1) - 2025-02-20

### Added
- update to Rust 2024 edition

## `ristretto_classloader` - [0.13.1](https://github.com/theseus-rs/ristretto/compare/ristretto_classloader-v0.13.0...ristretto_classloader-v0.13.1) - 2025-02-20

### Added
- update to Rust 2024 edition
- add Java 24 support

### Fixed
- make class object singleton

### Other
- refactor Class constructor functions to return Arc<Class>

##
`ristretto_vm` - [0.13.0](https://github.com/theseus-rs/ristretto/compare/ristretto_vm-v0.12.3...ristretto_vm-v0.13.0) - 2025-01-24

### Fixed

- correct exception byte to instruction offset conversion error
- remove unnecessary parameter cloning
- remove unnecessary clone from LocalVariables
- correct bug in java.lang.Class.getDeclaredMethods0() where class constructors were incorrectly returned

### Other

- add vm benchmarks
- update VM to support LTS and latest Java versions only
- update java runtime versions
- move invokedynamic into separate module

##
`ristretto_cli` - [0.13.0](https://github.com/theseus-rs/ristretto/compare/ristretto-v0.12.3...ristretto-v0.13.0) - 2025-01-24

### Added

- add Java 25 class file support

### Fixed

- correct StackFrame offset delta byte to instruction mapping
- correct exception byte to instruction offset conversion error
- remove unncessary parameter cloning
- remove unnecessary clone from LocalVariables
- correct bug in java.lang.Class.getDeclaredMethods0() where class constructors were incorrectly returned

### Other

- update Cargo.toml dependencies
- add enum documentation
- add vm benchmarks
- update VM to support LTS and latest Java versions only
- update java runtime versions
- move invokedynamic into separate module

##
`ristretto_classfile` - [0.13.0](https://github.com/theseus-rs/ristretto/compare/ristretto_classfile-v0.12.3...ristretto_classfile-v0.13.0) - 2025-01-24

### Added

- add Java 25 class file support

### Fixed

- correct StackFrame offset delta byte to instruction mapping
- correct exception byte to instruction offset conversion error

### Other

- add enum documentation

##
`ristretto_classloader` - [0.13.0](https://github.com/theseus-rs/ristretto/compare/ristretto_classloader-v0.12.3...ristretto_classloader-v0.13.0) - 2025-01-24

### Other

- add vm benchmarks
- update VM to support LTS and latest Java versions only
- update java runtime versions

## `ristretto` - [v0.12.3](https://github.com/theseus-rs/ristretto/compare/v0.12.2...v0.12.3) - 2025-01-20

### Build

- upgrade cargo dist to 0.27.0
- *(deps)* bump reqwest from 0.12.11 to 0.12.12
- update to Rust 1.84.0
- update bitflags, clap, thiserror and cargo dist
- run code coverage on mac arm64
- update native methods for windows

### Chore

- display JAVA_VERSION or default java version for the CLI --version option
- consolidate method construction
- update dirs and convert-case
- replace hard coded paths with join()
- Release

### Feat

- implement java.lang.Class.getDeclaredMethods0(Z)[Ljava/lang/reflect/Method;
- implement Value::TryInto and Reference::TryInto for class vec

### Fix

- remove unnecessary use of Arc from LocalVariables
- correct null handling of caller argument in MethodHandleNatives.resolve()
- implement java.lang.ProcessHandleImpl
- implement sun.nio.ch.IOUtil.iovMax()I and sun.nio.ch.IOUtil.writeMax()J
- implement java.lang.Class.getInterfaces0()[Ljava/lang/Class;
- implement java.lang.Class.getDeclaredClasses0()[Ljava/lang/Class;
- implement java.lang.ref.Finalizer
- implement java.lang.Class.getDeclaredConstructors0(Z)[Ljava/lang/reflect/Constructor;
- correct invalid return value for java.lang.Class.initClassName()Ljava/lang/String;
- correct java.lang.Class.getComponentType()
- remove unnecessary clone() for Value::TryInto and Reference::TryInto

### Refactor

- encapsulate LocalVariables to Frame and remove unnecessary uses of Arc/RwLock
- encapsulate OperandStack to Frame and remove unnecessary uses of Arc/RwLock
- limit method creation to require ClassFile references
- rename Arguments struct to Parameters

### Test

- improve test coverage
- verify native method definitions
- verify native method definitions
- verify native method definitions
- add sun.* native method tests

## `ristretto` - [v0.12.2](https://github.com/theseus-rs/ristretto/compare/v0.12.1...v0.12.2) - 2024-12-13

### Build

- update x86_64-apple-darwin release to use macos-13
- correct wasm32-unknown-unknown build
- update serde and thiserror
- conditionally register macos native functions
- upgrade to cargo dist 0.26.0 and add new builds
- upgrade to cargo dist 0.26.1
- remove failing aarch64-pc-windows-msvc build
- enable auditable, cyclonedx and github attestations
- remove failing aarch64-unknown-linux-gnu build

### Chore

- Release

### Feat

- optimize java.lang.Math and java.lang.StrictMath methods with Rust implementations

### Fix

- implement java.lang.StrictMath
- correct exception handling class check
- update java.lang.Class.getEnclosingMethod0() to use the class file EnclosingMethod attribute per 4.7.7
- update java.lang.Class.getDeclaringClass0() to return null instead of void
- update java.lang.Class.getSimpleBinaryName0() to work with class objects
- add test
- correct value returned from java.lang.Class.getName()
- implement java.lang.Class.getDeclaringClass0()
- implement java.lang.Class.isInstance(Ljava/lang/Object;)Z
- implement java.lang.Class.isInstance(Ljava/lang/Object;)Z
- implement deep clone for java.lang.Object.clone()
- implement jdk.internal.misc.Unsafe.shouldBeInitialized0(Ljava/lang/Class;)Z
- implement java.lang.invoke.MethodHandleNatives.resolve(Ljava/lang/invoke/MemberName;Ljava/lang/Class;IZ)
  Ljava/lang/invoke/MemberName;

### Revert

- revert HelloWorld implementation

## `ristretto` - [v0.12.1](https://github.com/theseus-rs/ristretto/compare/v0.12.0...v0.12.1) - 2024-12-06

### Build

- update dependencies

### Chore

- update to Rust 1.83
- add System.getSecurityManager() and System.setSecurityManager() implementations
- improve todo docs
- Release

### Docs

- add reference to AsyncDrop
- update documentation links to point to latest jvm version

### Fix

- correct getCallerClass() to return null instead of void
- implement java.lang.ClassLoader methods
- implement java.lang.Float.intBitsToFloat(I)F
- implement java.util.concurrent.atomic.AtomicLong.VMSupportsCS8()Z
- implement java.lang.String.intern()Ljava/lang/String;
- correct line number offsets
- add properties to jdk.internal.util.SystemProps$Raw.vmProperties()[Ljava/lang/String;
- correct java.lang.Class.getDeclaredFields0(Z)[Ljava/lang/reflect/Field; to work with latest Java versions
- implement jdk.internal.misc.Unsafe.objectFieldOffset0(Ljava/lang/reflect/Field;)J
- implement numerous jdk.internal.misc.Unsafe get<type>() and put<type>() methods
- correct exception handling by translating ranges to/from bytes/instructions
- implement java.lang.ProcessHandleImpl.getCurrentPid0()J
- correct minimum value handling for ineg and lneg instructions
- correct object array lengt allocation for java.lang.reflect.Array.newArray(Ljava/lang/Class;I)Ljava/lang/Object;
- implement preview features flag

### Refactor

- refactor native methods into a module hierarchy

## `ristretto` - [v0.12.0](https://github.com/theseus-rs/ristretto/compare/v0.11.1...v0.12.0) - 2024-11-29

### Chore

- order object field debug output
- update to Rust 1.83
- Release

### Feat

- optimize class equality check
- add ClassPath.class_names()

### Fix

- add java.lang.Class.getModifiers()I and jdk.internal.reflect.Reflection.getClassAccessFlags(Ljava/lang/Class;)I
- implement Java ArithmeticException for / by zero
- correct instanceof check
- add BaseType.class_name() and FieldType.class_name()
- add java.lang.reflect.Array.newArray(Ljava/lang/Class;I)Ljava/lang/Object;
- set String.hashIsZero to false when creating a new string in Java 17+
- correct Object.clone() to behave as a reference copy
- add java.lang.invoke.MethodHandleNatives.registerNatives()V and jdk.internal.misc.PreviewFeatures.isPreviewEnabled()Z
- update class names to support modules
- stub all native methods
- stub all native Java 11 methods
- stub all native Java 17 methods
- stub all native Java 18 methods
- stub all native Java 19 methods
- stub all native Java 20 methods
- stub all native Java 21 methods
- stub all native Java 22 methods
- stub all native Java 23 methods
- reinstate native method implementations
- reinstate windows native method implementations
- add native method implementations
- implement java.security.AccessController.doPrivileged() methods
- implement java.lang.Class.getDeclaredFields0() and sun.misc.Unsafe.* methods
- correct FieldType.class_name() to handle object arrays and multi-arrays
- correct field initialization
- add Class.fields() and update Class.get_declared_fields_0() to use the class fields instead of the object fields
- split strings on character instead of byte boundaries to support unicode
- correct errors with Java 11 initialization

### Refactor

- [**breaking**] create JavaError to convert Rust error to Java exceptions
- rename Arguments.[pop|push]_object() -> Arguments.[pop|push]_reference()

## `ristretto` - [v0.11.1](https://github.com/theseus-rs/ristretto/compare/v0.11.0...v0.11.1) - 2024-11-17

### Build

- update dist to 0.25.1

### Chore

- Release

### Docs

- update comments

### Fix

- remove unnecessary thread creation for new objects
- add java.lang.Class.forName0(Ljava/lang/String;ZLjava/lang/ClassLoader;Ljava/lang/Class;)Ljava/lang/Class; and
  java.security.AccessController.getStackAccessControlContext()Ljava/security/AccessControlContext;
- add jdk.internal.misc.CDS.getCDSConfigStatus()I
- correct implementation of jdk.internalmisc.Unsafe for int and long
- add Class.componentType for array classes
- add jdk.internal.misc.Unsafe.copyMemory0(Ljava/lang/Object;JLjava/lang/Object;JJ)V
- initialize VM; call System.initPhase2() and System.initPhase3()
- enable legacy Java system property initialization
- add java.lang.Class.getComponentType()Ljava/lang/Class; for Java 8

### Refactor

- add primordial thread
- update method registry to conditionally register methods based on java version

## `ristretto` - [v0.11.0](https://github.com/theseus-rs/ristretto/compare/v0.10.1...v0.11.0) - 2024-11-13

### Build

- update dependencies

### Chore

- allow Unicode-3.0 license
- update dependencies
- Release

### Feat

- [**breaking**] implement TryInto for Value, Reference and Object
- add JavaObject trait
- add VM.new_object()
- add string support to RustValue

### Fix

- add awt native initIDs() methods
- correct is_assignable_from, checkcast and instanceof array handling
- implement jdk.internal.misc.VM.initializeFromArchive(Ljava/lang/Class;)V, jdk.internal.misc.Unsafe.addressSize0()I,
  jdk.internal.misc.Unsafe.isBigEndian0()Z, jdk.internal.misc.Unsafe.unalignedAccess0()Z
- correct primitive class support
- update Unsafe.compareAndSetReference() to support Objects
- add Class.isInterface()Z
- add Class.getClassAccessFlagsRaw0()I, Class.getClassFileVersion0()I and Class.getPermittedSubclasses0()[
  Ljava/lang/Class;
- add Class.getDeclaringClass0()java/lang/Class, Class.getSigners()Ljava/lang/Object;, Class.isHidden()Z,
  Class.setSigners(Ljava/lang/Object;)V

### Refactor

- remove explicit use of box pin
- update parent reference
- [**breaking**] refactor VM.load_class() -> Thread.class() and VM.new_object() -> VM.object()
- delegate VM.object() -> Thread.object()
- [**breaking**] refactor VM invoke interfaces to use RustValue arguments

### Test

- improve test coverage

## `ristretto` - [v0.10.1](https://github.com/theseus-rs/ristretto/compare/v0.10.0...v0.10.1) - 2024-11-01

### Build

- update macos-arm64 build to use macos-15

### Chore

- simplify invokedynamic serde
- update panic message
- Release

### Docs

- update readme

### Feat

- add environment variable support

### Fix

- correct invokedynamic bytecode serialization
- create array class for anewarray instruction
- correct invokedynamic instruction debug logging
- pass command line arguments as a string array to main methods

### Refactor

- refactor VM internals for threading support
- move additional logic into instruction functions
- simplify native method implementations

### Test

- add object debug test

## `ristretto` - [v0.10.0](https://github.com/theseus-rs/ristretto/compare/v0.9.0...v0.10.0) - 2024-10-21

### Build

- update to Rust 1.82.0
- update to cargo-dist 0.23.0

### Chore

- implement async Thread.sleep() and Thread.yield()
- update corretto versions
- update rename failure log level from debug to warn
- remove unnecessary comments
- Release

### Docs

- update readme

### Feat

- implement athrow instruction
- add system properties
- implement multianewarray instruction

### Fix

- remove unnecessary clone of method code
- update implementation to work with wasm32 builds
- correct exception table hanler_pc offsets to/from instructions/bytes
- correct invoke instruction error messages
- [**breaking**] implement java.home system property
- implement jdk.internal.loader.NativeLibraries.findBuiltinLib()
- correct handling of finally blocks
- implement java.io.UnixFileSystem.getBooleanAttributes0()
- implement sun.nio.fs.UnixNativeDispatcher
- implement java.io.WinNTFileSystem.initIDs()
- update cli to print stack trace when a throwable is returned from the vm
- correct bug setting StackTraceElement.declaringClass

### Refactor

- rename RuntimeError to InternalError
- rename CodeException to ExceptionTableEntry to better align with JVM specification naming
- refactor default runtime version constant
- update the usage of BooleanAttributeFlags

### Test

- add tests for object, reference and value
- add test for long ConcurrentVec value
- add test for large ConcurrentVec value
- add class tests
- add class loader tests
- add test coverage

## `ristretto` - [v0.9.0](https://github.com/theseus-rs/ristretto/compare/v0.8.1...v0.9.0) - 2024-10-15

### Build

- correct clippy errors
- update ci.yml for to run code coverage

### Chore

- remove print statement
- update readme to point to crates.io vm package
- Release

### Docs

- update cli readme
- add oranda workspace
- update readme

### Feat

- add support for invoking interface method ref
- implement native method CDS.getRandomSeedForDumping()J
- implement Reflection.getCallerClass()
- update vm to be async
- enable wasm32 builds
- add stack size to frame debug logging

### Fix

- correct class interface loading and association
- corrected bug where lushr instruction was using incorrect mask
- implement Class.is_array
- implement java.io.UnixFileSystem.initIDs()
- implement java.lang.System.identityHashCode()
- implement java.lang.System.mapLibraryName()
- implement Class.getSuperclass() and Class.isAssignableFrom()
- increase windows stack size to 8MB
- [**breaking**] correct invokeinterface instruction class resolution
- stub implementation of java.lang.ref.Reflection.refersTo0()

### Refactor

- update call stack to have interior mutability of frames
- update CallStack to store reference to VM
- update Frame to reference CallStack
- update frame to have interior mutability
- [**breaking**] rename VM.class() to VM.load_class() and VM.load() to VM.class()

### Test

- add invoke method test coverage
- add methods tests

## `ristretto` - [v0.8.1](https://github.com/theseus-rs/ristretto/compare/v0.8.0...v0.8.1) - 2024-10-11

### Chore

- remove unnecessary instanceof option check
- Release

### Docs

- update readme and web docs
- update readme

### Fix

- correct constant pool formatted string values

### Refactor

- remove unused deref from ConcurrentVec

### Test

- add Instruction to_formatted_string() tests
- add Reference eq tests

## `ristretto` - [v0.8.0](https://github.com/theseus-rs/ristretto/compare/v0.7.0...v0.8.0) - 2024-10-10

### Build

- correct linting errors
- correct workspace definition for workspace
- add github web action
- add feature to enable native-tls by default
- update cargo dist to use existing release
- update release configuration
- update default feature to use rustls-tls
- correct deny.yml definition for rustls

### Chore

- add FUNDING.yml
- Release

### Docs

- update readme
- update readme
- update ristretto cli readme

### Feat

- add reserved impdep1 and impdep2 instructions
- [**breaking**] improve class loader interface
- update lookupswitch instruction to use an IndexMap

### Fix

- [**breaking**] removed cycle between ClassLoader and Class

## `ristretto` - [v0.7.0](https://github.com/theseus-rs/ristretto/compare/v0.6.0...v0.7.0) - 2024-08-20

### Build

- correct linting errors
- enable dashmap inline feature
- correct example build failure

### Chore

- Release

### Docs

- update jar documentation

### Feat

- add jar manifest

### Fix

- [**breaking**] correct definition of goto_w and jsr_w definitions

### Refactor

- [**breaking**] optimize jar class loader by deferring class file loading

### Test

- add class loader benchmarks
- improve test coverage

## `ristretto` - [v0.6.0](https://github.com/theseus-rs/ristretto/compare/v0.5.0...v0.6.0) - 2024-08-15

### Build

- update formatting

### Chore

- Release

### Docs

- add runtime class loader example
- update class loader library documentation

### Feat

- initial Java runtime class loader

### Fix

- correct panics with invalid instructions

### Refactor

- [**breaking**] change class loader function names
- expose zip errors when loading classes from jar
- [**breaking**] remove Default trait from class loader
- rename hello_world example to write_class

### Test

- improve test coverage
- update benchmarks to run independently on the main branch

## `ristretto` - [v0.5.0](https://github.com/theseus-rs/ristretto/compare/v0.4.0...v0.5.0) - 2024-08-09

### Build

- build fuzz target once and re-use for all fuzz tests
- add dependency between fuzz jobs
- attempt to reuse fuzz target
- correct fuzz run command
- remove fuzz target pre-build
- add wasm32 builds
- add target to rust install
- add target to cargo commands
- update dependencies to work with wasm32
- update dependencies to work with wasm32
- update dependencies
- correct deny check
- update CI build definitions
- update code coverage build definitions
- remove unnecessary build definition
- fix code coverage reporting
- add missing version to ristretto_classfile dependency
- update ristretto_classfile dependency
- update version to 0.5.0

### Docs

- updated hello world example
- document url feature flag

### Feat

- add java 24 version
- initial classloader crate
- implement parent class loader logic

### Refactor

- replace usages of Box::new() with .into()
- [**breaking**] remove ClassFile.source_file()

### Test

- improve runtime test coverage
- reduce fuzz total time from 60 to 30 seconds
- add deserialization fuzz testing
- remove invalid test
- reduce fuzz test max time
- update max major version

## `ristretto` - [v0.4.0](https://github.com/theseus-rs/ristretto/compare/v0.3.0...v0.4.0) - 2024-07-26

### Build

- update fuzz actions to use nightly rust
- update fuzz actions to use nightly rust
- update rust toolchain install
- update fuzz action to use ubuntu-latest
- attempt to run fuzz tests on macos-14
- change how cargo fuzz is installed
- update fuzz action to use ubuntu-latest
- attempt to run fuzz tests on macos-14
- remove fuzz from workspace
- update .gitignore
- update read_class error test

### Chore

- Release

### Feat

- add ConstantPool try_get_ functions

### Fix

- [**breaking**] correct mutf8 encoding panics discovered with fuzz tests
- corrected numerous bugs surfaced with fuzz testing

### Refactor

- [**breaking**] remove unknown version support

### Test

- improve test coverage
- improve code coverage for read_class example
- improve try_get_ test coverage
- correct fuzz test configuration
- increase fuzz test memory

## `ristretto` - [v0.3.0](https://github.com/theseus-rs/ristretto/compare/v0.2.2...v0.3.0) - 2024-07-24

### Build

- update dependencies
- update codecov/codecov-action to v4

### Chore

- Release

### Feat

- add read class example
- [**breaking**] use instruction enum instead of bytes for code
- improve class string format

### Fix

- correct tableswitch and lookupswitch serialization
- correct instruction to byte conversion
- correct tableswitch and lookupswitch string format

### Refactor

- remove unnecessary .to_string() class in example

### Tests

- update read class test

## `ristretto` - [v0.2.2](https://github.com/theseus-rs/ristretto/compare/v0.2.1...v0.2.2) - 2024-07-18

### Chore

- Release

### Feat

- optimize constant pool access by utilizing a placeholder at index 0

### Fix

- add future unknown Java version support

## `ristretto` - [v0.2.1](https://github.com/theseus-rs/ristretto/compare/v0.2.0...v0.2.1) - 2024-07-14

### Chore

- Release

### Docs

- add helloworld example
- remove unused import from example in docs

### Feat

- improve Display output to include constant pool, code and line table
- add as_code() function to class, method and field access flags
- add methods to the constant pool to simplify adding constants

## `ristretto` - [v0.2.0](https://github.com/theseus-rs/ristretto/compare/v0.1.0...v0.2.0) - 2024-07-10

### Chore

- Release

### Docs

- correct licensing links

### Feat

- add Display implementation to Instruction and ArrayType
- implement Display

### Fix

- added nested class access flags and added Display implementation for all access flags

### Test

- add missing tests to improve coverage

##

`ristretto` - [v0.1.0](https://github.com/theseus-rs/ristretto/compare/39a6addab11003d27a7ba499b932c6464a3707aa...v0.1.0) -
2024-07-09

### Build

- add deny.toml
- add to_bytes and verify benchmarks
- update Cargo.toml configuration for release

### Chore

- Release

### Docs

- update codecov link
- update readme links
- add security policy
- add library documentation
- correct linting errors

### Feat

- initial class file implementation
- add example classfile usage

### Test

- add missing tests to improve coverage
- add missing tests to improve coverage

