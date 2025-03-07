pub fn is_flutter_project() -> bool {
    let current_dir = std::env::current_dir().unwrap();
    let pubspec_path = current_dir.join("pubspec.yaml");
    let android_dir = current_dir.join("android");
    let ios_dir = current_dir.join("ios");

    pubspec_path.exists() && android_dir.exists() && ios_dir.exists()
}

pub fn convert_name_to_upper_case(name: &str) -> String {
    let mut result = String::new();
    let mut is_capitalize = true;

    for c in name.chars() {
        if is_capitalize &&  c.is_alphabetic() {
            result.push(c.to_uppercase().next().unwrap());
            is_capitalize= false
        } else {
            result.push(c);
            is_capitalize = c == '_';
        }
    }

    result
}