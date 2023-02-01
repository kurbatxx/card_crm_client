import 'dart:ffi';
import 'dart:io' as io;
import 'package:crm_client/rust/bridge_generated.dart';

const _base = 'card_crm_client';

final _dylib = io.Platform.isWindows ? '$_base.dll' : 'lib$_base.so';

final api = CardCrmClientImpl(io.Platform.isIOS || io.Platform.isMacOS
    ? DynamicLibrary.executable()
    : DynamicLibrary.open(_dylib));
