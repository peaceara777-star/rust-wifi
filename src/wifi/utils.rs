use std::process::Command;

pub fn get_wifi_interface() -> Option<String> {
    #[cfg(target_os = "windows")]
    {
        Some("Wi-Fi".to_string())
    }
    
    #[cfg(target_os = "linux")]
    {
        let output = Command::new("iwconfig")
            .output()
            .ok()?;
        
        let output_str = String::from_utf8_lossy(&output.stdout);
        for line in output_str.lines() {
            if line.contains("IEEE 802.11") {
                return Some(line.split_whitespace().next()?.to_string());
            }
        }
        None
    }
    
    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
        None
    }
}

pub fn get_current_ssid() -> Option<String> {
    #[cfg(target_os = "windows")]
    {
        let output = Command::new("netsh")
            .args(["wlan", "show", "interfaces"])
            .output()
            .ok()?;
        
        let output_str = String::from_utf8_lossy(&output.stdout);
        for line in output_str.lines() {
            if line.contains("SSID") && !line.contains("BSSID") {
                return Some(line.split(':').nth(1)?.trim().to_string());
            }
        }
        None
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        None
    }
}

pub fn restart_wifi_adapter() -> bool {
    #[cfg(target_os = "windows")]
    {
        // تعطيل
        Command::new("netsh")
            .args(["interface", "set", "interface", "Wi-Fi", "admin=disable"])
            .output()
            .ok()?;
        
        std::thread::sleep(std::time::Duration::from_secs(2));
        
        // تمكين
        Command::new("netsh")
            .args(["interface", "set", "interface", "Wi-Fi", "admin=enable"])
            .output()
            .ok()?;
        
        true
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        false
    }
}

pub fn signal_to_percentage(signal_dbm: i32) -> u8 {
    // تحويل قوة الإشارة من dBm إلى نسبة مئوية
    // النطاق المعتاد: -30 dBm (ممتاز) إلى -90 dBm (ضعيف)
    if signal_dbm >= -30 {
        100
    } else if signal_dbm <= -90 {
        0
    } else {
        ((signal_dbm + 90) * 100 / 60) as u8
    }
}

pub fn channel_to_frequency(channel: u8) -> String {
    match channel {
        1..=14 => format!("2.4 GHz (Channel {})", channel),
        36..=165 => format!("5 GHz (Channel {})", channel),
        _ => "Unknown".to_string(),
    }
}
