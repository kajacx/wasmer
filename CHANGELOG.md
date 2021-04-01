# Changelog

*The format is based on [Keep a Changelog].*

[Keep a Changelog]: http://keepachangelog.com/en/1.0.0/


## **[Unreleased]**

### Added
- [#2103](https://github.com/wasmerio/wasmer/pull/2103) Add middleware (incl. metering) in the C API.
- [#2003](https://github.com/wasmerio/wasmer/pull/2003) Wasmer works with musl, and is built, tested and packaged for musl.
- [#2116](https://github.com/wasmerio/wasmer/pull/2116) Add a package for Windows that is not an installer, but all the `lib` and `include` files as for macOS and Linux.
- [#2153](https://github.com/wasmerio/wasmer/pull/2153) Add a `wasmer_features_t` unstable C API to define features for the engine and the compiler in the Wasm C API.
- [#2123](https://github.com/wasmerio/wasmer/pull/2123) Use `ENABLE_{{compiler_name}}=(0|1)` to resp. force to disable or enable a compiler when running the `Makefile`, e.g. `ENABLE_LLVM=1 make build-wasmer`.
- [#2123](https://github.com/wasmerio/wasmer/pull/2123) `libwasmer` comes with all available compilers per target instead of Cranelift only.
- [#2118](https://github.com/wasmerio/wasmer/pull/2118) Add an unstable non-standard C API to query available engines and compilers.
- [#2135](https://github.com/wasmerio/wasmer/pull/2135) [Documentation](./PACKAGING.md) for linux distribution maintainers

### Changed
- [#2201](https://github.com/wasmerio/wasmer/pull/2201) Implement `loupe::MemoryUsage` for `wasmer::Instance`.
- [#2200](https://github.com/wasmerio/wasmer/pull/2200) Implement `loupe::MemoryUsage` for `wasmer::Module`.
- [#2199](https://github.com/wasmerio/wasmer/pull/2199) Implement `loupe::MemoryUsage` for `wasmer::Store`.
- [#2140](https://github.com/wasmerio/wasmer/pull/2140) Reduce the number of dependencies in the `wasmer.dll` shared library by statically compiling CRT.
- [#2113](https://github.com/wasmerio/wasmer/pull/2113) Bump minimum supported Rust version to 1.49
- [#2144](https://github.com/wasmerio/wasmer/pull/2144) Bump cranelift version to 0.70
- [#2149](https://github.com/wasmerio/wasmer/pull/2144) `wasmer-engine-native` looks for clang-11 instead of clang-10.

### Fixed
- [#2210](https://github.com/wasmerio/wasmer/pull/2210) Fix a memory leak in the Wasm C API in the strings used to identify imports and exports coming from user code.
- [#2108](https://github.com/wasmerio/wasmer/pull/2108) The Object Native Engine generates code that now compiles correctly with C++.
- [#2125](https://github.com/wasmerio/wasmer/pull/2125) Fix RUSTSEC-2021-0023.
- [#2155](https://github.com/wasmerio/wasmer/pull/2155) Fix the implementation of shift and rotate in the LLVM compiler.
- [#2117](https://github.com/wasmerio/wasmer/pull/2117) Formalize API prefixes in the C API. Only unstable functions have been renamed.
- [#2097](https://github.com/wasmerio/wasmer/pull/2097) Fix how string's length is computed in `wasm_cpu_features_add` in the C API.
- [#2101](https://github.com/wasmerio/wasmer/pull/2101) cflags emitted by `wasmer config --pkg-config` are now correct.

## 1.0.2 - 2021-02-04

### Added
- [#2053](https://github.com/wasmerio/wasmer/pull/2053) Implement the non-standard `wasi_get_unordered_imports` function in the C API.
- [#2072](https://github.com/wasmerio/wasmer/pull/2072) Add `wasm_config_set_target`, along with `wasm_target_t`, `wasm_triple_t` and `wasm_cpu_features_t` in the unstable C API.
- [#2059](https://github.com/wasmerio/wasmer/pull/2059) Ability to capture `stdout` and `stderr` with WASI in the C API.
- [#2040](https://github.com/wasmerio/wasmer/pull/2040) Add `InstanceHandle::vmoffsets` to expose the offsets of the `vmctx` region.
- [#2026](https://github.com/wasmerio/wasmer/pull/2026) Expose trap code of a `RuntimeError`, if it's a `Trap`.
- [#2054](https://github.com/wasmerio/wasmer/pull/2054) Add `wasm_config_delete` to the Wasm C API.
- [#2072](https://github.com/wasmerio/wasmer/pull/2072) Added cross-compilation to Wasm C API.

### Changed
- [#2085](https://github.com/wasmerio/wasmer/pull/2085) Update to latest inkwell and LLVM 11.
- [#2037](https://github.com/wasmerio/wasmer/pull/2037) Improved parallelism of LLVM with the Native/Object engine
- [#2012](https://github.com/wasmerio/wasmer/pull/2012) Refactor Singlepass init stack assembly (more performant now)
- [#2036](https://github.com/wasmerio/wasmer/pull/2036) Optimize memory allocated for Function type definitions
- [#2083](https://github.com/wasmerio/wasmer/pull/2083) Mark `wasi_env_set_instance` and `wasi_env_set_memory` as deprecated. You may simply remove the calls with no side-effect.
- [#2056](https://github.com/wasmerio/wasmer/pull/2056) Change back to depend on the `enumset` crate instead of `wasmer_enumset`

### Fixed
- [#2066](https://github.com/wasmerio/wasmer/pull/2066) Include 'extern "C"' in our C headers when included by C++ code.
- [#2090](https://github.com/wasmerio/wasmer/pull/2090) `wasi_env_t` needs to be freed with `wasi_env_delete` in the C API.
- [#2084](https://github.com/wasmerio/wasmer/pull/2084) Avoid calling the function environment finalizer more than once when the environment has been cloned in the C API.
- [#2069](https://github.com/wasmerio/wasmer/pull/2069) Use the new documentation for `include/README.md` in the Wasmer package.
- [#2042](https://github.com/wasmerio/wasmer/pull/2042) Parse more exotic environment variables in `wasmer run`.
- [#2041](https://github.com/wasmerio/wasmer/pull/2041) Documentation diagrams now have a solid white background rather than a transparent background.
- [#2070](https://github.com/wasmerio/wasmer/pull/2070) Do not drain the entire captured stream at first read with `wasi_env_read_stdout` or `_stderr` in the C API.
- [#2058](https://github.com/wasmerio/wasmer/pull/2058) Expose WASI versions to C correctly.
- [#2044](https://github.com/wasmerio/wasmer/pull/2044) Do not build C headers on docs.rs.

## 1.0.1 - 2021-01-12

This release includes a breaking change in the API (changing the trait `enumset::EnumsetType` to `wasmer_enumset::EnumSetType` and changing `enumset::EnumSet` in signatures to `wasmer_enumset::EnumSet` to work around a breaking change introduced by `syn`) but is being released as a minor version because `1.0.0` is also in a broken state due to a breaking change introduced by `syn` which affects `enumset` and thus `wasmer`.

This change is unlikely to affect any users of `wasmer`, but if it does please change uses of the `enumset` crate to the `wasmer_enumset` crate where possible.

### Added
- [#2010](https://github.com/wasmerio/wasmer/pull/2010) A new, experimental, minified build of `wasmer` called `wasmer-headless` will now be included with releases. `wasmer-headless` is the `wasmer` VM without any compilers attached, so it can only run precompiled Wasm modules.
- [#2005](https://github.com/wasmerio/wasmer/pull/2005) Added the arguments `alias` and `optional` to `WasmerEnv` derive's `export` attribute.

### Changed
- [#2006](https://github.com/wasmerio/wasmer/pull/2006) Use `wasmer_enumset`, a fork of the `enumset` crate to work around a breaking change in `syn`
- [#1985](https://github.com/wasmerio/wasmer/pull/1985) Bump minimum supported Rust version to 1.48

### Fixed
- [#2007](https://github.com/wasmerio/wasmer/pull/2007) Fix packaging of wapm on Windows
- [#2005](https://github.com/wasmerio/wasmer/pull/2005) Emscripten is now working again.

## 1.0.0 - 2021-01-05

### Added

- [#1969](https://github.com/wasmerio/wasmer/pull/1969) Added D integration to the README

### Changed
- [#1979](https://github.com/wasmerio/wasmer/pull/1979) `WasmPtr::get_utf8_string` was renamed to `WasmPtr::get_utf8_str` and made `unsafe`.

### Fixed
- [#1979](https://github.com/wasmerio/wasmer/pull/1979) `WasmPtr::get_utf8_string` now returns a `String`, fixing a soundness issue in certain circumstances. The old functionality is available under a new `unsafe` function, `WasmPtr::get_utf8_str`.

## 1.0.0-rc1 - 2020-12-23

### Added

* [#1894](https://github.com/wasmerio/wasmer/pull/1894) Added exports `wasmer::{CraneliftOptLevel, LLVMOptLevel}` to allow using `Cranelift::opt_level` and `LLVM::opt_level` directly via the `wasmer` crate

### Changed

* [#1941](https://github.com/wasmerio/wasmer/pull/1941) Turn `get_remaining_points`/`set_remaining_points` of the `Metering` middleware into free functions to allow using them in an ahead-of-time compilation setup
* [#1955](https://github.com/wasmerio/wasmer/pull/1955) Set `jit` as a default feature of the `wasmer-wasm-c-api` crate
* [#1944](https://github.com/wasmerio/wasmer/pull/1944) Require `WasmerEnv` to be `Send + Sync` even in dynamic functions.
* [#1963](https://github.com/wasmerio/wasmer/pull/1963) Removed `to_wasm_error` in favour of `impl From<BinaryReaderError> for WasmError`
* [#1962](https://github.com/wasmerio/wasmer/pull/1962) Replace `wasmparser::Result<()>` with `Result<(), MiddlewareError>` in middleware, allowing implementors to return errors in `FunctionMiddleware::feed`

### Fixed

- [#1949](https://github.com/wasmerio/wasmer/pull/1949) `wasm_<type>_vec_delete` functions no longer crash when the given vector is uninitialized, in the Wasmer C API
- [#1949](https://github.com/wasmerio/wasmer/pull/1949) The `wasm_frame_vec_t`, `wasm_functype_vec_t`, `wasm_globaltype_vec_t`, `wasm_memorytype_vec_t`, and `wasm_tabletype_vec_t` are now boxed vectors in the Wasmer C API

## 1.0.0-beta2 - 2020-12-16

### Added
 
* [#1916](https://github.com/wasmerio/wasmer/pull/1916) Add the `WASMER_VERSION*` constants with the `wasmer_version*` functions in the Wasmer C API
* [#1867](https://github.com/wasmerio/wasmer/pull/1867) Added `Metering::get_remaining_points` and `Metering::set_remaining_points` 
* [#1881](https://github.com/wasmerio/wasmer/pull/1881) Added `UnsupportedTarget` error to `CompileError`
* [#1908](https://github.com/wasmerio/wasmer/pull/1908) Implemented `TryFrom<Value<T>>` for `i32`/`u32`/`i64`/`u64`/`f32`/`f64`
* [#1927](https://github.com/wasmerio/wasmer/pull/1927) Added mmap support in `Engine::deserialize_from_file` to speed up artifact loading
* [#1911](https://github.com/wasmerio/wasmer/pull/1911) Generalized signature type in `Function::new` and `Function::new_with_env` to accept owned and reference `FunctionType` as well as array pairs. This allows users to define signatures as constants. Implemented `From<([Type; $N], [Type; $M])>` for `FunctionType` to support this.

### Changed

- [#1865](https://github.com/wasmerio/wasmer/pull/1865) Require that implementors of `WasmerEnv` also implement `Send`, `Sync`, and `Clone`.
- [#1851](https://github.com/wasmerio/wasmer/pull/1851) Improve test suite and documentation of the Wasmer C API
- [#1874](https://github.com/wasmerio/wasmer/pull/1874) Set `CompilerConfig` to be owned (following wasm-c-api)
- [#1880](https://github.com/wasmerio/wasmer/pull/1880) Remove cmake dependency for tests
- [#1924](https://github.com/wasmerio/wasmer/pull/1924) Rename reference implementation `wasmer::Tunables` to `wasmer::BaseTunables`. Export trait `wasmer_engine::Tunables` as `wasmer::Tunables`.

### Fixed

- [#1865](https://github.com/wasmerio/wasmer/pull/1865) Fix memory leaks with host function environments.
- [#1870](https://github.com/wasmerio/wasmer/pull/1870) Fixed Trap instruction address maps in Singlepass
* [#1914](https://github.com/wasmerio/wasmer/pull/1914) Implemented `TryFrom<Bytes> for Pages` instead of `From<Bytes> for Pages` to properly handle overflow errors

## 1.0.0-beta1 - 2020-12-01

### Added

- [#1839](https://github.com/wasmerio/wasmer/pull/1839) Added support for Metering Middleware
- [#1837](https://github.com/wasmerio/wasmer/pull/1837) It is now possible to use exports of an `Instance` even after the `Instance` has been freed
- [#1831](https://github.com/wasmerio/wasmer/pull/1831) Added support for Apple Silicon chips (`arm64-apple-darwin`)
- [#1739](https://github.com/wasmerio/wasmer/pull/1739) Improved function environment setup via `WasmerEnv` proc macro.
- [#1649](https://github.com/wasmerio/wasmer/pull/1649) Add outline of migration to 1.0.0 docs.

### Changed

- [#1739](https://github.com/wasmerio/wasmer/pull/1739) Environments passed to host function- must now implement the `WasmerEnv` trait. You can implement it on your existing type with `#[derive(WasmerEnv)]`.
- [#1838](https://github.com/wasmerio/wasmer/pull/1838) Deprecate `WasiEnv::state_mut`: prefer `WasiEnv::state` instead.
- [#1663](https://github.com/wasmerio/wasmer/pull/1663) Function environments passed to host functions now must be passed by `&` instead of `&mut`. This is a breaking change. This change fixes a race condition when a host function is called from multiple threads. If you need mutability in your environment, consider using `std::sync::Mutex` or other synchronization primitives.
- [#1830](https://github.com/wasmerio/wasmer/pull/1830) Minimum supported Rust version bumped to 1.47.0
- [#1810](https://github.com/wasmerio/wasmer/pull/1810) Make the `state` field of `WasiEnv` public

### Fixed

- [#1857](https://github.com/wasmerio/wasmer/pull/1857) Fix dynamic function with new Environment API
- [#1855](https://github.com/wasmerio/wasmer/pull/1855) Fix memory leak when using `wat2wasm` in the C API, the function now takes its output parameter by pointer rather than returning an allocated `wasm_byte_vec_t`.
- [#1841](https://github.com/wasmerio/wasmer/pull/1841) We will now panic when attempting to use a native function with a captured env as a host function. Previously this would silently do the wrong thing. See [#1840](https://github.com/wasmerio/wasmer/pull/1840) for info about Wasmer's support of closures as host functions.
- [#1764](https://github.com/wasmerio/wasmer/pull/1764) Fix bug in WASI `path_rename` allowing renamed files to be 1 directory below a preopened directory.

## 1.0.0-alpha5 - 2020-11-06

### Added

- [#1761](https://github.com/wasmerio/wasmer/pull/1761) Implement the `wasm_trap_t**` argument of `wasm_instance_new` in the Wasm C API.
- [#1687](https://github.com/wasmerio/wasmer/pull/1687) Add basic table example; fix ownership of local memory and local table metadata in the VM.
- [#1751](https://github.com/wasmerio/wasmer/pull/1751) Implement `wasm_trap_t` inside a function declared with `wasm_func_new_with_env` in the Wasm C API.
- [#1741](https://github.com/wasmerio/wasmer/pull/1741) Implement `wasm_memory_type` in the Wasm C API.
- [#1736](https://github.com/wasmerio/wasmer/pull/1736) Implement `wasm_global_type` in the Wasm C API.
- [#1699](https://github.com/wasmerio/wasmer/pull/1699) Update `wasm.h` to its latest version.
- [#1685](https://github.com/wasmerio/wasmer/pull/1685) Implement `wasm_exporttype_delete` in the Wasm C API.
- [#1725](https://github.com/wasmerio/wasmer/pull/1725) Implement `wasm_func_type` in the Wasm C API.
- [#1715](https://github.com/wasmerio/wasmer/pull/1715) Register errors from `wasm_module_serialize` in the Wasm C API.
- [#1709](https://github.com/wasmerio/wasmer/pull/1709) Implement `wasm_module_name` and `wasm_module_set_name` in the Wasm(er) C API.
- [#1700](https://github.com/wasmerio/wasmer/pull/1700) Implement `wasm_externtype_copy` in the Wasm C API.
- [#1785](https://github.com/wasmerio/wasmer/pull/1785) Add more examples on the Rust API.
- [#1783](https://github.com/wasmerio/wasmer/pull/1783) Handle initialized but empty results in `wasm_func_call` in the Wasm C API.
- [#1780](https://github.com/wasmerio/wasmer/pull/1780) Implement new SIMD zero-extend loads in compiler-llvm.
- [#1754](https://github.com/wasmerio/wasmer/pull/1754) Implement aarch64 ABI for compiler-llvm.
- [#1693](https://github.com/wasmerio/wasmer/pull/1693) Add `wasmer create-exe` subcommand.

### Changed

- [#1772](https://github.com/wasmerio/wasmer/pull/1772) Remove lifetime parameter from `NativeFunc`.
- [#1762](https://github.com/wasmerio/wasmer/pull/1762) Allow the `=` sign in a WASI environment variable value.
- [#1710](https://github.com/wasmerio/wasmer/pull/1710) Memory for function call trampolines is now owned by the Artifact.
- [#1781](https://github.com/wasmerio/wasmer/pull/1781) Cranelift upgrade to 0.67.
- [#1777](https://github.com/wasmerio/wasmer/pull/1777) Wasmparser update to 0.65.
- [#1775](https://github.com/wasmerio/wasmer/pull/1775) Improve LimitingTunables implementation.
- [#1720](https://github.com/wasmerio/wasmer/pull/1720) Autodetect llvm regardless of architecture.

### Fixed

- [#1718](https://github.com/wasmerio/wasmer/pull/1718) Fix panic in the API in some situations when the memory's min bound was greater than the memory's max bound.
- [#1731](https://github.com/wasmerio/wasmer/pull/1731) In compiler-llvm always load before store, to trigger any traps before any bytes are written.

## 1.0.0-alpha4 - 2020-10-08

### Added
- [#1635](https://github.com/wasmerio/wasmer/pull/1635) Implement `wat2wasm` in the Wasm C API.
- [#1636](https://github.com/wasmerio/wasmer/pull/1636) Implement `wasm_module_validate` in the Wasm C API.
- [#1657](https://github.com/wasmerio/wasmer/pull/1657) Implement `wasm_trap_t` and `wasm_frame_t` for Wasm C API; add examples in Rust and C of exiting early with a host function.

### Fixed
- [#1690](https://github.com/wasmerio/wasmer/pull/1690) Fix `wasm_memorytype_limits` where `min` and `max` represents pages, not bytes. Additionally, fixes the max limit sentinel value.
- [#1671](https://github.com/wasmerio/wasmer/pull/1671) Fix probestack firing inappropriately, and sometimes over/under allocating stack.
- [#1660](https://github.com/wasmerio/wasmer/pull/1660) Fix issue preventing map-dir aliases starting with `/` from working properly.
- [#1624](https://github.com/wasmerio/wasmer/pull/1624) Add Value::I32/Value::I64 converters from unsigned ints.

### Changed
- [#1682](https://github.com/wasmerio/wasmer/pull/1682) Improve error reporting when making a memory with invalid settings.
- [#1691](https://github.com/wasmerio/wasmer/pull/1691) Bump minimum supported Rust version to 1.46.0
- [#1645](https://github.com/wasmerio/wasmer/pull/1645) Move the install script to https://github.com/wasmerio/wasmer-install

## 1.0.0-alpha3 - 2020-09-14

### Fixed

- [#1620](https://github.com/wasmerio/wasmer/pull/1620) Fix bug causing the Wapm binary to not be packaged with the release
- [#1619](https://github.com/wasmerio/wasmer/pull/1619) Improve error message in engine-native when C compiler is missing

## 1.0.0-alpha02.0 - 2020-09-11

### Added

- [#1566](https://github.com/wasmerio/wasmer/pull/1566) Add support for opening special Unix files to the WASI FS

### Fixed

- [#1602](https://github.com/wasmerio/wasmer/pull/1602) Fix panic when calling host functions with negative numbers in certain situations
- [#1590](https://github.com/wasmerio/wasmer/pull/1590) Fix soundness issue in API of vm::Global

## TODO: 1.0.0-alpha01.0

- Wasmer refactor lands

## 0.17.1 - 2020-06-24

### Changed
- [#1439](https://github.com/wasmerio/wasmer/pull/1439) Move `wasmer-interface-types` into its own repository

### Fixed

- [#1554](https://github.com/wasmerio/wasmer/pull/1554) Update supported stable Rust version to 1.45.2.
- [#1552](https://github.com/wasmerio/wasmer/pull/1552) Disable `sigint` handler by default.

## 0.17.0 - 2020-05-11

### Added
- [#1331](https://github.com/wasmerio/wasmer/pull/1331) Implement the `record` type and instrutions for WIT
- [#1345](https://github.com/wasmerio/wasmer/pull/1345) Adding ARM testing in Azure Pipelines
- [#1329](https://github.com/wasmerio/wasmer/pull/1329) New numbers and strings instructions for WIT
- [#1285](https://github.com/wasmerio/wasmer/pull/1285) Greatly improve errors in `wasmer-interface-types`
- [#1303](https://github.com/wasmerio/wasmer/pull/1303) NaN canonicalization for singlepass backend.
- [#1313](https://github.com/wasmerio/wasmer/pull/1313) Add new high-level public API through `wasmer` crate. Includes many updates including:
  - Minor improvement: `imports!` macro now handles no trailing comma as well as a trailing comma in namespaces and between namespaces.
  - New methods on `Module`: `exports`, `imports`, and `custom_sections`.
  - New way to get exports from an instance with `let func_name: Func<i32, i64> = instance.exports.get("func_name");`.
  - Improved `Table` APIs including `set` which now allows setting functions directly.  TODO: update this more if `Table::get` gets made public in this PR
  - TODO: finish the list of changes here
- [#1305](https://github.com/wasmerio/wasmer/pull/1305) Handle panics from DynamicFunc.
- [#1300](https://github.com/wasmerio/wasmer/pull/1300) Add support for multiple versions of WASI tests: wasitests now test all versions of WASI.
- [#1292](https://github.com/wasmerio/wasmer/pull/1292) Experimental Support for Android (x86_64 and AArch64)

### Fixed
- [#1283](https://github.com/wasmerio/wasmer/pull/1283) Workaround for floating point arguments and return values in `DynamicFunc`s.

### Changed
- [#1401](https://github.com/wasmerio/wasmer/pull/1401) Make breaking change to `RuntimeError`: `RuntimeError` is now more explicit about its possible error values allowing for better insight into why a call into Wasm failed.
- [#1382](https://github.com/wasmerio/wasmer/pull/1382) Refactored test infranstructure (part 2)
- [#1380](https://github.com/wasmerio/wasmer/pull/1380) Refactored test infranstructure (part 1)
- [#1357](https://github.com/wasmerio/wasmer/pull/1357) Refactored bin commands into separate files
- [#1335](https://github.com/wasmerio/wasmer/pull/1335) Change mutability of `memory` to `const` in `wasmer_memory_data_length` in the C API
- [#1332](https://github.com/wasmerio/wasmer/pull/1332) Add option to `CompilerConfig` to force compiler IR verification off even when `debug_assertions` are enabled. This can be used to make debug builds faster, which may be important if you're creating a library that wraps Wasmer and depend on the speed of debug builds.
- [#1320](https://github.com/wasmerio/wasmer/pull/1320) Change `custom_sections` field in `ModuleInfo` to be more standards compliant by allowing multiple custom sections with the same name. To get the old behavior with the new API, you can add `.last().unwrap()` to accesses. For example, `module_info.custom_sections["custom_section_name"].last().unwrap()`.
- [#1301](https://github.com/wasmerio/wasmer/pull/1301) Update supported stable Rust version to 1.41.1.

## 0.16.2 - 2020-03-11

### Fixed

- [#1294](https://github.com/wasmerio/wasmer/pull/1294) Fix bug related to system calls in WASI that rely on reading from WasmPtrs as arrays of length 0. `WasmPtr` will now succeed on length 0 arrays again.

## 0.16.1 - 2020-03-11

### Fixed

- [#1291](https://github.com/wasmerio/wasmer/pull/1291) Fix installation packaging script to package the `wax` command.

## 0.16.0 - 2020-03-11

### Added
- [#1286](https://github.com/wasmerio/wasmer/pull/1286) Updated Windows Wasmer icons. Add wax
- [#1284](https://github.com/wasmerio/wasmer/pull/1284) Implement string and memory instructions in `wasmer-interface-types`

### Fixed
- [#1272](https://github.com/wasmerio/wasmer/pull/1272) Fix off-by-one error bug when accessing memory with a `WasmPtr` that contains the last valid byte of memory. Also changes the behavior of `WasmPtr<T, Array>` with a length of 0 and `WasmPtr<T>` where `std::mem::size_of::<T>()` is 0 to always return `None`

## 0.15.0 - 2020-03-04

- [#1263](https://github.com/wasmerio/wasmer/pull/1263) Changed the behavior of some WASI syscalls to now handle preopened directories more properly. Changed default `--debug` logging to only show Wasmer-related messages.
- [#1217](https://github.com/wasmerio/wasmer/pull/1217) Polymorphic host functions based on dynamic trampoline generation.
- [#1252](https://github.com/wasmerio/wasmer/pull/1252) Allow `/` in wasi `--mapdir` wasm path.
- [#1212](https://github.com/wasmerio/wasmer/pull/1212) Add support for GDB JIT debugging:
  - Add `--generate-debug-info` and `-g` flags to `wasmer run` to generate debug information during compilation. The debug info is passed via the GDB JIT interface to a debugger to allow source-level debugging of Wasm files. Currently only available on clif-backend.
  - Break public middleware APIs: there is now a `source_loc` parameter that should be passed through if applicable.
  - Break compiler trait methods such as `feed_local`, `feed_event` as well as `ModuleCodeGenerator::finalize`.

## 0.14.1 - 2020-02-24

- [#1245](https://github.com/wasmerio/wasmer/pull/1245) Use Ubuntu 16.04 in CI so that we use an earlier version of GLIBC.
- [#1234](https://github.com/wasmerio/wasmer/pull/1234) Check for unused excluded spectest failures.
- [#1232](https://github.com/wasmerio/wasmer/pull/1232) `wasmer-interface-types` has a WAT decoder.

## 0.14.0 - 2020-02-20

- [#1233](https://github.com/wasmerio/wasmer/pull/1233) Improved Wasmer C API release artifacts.
- [#1216](https://github.com/wasmerio/wasmer/pull/1216) `wasmer-interface-types` receives a binary encoder.
- [#1228](https://github.com/wasmerio/wasmer/pull/1228) Singlepass cleanup: Resolve several FIXMEs and remove protect_unix.
- [#1218](https://github.com/wasmerio/wasmer/pull/1218) Enable Cranelift verifier in debug mode. Fix bug with table indices being the wrong type.
- [#787](https://github.com/wasmerio/wasmer/pull/787) New crate `wasmer-interface-types` to implement WebAssembly Interface Types.
- [#1213](https://github.com/wasmerio/wasmer/pull/1213) Fixed WASI `fdstat` to detect `isatty` properly.
- [#1192](https://github.com/wasmerio/wasmer/pull/1192) Use `ExceptionCode` for error representation.
- [#1191](https://github.com/wasmerio/wasmer/pull/1191) Fix singlepass miscompilation on `Operator::CallIndirect`.
- [#1180](https://github.com/wasmerio/wasmer/pull/1180) Fix compilation for target `x86_64-unknown-linux-musl`.
- [#1170](https://github.com/wasmerio/wasmer/pull/1170) Improve the WasiFs builder API with convenience methods for overriding stdin, stdout, and stderr as well as a new sub-builder for controlling the permissions and properties of preopened directories.  Also breaks that implementations of `WasiFile` must be `Send` -- please file an issue if this change causes you any issues.
- [#1161](https://github.com/wasmerio/wasmer/pull/1161) Require imported functions to be `Send`. This is a breaking change that fixes a soundness issue in the API.
- [#1140](https://github.com/wasmerio/wasmer/pull/1140) Use [`blake3`](https://github.com/BLAKE3-team/BLAKE3) as default hashing algorithm for caching.
- [#1129](https://github.com/wasmerio/wasmer/pull/1129) Standard exception types for singlepass backend.

## 0.13.1 - 2020-01-16
- Fix bug in wapm related to the `package.wasmer_extra_flags` entry in the manifest

## 0.13.0 - 2020-01-15

Special thanks to [@repi](https://github.com/repi) and [@srenatus](https://github.com/srenatus) for their contributions!

- [#1153](https://github.com/wasmerio/wasmer/pull/1153) Added Wasmex, an Elixir language integration, to the README
- [#1133](https://github.com/wasmerio/wasmer/pull/1133) New `wasmer_trap` function in the C API, to properly error from within a host function
- [#1147](https://github.com/wasmerio/wasmer/pull/1147) Remove `log` and `trace` macros from `wasmer-runtime-core`, remove `debug` and `trace` features from `wasmer-*` crates, use the `log` crate for logging and use `fern` in the Wasmer CLI binary to output log messages.  Colorized output will be enabled automatically if printing to a terminal, to force colorization on or off, set the `WASMER_COLOR` environment variable to `true` or `false`.
- [#1128](https://github.com/wasmerio/wasmer/pull/1128) Fix a crash when a host function is missing and the `allow_missing_functions` flag is enabled
- [#1099](https://github.com/wasmerio/wasmer/pull/1099) Remove `backend::Backend` from `wasmer_runtime_core`
- [#1097](https://github.com/wasmerio/wasmer/pull/1097) Move inline breakpoint outside of runtime backend
- [#1095](https://github.com/wasmerio/wasmer/pull/1095) Update to cranelift 0.52.
- [#1092](https://github.com/wasmerio/wasmer/pull/1092) Add `get_utf8_string_with_nul` to `WasmPtr` to read nul-terminated strings from memory.
- [#1071](https://github.com/wasmerio/wasmer/pull/1071) Add support for non-trapping float-to-int conversions, enabled by default.

## 0.12.0 - 2019-12-18

Special thanks to [@ethanfrey](https://github.com/ethanfrey), [@AdamSLevy](https://github.com/AdamSLevy), [@Jasper-Bekkers](https://github.com/Jasper-Bekkers), [@srenatus](https://github.com/srenatus) for their contributions!

- [#1078](https://github.com/wasmerio/wasmer/pull/1078) Increase the maximum number of parameters `Func` can take
- [#1062](https://github.com/wasmerio/wasmer/pull/1062) Expose some opt-in Emscripten functions to the C API
- [#1032](https://github.com/wasmerio/wasmer/pull/1032) Change the signature of the Emscripten `abort` function to work with Emscripten 1.38.30
- [#1060](https://github.com/wasmerio/wasmer/pull/1060) Test the capi with all the backends
- [#1069](https://github.com/wasmerio/wasmer/pull/1069) Add function `get_memory_and_data` to `Ctx` to help prevent undefined behavior and mutable aliasing. It allows accessing memory while borrowing data mutably for the `Ctx` lifetime. This new function is now being used in `wasmer-wasi`.
- [#1058](https://github.com/wasmerio/wasmer/pull/1058) Fix minor panic issue when `wasmer::compile_with` called with llvm backend.
- [#858](https://github.com/wasmerio/wasmer/pull/858) Minor panic fix when wasmer binary with `loader` option run a module without exported `_start` function.
- [#1056](https://github.com/wasmerio/wasmer/pull/1056) Improved `--invoke` args parsing (supporting `i32`, `i64`, `f32` and `f32`) in Wasmer CLI
- [#1054](https://github.com/wasmerio/wasmer/pull/1054) Improve `--invoke` output in Wasmer CLI
- [#1053](https://github.com/wasmerio/wasmer/pull/1053) For RuntimeError and breakpoints, use Box<Any + Send> instead of Box<Any>.
- [#1052](https://github.com/wasmerio/wasmer/pull/1052) Fix minor panic and improve Error handling in singlepass backend.
- [#1050](https://github.com/wasmerio/wasmer/pull/1050) Attach C & C++ headers to releases.
- [#1033](https://github.com/wasmerio/wasmer/pull/1033) Set cranelift backend as default compiler backend again, require at least one backend to be enabled for Wasmer CLI
- [#1044](https://github.com/wasmerio/wasmer/pull/1044) Enable AArch64 support in the LLVM backend.
- [#1030](https://github.com/wasmerio/wasmer/pull/1030) Ability to generate `ImportObject` for a specific version WASI version with the C API.
- [#1028](https://github.com/wasmerio/wasmer/pull/1028) Introduce strict/non-strict modes for `get_wasi_version`
- [#1029](https://github.com/wasmerio/wasmer/pull/1029) Add the “floating” `WasiVersion::Latest` version.
- [#1006](https://github.com/wasmerio/wasmer/pull/1006) Fix minor panic issue when `wasmer::compile_with` called with llvm backend
- [#1009](https://github.com/wasmerio/wasmer/pull/1009) Enable LLVM verifier for all tests, add new llvm-backend-tests crate.
- [#1022](https://github.com/wasmerio/wasmer/pull/1022) Add caching support for Singlepass backend.
- [#1004](https://github.com/wasmerio/wasmer/pull/1004) Add the Auto backend to enable to adapt backend usage depending on wasm file executed.
- [#1068](https://github.com/wasmerio/wasmer/pull/1068) Various cleanups for the singlepass backend on AArch64.

## 0.11.0 - 2019-11-22

- [#713](https://github.com/wasmerio/wasmer/pull/713) Add AArch64 support for singlepass.
- [#995](https://github.com/wasmerio/wasmer/pull/995) Detect when a global is read without being initialized (emit a proper error instead of panicking)
- [#996](https://github.com/wasmerio/wasmer/pull/997) Refactored spectests, emtests and wasitests to use default compiler logic
- [#992](https://github.com/wasmerio/wasmer/pull/992) Updates WAPM version to 0.4.1, fix arguments issue introduced in #990
- [#990](https://github.com/wasmerio/wasmer/pull/990) Default wasmer CLI to `run`.  Wasmer will now attempt to parse unrecognized command line options as if they were applied to the run command: `wasmer mywasm.wasm --dir=.` now works!
- [#987](https://github.com/wasmerio/wasmer/pull/987) Fix `runtime-c-api` header files when compiled by gnuc.
- [#957](https://github.com/wasmerio/wasmer/pull/957) Change the meaning of `wasmer_wasi::is_wasi_module` to detect any type of WASI module, add support for new wasi snapshot_preview1
- [#934](https://github.com/wasmerio/wasmer/pull/934) Simplify float expressions in the LLVM backend.

## 0.10.2 - 2019-11-18

- [#968](https://github.com/wasmerio/wasmer/pull/968) Added `--invoke` option to the command
- [#964](https://github.com/wasmerio/wasmer/pull/964) Enable cross-compilation for specific target
- [#971](https://github.com/wasmerio/wasmer/pull/971) In LLVM backend, use unaligned loads and stores for non-atomic accesses to wasmer memory.
- [#960](https://github.com/wasmerio/wasmer/pull/960) Fix `runtime-c-api` header files when compiled by clang.
- [#925](https://github.com/wasmerio/wasmer/pull/925) Host functions can be closures with a captured environment.
- [#917](https://github.com/wasmerio/wasmer/pull/917) Host functions (aka imported functions) may not have `&mut vm::Ctx` as first argument, i.e. the presence of the `&mut vm::Ctx` argument is optional.
- [#915](https://github.com/wasmerio/wasmer/pull/915) All backends share the same definition of `Trampoline` (defined in `wasmer-runtime-core`).

## 0.10.1 - 2019-11-11

- [#952](https://github.com/wasmerio/wasmer/pull/952) Use C preprocessor to properly hide trampoline functions on Windows and non-x86_64 targets.

## 0.10.0 - 2019-11-11

Special thanks to [@newpavlov](https://github.com/newpavlov) and [@Maxgy](https://github.com/Maxgy) for their contributions!

- [#942](https://github.com/wasmerio/wasmer/pull/942) Deny missing docs in runtime core and add missing docs
- [#939](https://github.com/wasmerio/wasmer/pull/939) Fix bug causing attempts to append to files with WASI to delete the contents of the file
- [#940](https://github.com/wasmerio/wasmer/pull/940) Update supported Rust version to 1.38+
- [#923](https://github.com/wasmerio/wasmer/pull/923) Fix memory leak in the C API caused by an incorrect cast in `wasmer_trampoline_buffer_destroy`
- [#921](https://github.com/wasmerio/wasmer/pull/921) In LLVM backend, annotate all memory accesses with TBAA metadata.
- [#883](https://github.com/wasmerio/wasmer/pull/883) Allow floating point operations to have arbitrary inputs, even including SNaNs.
- [#856](https://github.com/wasmerio/wasmer/pull/856) Expose methods in the runtime C API to get a WASI import object

## 0.9.0 - 2019-10-23

Special thanks to @alocquet for their contributions!

- [#898](https://github.com/wasmerio/wasmer/pull/898) State tracking is now disabled by default in the LLVM backend. It can be enabled with `--track-state`.
- [#861](https://github.com/wasmerio/wasmer/pull/861) Add descriptions to `unimplemented!` macro in various places
- [#897](https://github.com/wasmerio/wasmer/pull/897) Removes special casing of stdin, stdout, and stderr in WASI.  Closing these files now works.  Removes `stdin`, `stdout`, and `stderr` from `WasiFS`, replaced by the methods `stdout`, `stdout_mut`, and so on.
- [#863](https://github.com/wasmerio/wasmer/pull/863) Fix min and max for cases involving NaN and negative zero when using the LLVM backend.

## 0.8.0 - 2019-10-02

Special thanks to @jdanford for their contributions!

- [#850](https://github.com/wasmerio/wasmer/pull/850) New `WasiStateBuilder` API. small, add misc. breaking changes to existing API (for example, changing the preopen dirs arg on `wasi::generate_import_object` from `Vec<String>` to `Vec<Pathbuf>`)
- [#852](https://github.com/wasmerio/wasmer/pull/852) Make minor grammar/capitalization fixes to README.md
- [#841](https://github.com/wasmerio/wasmer/pull/841) Slightly improve rustdoc documentation and small updates to outdated info in readme files
- [#836](https://github.com/wasmerio/wasmer/pull/836) Update Cranelift fork version to `0.44.0`
- [#839](https://github.com/wasmerio/wasmer/pull/839) Change supported version to stable Rust 1.37+
- [#834](https://github.com/wasmerio/wasmer/pull/834) Fix panic when unwraping `wasmer` arguments
- [#835](https://github.com/wasmerio/wasmer/pull/835) Add parallel execution example (independent instances created from the same `ImportObject` and `Module` run with rayon)
- [#834](https://github.com/wasmerio/wasmer/pull/834) Fix panic when parsing numerical arguments for no-ABI targets run with the wasmer binary
- [#833](https://github.com/wasmerio/wasmer/pull/833) Add doc example of using ImportObject's new `maybe_with_namespace` method
- [#832](https://github.com/wasmerio/wasmer/pull/832) Delete unused runtime ABI
- [#809](https://github.com/wasmerio/wasmer/pull/809) Fix bugs leading to panics in `LocalBacking`.
- [#831](https://github.com/wasmerio/wasmer/pull/831) Add support for atomic operations, excluding wait and notify, to singlepass.
- [#822](https://github.com/wasmerio/wasmer/pull/822) Update Cranelift fork version to `0.43.1`
- [#829](https://github.com/wasmerio/wasmer/pull/829) Fix deps on `make bench-*` commands; benchmarks don't compile other backends now
- [#807](https://github.com/wasmerio/wasmer/pull/807) Implement Send for `Instance`, breaking change on `ImportObject`, remove method `get_namespace` replaced with `with_namespace` and `maybe_with_namespace`
- [#817](https://github.com/wasmerio/wasmer/pull/817) Add document for tracking features across backends and language integrations, [docs/feature_matrix.md]
- [#823](https://github.com/wasmerio/wasmer/issues/823) Improved Emscripten / WASI integration
- [#821](https://github.com/wasmerio/wasmer/issues/821) Remove patch version on most deps Cargo manifests.  This gives Wasmer library users more control over which versions of the deps they use.
- [#820](https://github.com/wasmerio/wasmer/issues/820) Remove null-pointer checks in `WasmPtr` from runtime-core, re-add them in Emscripten
- [#803](https://github.com/wasmerio/wasmer/issues/803) Add method to `Ctx` to invoke functions by their `TableIndex`
- [#790](https://github.com/wasmerio/wasmer/pull/790) Fix flaky test failure with LLVM, switch to large code model.
- [#788](https://github.com/wasmerio/wasmer/pull/788) Use union merge on the changelog file.
- [#785](https://github.com/wasmerio/wasmer/pull/785) Include Apache license file for spectests.
- [#786](https://github.com/wasmerio/wasmer/pull/786) In the LLVM backend, lower atomic wasm operations to atomic machine instructions.
- [#784](https://github.com/wasmerio/wasmer/pull/784) Fix help string for wasmer run.

## 0.7.0 - 2019-09-12

Special thanks to @YaronWittenstein @penberg for their contributions.

- [#776](https://github.com/wasmerio/wasmer/issues/776) Allow WASI preopened fds to be closed
- [#774](https://github.com/wasmerio/wasmer/issues/774) Add more methods to the `WasiFile` trait
- [#772](https://github.com/wasmerio/wasmer/issues/772) [#770](https://github.com/wasmerio/wasmer/issues/770) Handle more internal failures by passing back errors
- [#756](https://github.com/wasmerio/wasmer/issues/756) Allow NULL parameter and 0 arity in `wasmer_export_func_call` C API
- [#747](https://github.com/wasmerio/wasmer/issues/747) Return error instead of panicking on traps when using the Wasmer binary
- [#741](https://github.com/wasmerio/wasmer/issues/741) Add validate Wasm fuzz target
- [#733](https://github.com/wasmerio/wasmer/issues/733) Remove dependency on compiler backends for `middleware-common`
- [#732](https://github.com/wasmerio/wasmer/issues/732) [#731](https://github.com/wasmerio/wasmer/issues/731) WASI bug fixes and improvements
- [#726](https://github.com/wasmerio/wasmer/issues/726) Add serialization and deserialization for Wasi State
- [#716](https://github.com/wasmerio/wasmer/issues/716) Improve portability of install script
- [#714](https://github.com/wasmerio/wasmer/issues/714) Add Code of Conduct
- [#708](https://github.com/wasmerio/wasmer/issues/708) Remove unconditional dependency on Cranelift in the C API
- [#703](https://github.com/wasmerio/wasmer/issues/703) Fix compilation on AArch64 Linux
- [#702](https://github.com/wasmerio/wasmer/issues/702) Add SharedMemory to Wasmer. Add `--enable-threads` flag, add partial implementation of atomics to LLVM backend.
- [#698](https://github.com/wasmerio/wasmer/issues/698) [#690](https://github.com/wasmerio/wasmer/issues/690) [#687](https://github.com/wasmerio/wasmer/issues/690) Fix panics in Emscripten
- [#689](https://github.com/wasmerio/wasmer/issues/689) Replace `wasmer_runtime_code::memory::Atomic` with `std::sync::atomic` atomics, changing its interface
- [#680](https://github.com/wasmerio/wasmer/issues/680) [#673](https://github.com/wasmerio/wasmer/issues/673) [#669](https://github.com/wasmerio/wasmer/issues/669) [#660](https://github.com/wasmerio/wasmer/issues/660) [#659](https://github.com/wasmerio/wasmer/issues/659) Misc. runtime and singlepass fixes
- [#677](https://github.com/wasmerio/wasmer/issues/677) [#675](https://github.com/wasmerio/wasmer/issues/675) [#674](https://github.com/wasmerio/wasmer/issues/674) LLVM backend fixes and improvements
- [#671](https://github.com/wasmerio/wasmer/issues/671) Implement fs polling in `wasi::poll_oneoff` for Unix-like platforms
- [#656](https://github.com/wasmerio/wasmer/issues/656) Move CI to Azure Pipelines
- [#650](https://github.com/wasmerio/wasmer/issues/650) Implement `wasi::path_rename`, improve WASI FS public api, and allow open files to exist even when the underlying file is deleted
- [#643](https://github.com/wasmerio/wasmer/issues/643) Implement `wasi::path_symlink` and improve WASI FS public api IO error reporting
- [#608](https://github.com/wasmerio/wasmer/issues/608) Implement wasi syscalls `fd_allocate`, `fd_sync`, `fd_pread`, `path_link`, `path_filestat_set_times`; update WASI fs API in a WIP way; reduce coupling of WASI code to host filesystem; make debug messages from WASI more readable; improve rights-checking when calling syscalls; implement reference counting on inodes; misc bug fixes and improvements
- [#616](https://github.com/wasmerio/wasmer/issues/616) Create the import object separately from instance instantiation in `runtime-c-api`
- [#620](https://github.com/wasmerio/wasmer/issues/620) Replace one `throw()` with `noexcept` in llvm backend
- [#618](https://github.com/wasmerio/wasmer/issues/618) Implement `InternalEvent::Breakpoint` in the llvm backend to allow metering in llvm
- [#615](https://github.com/wasmerio/wasmer/issues/615) Eliminate `FunctionEnvironment` construction in `feed_event()` speeding up to 70% of compilation in clif
- [#609](https://github.com/wasmerio/wasmer/issues/609) Update dependencies
- [#602](https://github.com/wasmerio/wasmer/issues/602) C api extract instance context from instance
- [#590](https://github.com/wasmerio/wasmer/issues/590) Error visibility changes in wasmer-c-api
- [#589](https://github.com/wasmerio/wasmer/issues/589) Make `wasmer_byte_array` fields `public` in wasmer-c-api

## 0.6.0 - 2019-07-31
- [#603](https://github.com/wasmerio/wasmer/pull/603) Update Wapm-cli, bump version numbers
- [#595](https://github.com/wasmerio/wasmer/pull/595) Add unstable public API for interfacing with the WASI file system in plugin-like usecases
- [#598](https://github.com/wasmerio/wasmer/pull/598) LLVM Backend is now supported in Windows
- [#599](https://github.com/wasmerio/wasmer/pull/599) Fix llvm backend failures in fat spec tests and simd_binaryen spec test.
- [#579](https://github.com/wasmerio/wasmer/pull/579) Fix bug in caching with LLVM and Singlepass backends.
  Add `default-backend-singlepass`, `default-backend-llvm`, and `default-backend-cranelift` features to `wasmer-runtime`
  to control the `default_compiler()` function (this is a breaking change).  Add `compiler_for_backend` function in `wasmer-runtime`
- [#561](https://github.com/wasmerio/wasmer/pull/561) Call the `data_finalizer` field on the `Ctx`
- [#576](https://github.com/wasmerio/wasmer/pull/576) fix `Drop` of uninit `Ctx`
- [#542](https://github.com/wasmerio/wasmer/pull/542) Add SIMD support to Wasmer (LLVM backend only)
  - Updates LLVM to version 8.0

## 0.5.7 - 2019-07-23
- [#575](https://github.com/wasmerio/wasmer/pull/575) Prepare for release; update wapm to 0.3.6
- [#555](https://github.com/wasmerio/wasmer/pull/555) WASI filesystem rewrite.  Major improvements
  - adds virtual root showing all preopened directories
  - improved sandboxing and code-reuse
  - symlinks work in a lot more situations
  - many misc. improvements to most syscalls touching the filesystem

## 0.5.6 - 2019-07-16
- [#565](https://github.com/wasmerio/wasmer/pull/565) Update wapm and bump version to 0.5.6
- [#563](https://github.com/wasmerio/wasmer/pull/563) Improve wasi testing infrastructure
  - fixes arg parsing from comments & fixes the mapdir test to have the native code doing the same thing as the WASI code
  - makes wasitests-generate output stdout/stderr by default & adds function to print stdout and stderr for a command if it fails
  - compiles wasm with size optimizations & strips generated wasm with wasm-strip
- [#554](https://github.com/wasmerio/wasmer/pull/554) Finish implementation of `wasi::fd_seek`, fix bug in filestat
- [#550](https://github.com/wasmerio/wasmer/pull/550) Fix singlepass compilation error with `imul` instruction


## 0.5.5 - 2019-07-10
- [#541](https://github.com/wasmerio/wasmer/pull/541) Fix dependency graph by making separate test crates; ABI implementations should not depend on compilers. Add Cranelift fork as git submodule of clif-backend
- [#537](https://github.com/wasmerio/wasmer/pull/537) Add hidden flag (`--cache-key`) to use prehashed key into the compiled wasm cache and change compiler backend-specific caching to use directories
- [#536](https://github.com/wasmerio/wasmer/pull/536) ~Update cache to use compiler backend name in cache key~

## 0.5.4 - 2019-07-06
- [#529](https://github.com/wasmerio/wasmer/pull/529) Updates the Wasm Interface library, which is used by wapm, with bug fixes and error message improvements

## 0.5.3 - 2019-07-03
- [#523](https://github.com/wasmerio/wasmer/pull/523) Update wapm version to fix bug related to signed packages in the global namespace and locally-stored public keys

## 0.5.2 - 2019-07-02
- [#516](https://github.com/wasmerio/wasmer/pull/516) Add workaround for singlepass miscompilation on GetLocal
- [#521](https://github.com/wasmerio/wasmer/pull/521) Update Wapm-cli, bump version numbers
- [#518](https://github.com/wasmerio/wasmer/pull/518) Update Cranelift and WasmParser
- [#514](https://github.com/wasmerio/wasmer/pull/514) [#519](https://github.com/wasmerio/wasmer/pull/519) Improved Emscripten network related calls, added a null check to `WasmPtr`
- [#515](https://github.com/wasmerio/wasmer/pull/515) Improved Emscripten dyncalls
- [#513](https://github.com/wasmerio/wasmer/pull/513) Fix emscripten lseek implementation.
- [#510](https://github.com/wasmerio/wasmer/pull/510) Simplify construction of floating point constants in LLVM backend. Fix LLVM assertion failure due to definition of %ctx.

## 0.5.1 - 2019-06-24
- [#508](https://github.com/wasmerio/wasmer/pull/508) Update wapm version, includes bug fixes

## 0.5.0 - 2019-06-17

- [#471](https://github.com/wasmerio/wasmer/pull/471) Added missing functions to run Python. Improved Emscripten bindings
- [#494](https://github.com/wasmerio/wasmer/pull/494) Remove deprecated type aliases from libc in the runtime C API
- [#493](https://github.com/wasmerio/wasmer/pull/493) `wasmer_module_instantiate` has better error messages in the runtime C API
- [#474](https://github.com/wasmerio/wasmer/pull/474) Set the install name of the dylib to `@rpath`
- [#490](https://github.com/wasmerio/wasmer/pull/490) Add MiddlewareChain and StreamingCompiler to runtime
- [#487](https://github.com/wasmerio/wasmer/pull/487) Fix stack offset check in singlepass backend
- [#450](https://github.com/wasmerio/wasmer/pull/450) Added Metering
- [#481](https://github.com/wasmerio/wasmer/pull/481) Added context trampoline into runtime
- [#484](https://github.com/wasmerio/wasmer/pull/484) Fix bugs in emscripten socket syscalls
- [#476](https://github.com/wasmerio/wasmer/pull/476) Fix bug with wasi::environ_get, fix off by one error in wasi::environ_sizes_get
- [#470](https://github.com/wasmerio/wasmer/pull/470) Add mapdir support to Emscripten, implement getdents for Unix
- [#467](https://github.com/wasmerio/wasmer/pull/467) `wasmer_instantiate` returns better error messages in the runtime C API
- [#463](https://github.com/wasmerio/wasmer/pull/463) Fix bug in WASI path_open allowing one level above preopened dir to be accessed
- [#461](https://github.com/wasmerio/wasmer/pull/461) Prevent passing negative lengths in various places in the runtime C API
- [#459](https://github.com/wasmerio/wasmer/pull/459) Add monotonic and real time clocks for wasi on windows
- [#447](https://github.com/wasmerio/wasmer/pull/447) Add trace macro (`--features trace`) for more verbose debug statements
- [#451](https://github.com/wasmerio/wasmer/pull/451) Add `--mapdir=src:dest` flag to rename host directories in the guest context
- [#457](https://github.com/wasmerio/wasmer/pull/457) Implement file metadata for WASI, fix bugs in WASI clock code for Unix platforms

## 0.4.2 - 2019-05-16

- [#416](https://github.com/wasmerio/wasmer/pull/416) Remote code loading framework
- [#449](https://github.com/wasmerio/wasmer/pull/449) Fix bugs: opening host files in filestat and opening with write permissions unconditionally in path_open
- [#442](https://github.com/wasmerio/wasmer/pull/442) Misc. WASI FS fixes and implement readdir
- [#440](https://github.com/wasmerio/wasmer/pull/440) Fix type mismatch between `wasmer_instance_call` and `wasmer_export_func_*_arity` functions in the runtime C API.
- [#269](https://github.com/wasmerio/wasmer/pull/269) Add better runtime docs
- [#432](https://github.com/wasmerio/wasmer/pull/432) Fix returned value of `wasmer_last_error_message` in the runtime C API
- [#429](https://github.com/wasmerio/wasmer/pull/429) Get wasi::path_filestat_get working for some programs; misc. minor WASI FS improvements
- [#413](https://github.com/wasmerio/wasmer/pull/413) Update LLVM backend to use new parser codegen traits

## 0.4.1 - 2019-05-06

- [#426](https://github.com/wasmerio/wasmer/pull/426) Update wapm-cli submodule, bump version to 0.4.1
- [#422](https://github.com/wasmerio/wasmer/pull/422) Improved Emscripten functions to run optipng and pngquant compiled to wasm
- [#409](https://github.com/wasmerio/wasmer/pull/409) Improved Emscripten functions to run JavascriptCore compiled to wasm
- [#399](https://github.com/wasmerio/wasmer/pull/399) Add example of using a plugin extended from WASI
- [#397](https://github.com/wasmerio/wasmer/pull/397) Fix WASI fs abstraction to work on Windows
- [#390](https://github.com/wasmerio/wasmer/pull/390) Pin released wapm version and add it as a git submodule
- [#408](https://github.com/wasmerio/wasmer/pull/408) Add images to windows installer and update installer to add wapm bin directory to path

## 0.4.0 - 2019-04-23

- [#383](https://github.com/wasmerio/wasmer/pull/383) Hook up wasi exit code to wasmer cli.
- [#382](https://github.com/wasmerio/wasmer/pull/382) Improve error message on `--backend` flag to only suggest currently enabled backends
- [#381](https://github.com/wasmerio/wasmer/pull/381) Allow retrieving propagated user errors.
- [#379](https://github.com/wasmerio/wasmer/pull/379) Fix small return types from imported functions.
- [#371](https://github.com/wasmerio/wasmer/pull/371) Add more Debug impl for WASI types
- [#368](https://github.com/wasmerio/wasmer/pull/368) Fix issue with write buffering
- [#343](https://github.com/wasmerio/wasmer/pull/343) Implement preopened files for WASI and fix aligment issue when accessing WASI memory
- [#367](https://github.com/wasmerio/wasmer/pull/367) Add caching support to the LLVM backend.
- [#366](https://github.com/wasmerio/wasmer/pull/366) Remove `UserTrapper` trait to fix [#365](https://github.com/wasmerio/wasmer/issues/365).
- [#348](https://github.com/wasmerio/wasmer/pull/348) Refactor internal runtime ↔️ backend abstraction.
- [#355](https://github.com/wasmerio/wasmer/pull/355) Misc changes to `Cargo.toml`s for publishing
- [#352](https://github.com/wasmerio/wasmer/pull/352) Bump version numbers to 0.3.0
- [#351](https://github.com/wasmerio/wasmer/pull/351) Add hidden option to specify wasm program name (can be used to improve error messages)
- [#350](https://github.com/wasmerio/wasmer/pull/350) Enforce that CHANGELOG.md is updated through CI.
- [#349](https://github.com/wasmerio/wasmer/pull/349) Add [CHANGELOG.md](https://github.com/wasmerio/wasmer/blob/master/CHANGELOG.md).

## 0.3.0 - 2019-04-12

- [#276](https://github.com/wasmerio/wasmer/pull/276) [#288](https://github.com/wasmerio/wasmer/pull/288) [#344](https://github.com/wasmerio/wasmer/pull/344) Use new singlepass backend (with the `--backend=singlepass` when running Wasmer)
- [#338](https://github.com/wasmerio/wasmer/pull/338) Actually catch traps/panics/etc when using a typed func.
- [#325](https://github.com/wasmerio/wasmer/pull/325) Fixed func_index in debug mode
- [#323](https://github.com/wasmerio/wasmer/pull/323) Add validate subcommand to validate Wasm files
- [#321](https://github.com/wasmerio/wasmer/pull/321) Upgrade to Cranelift 0.3.0
- [#319](https://github.com/wasmerio/wasmer/pull/319) Add Export and GlobalDescriptor to Runtime API
- [#310](https://github.com/wasmerio/wasmer/pull/310) Cleanup warnings
- [#299](https://github.com/wasmerio/wasmer/pull/299) [#300](https://github.com/wasmerio/wasmer/pull/300) [#301](https://github.com/wasmerio/wasmer/pull/301) [#303](https://github.com/wasmerio/wasmer/pull/303) [#304](https://github.com/wasmerio/wasmer/pull/304) [#305](https://github.com/wasmerio/wasmer/pull/305) [#306](https://github.com/wasmerio/wasmer/pull/306) [#307](https://github.com/wasmerio/wasmer/pull/307) Add support for WASI 🎉
- [#286](https://github.com/wasmerio/wasmer/pull/286) Add extend to imports
- [#278](https://github.com/wasmerio/wasmer/pull/278) Add versioning to cache
- [#250](https://github.com/wasmerio/wasmer/pull/250) Setup bors