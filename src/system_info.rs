use sys_info;
use crate::models::SystemInfo;
use crate::gpu::get_gpu_info;

pub fn get_system_info() -> SystemInfo {
    let gpu = get_gpu_info();
    let ram = sys_info::mem_info().unwrap().total;
    let internet_speed = "5Mbps".to_string(); // Measuring internet speed requires a different approach
    let token = "123456".to_string();
    let unique_id = "123456".to_string();
    let input = 0;
    let output = 0;

    SystemInfo {
        gpu,
        ram,
        internet_speed,
        token,
        unique_id,
        input,
        output
    }
}
