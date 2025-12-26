fn main() {
    // enable thunk-rs when target os is windows and arch is x86_64 or i686
    #[cfg(target_os = "windows")]
    if !std::env::var("TARGET")
        .unwrap_or_default()
        .contains("aarch64")
    {
        // Ensure 7z is available for thunk-rs (used to unpack downloaded libraries).
        // Do not override user configuration if it's already present.
        let seven_zip_dir = r"C:\Program Files\7-Zip";
        let path = std::env::var("PATH").unwrap_or_default();
        let mut has_7z = false;
        for p in std::env::split_paths(&path) {
            if p.to_string_lossy().eq_ignore_ascii_case(seven_zip_dir) {
                has_7z = true;
                break;
            }
        }
        if !has_7z {
            let mut new_path = path;
            if !new_path.is_empty() {
                new_path.push(';');
            }
            new_path.push_str(seven_zip_dir);
            std::env::set_var("PATH", new_path);
        }

        // If direct GitHub access is not available, allow using a proxy.
        // Respect user provided values if already set.
        let proxy_prefix = std::env::var("EASYTIER_GITHUB_PROXY")
            .unwrap_or_else(|_| "https://ghproxy.net/".to_string());
        if std::env::var_os("VC_LTL_URL").is_none() {
            std::env::set_var(
                "VC_LTL_URL",
                format!(
                    "{}https://github.com/Chuyu-Team/VC-LTL5/releases/download/v5.2.2/VC-LTL-Binary.7z",
                    proxy_prefix
                ),
            );
        }
        if std::env::var_os("YY_THUNKS_URL").is_none() {
            std::env::set_var(
                "YY_THUNKS_URL",
                format!(
                    "{}https://github.com/Chuyu-Team/YY-Thunks/releases/download/v1.1.7/YY-Thunks-Objs.zip",
                    proxy_prefix
                ),
            );
        }

        thunk::thunk();
    }
}
