import 'package:crm_client/native.dart';
import 'package:crm_client/rust/bridge_generated.dart';
import 'package:flutter/material.dart';

void main() {
  runApp(const MainApp());
}

class MainApp extends StatelessWidget {
  const MainApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        body: Center(
          child: FutureBuilder(
            future: api.login(
              accessData: AccessData(
                login: "login",
                password: "password",
              ),
            ),
            builder: (context, data) {
              if (data.hasData) {
                return Text(data.data ?? "ERROR");
              }
              return const Center(
                child: CircularProgressIndicator(),
              );
            },
          ),
        ),
      ),
    );
  }
}
