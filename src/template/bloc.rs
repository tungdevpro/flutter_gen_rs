pub fn bloc_content(name: &str, class_name: &str) -> String {
    format!(
        r#"
import 'package:flutter_bloc/flutter_bloc.dart';
import '{name}_event.dart';
import '{name}_state.dart';

class {class_name}Bloc extends Bloc<{class_name}Event, {class_name}State> {{
    const {class_name}Bloc : super({class_name}State());
}}
    "#,
        name = name,
        class_name = class_name,
    )
}
