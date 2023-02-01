# crm_client

```
cargo install flutter_rust_bridge_codegen
cargo add flutter_rust_bridge
```

# flutter

```
flutter pub add ffi
flutter pub add --dev ffigen
flutter pub add flutter_rust_bridge
```

# buiid

```
flutter_rust_bridge_codegen --rust-input src/api.rs --dart-output crm_client/lib/rust/bridge_generated.dart --llvm-path "C:\DEV\LLVM"
```