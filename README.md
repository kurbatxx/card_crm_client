# crm_client

```
cargo install flutter_rust_bridge_codegen
cargo add flutter_rust_bridge
```

# flutter

```
flutter pub add ffi -d ffigen flutter_rust_bridge -d build_runner -d freezed freezed_annotation
```

# for non arch add variable
```
export CPATH="$(clang -v 2>&1 | grep "Selected GCC installation" | rev | cut -d' ' -f1 | rev)/include"
```

# buiid

```
flutter_rust_bridge_codegen --rust-input src/api.rs --dart-output crm_client/lib/rust/bridge_generated.dart --llvm-path "C:\DEV\LLVM"
```