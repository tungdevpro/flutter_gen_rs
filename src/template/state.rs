pub fn state_content(class_name: &str) -> String {
    format!(
        r#"
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:equatable/equatable.dart';

final class {class_name}State extends Equatable {{
    {class_name}State();

     {class_name}State copyWith({{}}) {{
        return {class_name}State();
     }}

  @override
  List<Object?> get props => []
}}
    "#,
        class_name = class_name,
    )
}
