#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn compare_versions(version1: String, version2: String) -> i32 {
    // 将版本号分割为修订号数组
    let v1_parts: Vec<&str> = version1.split('.').collect();
    let v2_parts: Vec<&str> = version2.split('.').collect();
    
    // 获取最大分段长度
    let max_len = v1_parts.len().max(v2_parts.len());
    
    for i in 0..max_len {
        // 转换分段为整数（自动去除前导零）
        let num1 = v1_parts.get(i).and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);
        let num2 = v2_parts.get(i).and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);
        
        match num1.cmp(&num2) {
            std::cmp::Ordering::Greater => return 1,
            std::cmp::Ordering::Less => return -1,
            _ => continue,
        }
    }
    0
}

