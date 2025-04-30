fn main() {
    dotenv::dotenv().ok();
    let profile = std::env::var("PROFILE").unwrap();
    let icon_name = "logo-32x32.png";

    let mut out_dir = match profile.as_str() {
        "debug" => std::env::var("DEBUG_BUILD_DIR").expect("Undefined env variable. Status"),
        "release" => std::env::var("LINUX_RELEASE_BUILD_DIR").expect("Undefined env variable. Status"),
        prof => panic!("Etc profile: {prof}"),
    };
    out_dir.push_str("/assets");

    if !std::fs::exists(out_dir.as_str()).unwrap() {
        let _ = std::fs::create_dir(out_dir.as_str());
    }


    std::fs::copy(
        format!("assets/{}", icon_name),
        format!("{}/{}", out_dir, icon_name),
    ).unwrap();
}

