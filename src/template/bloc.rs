
pub fn bloc_content(name: &str) -> String {
    format!(r#"
        import 'package:flutter_bloc/flutter_bloc.dart';
        import '{name}_event.dart';
        import '{name}_state.dart';


    "#, name = name)
}
