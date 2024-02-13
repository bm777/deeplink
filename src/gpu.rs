use std::process::Command;

pub fn get_gpu_info() -> String {
    // #[cfg(target_os = "windows")]
    // let output = Command::new("cmd")
    //                     .args(["/C", "wmic path win32_VideoController get name"])
    //                     .output()
    //                     .expect("Failed to execute command");

    // #[cfg(target_os = "linux")]
    // let output = Command::new("sh")
    //                     .arg("-c")
    //                     .arg("lspci | grep VGA")
    //                     .output()
    //                     .expect("Failed to execute command");

    #[cfg(target_os = "macos")]
    let output = Command::new("sh")
                        .arg("-c")
                        .arg("system_profiler SPDisplaysDataType | grep 'Chipset Model'")
                        .output()
                        .expect("Failed to execute command");

    String::from_utf8_lossy(&output.stdout).trim().to_string()
}
