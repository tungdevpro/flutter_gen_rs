pub fn event_content(class_name: &str) -> String {
    format!(
        r#"
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:equatable/equatable.dart';

sealed class {class_name}Event extends Equatable {{
  @override
  List<Object?> get props => []
}}

final class {class_name}DataRequestEvent extends {class_name}Event {{
  @override
  List<Object?> get props => []
}}

    "#,
        class_name = class_name,
    )
}
