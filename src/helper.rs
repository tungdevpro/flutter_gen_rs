pub fn is_flutter_project() -> bool {
    let current_dir = std::env::current_dir().unwrap();
    let pubspec_path = current_dir.join("pubspec.yaml");
    let android_dir = current_dir.join("android");
    let ios_dir = current_dir.join("ios");

    pubspec_path.exists() && android_dir.exists() && ios_dir.exists()
}
