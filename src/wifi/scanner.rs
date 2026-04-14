use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct NetworkInfo {
    pub ssid: String,
    pub bssid: String,
    pub signal_strength: i32,
    pub channel: u8,
    pub frequency: String,
    pub security: String,
    pub vendor: Option<String>,
}

pub struct WifiScanner {
    networks: Vec<NetworkInfo>,
    last_scan: Option<std::time::Instant>,
}

impl WifiScanner {
    pub fn new() -> Self {
        Self {
            networks: Vec::new(),
            last_scan: None,
        }
    }
    
    pub fn start_scan(&mut self) {
        self.networks.clear();
        
        // محاكاة الفحص - في الإصدار الحقيقي استخدم WlanScan
        let mock_networks = vec![
            NetworkInfo {
                ssid: "Home_Network_5G".to_string(),
                bssid: "AA:BB:CC:DD:EE:FF".to_string(),
                signal_strength: -45,
                channel: 44,
                frequency: "5 GHz".to_string(),
                security: "WPA3-SAE".to_string(),
                vendor: Some("Intel".to_string()),
            },
            NetworkInfo {
                ssid: "Guest_WiFi".to_string(),
                bssid: "11:22:33:44:55:66".to_string(),
                signal_strength: -62,
                channel: 6,
                frequency: "2.4 GHz".to_string(),
                security: "WPA2-PSK".to_string(),
                vendor: Some("TP-Link".to_string()),
            },
            NetworkInfo {
                ssid: "Neighbor_Network".to_string(),
                bssid: "22:33:44:55:66:77".to_string(),
                signal_strength: -78,
                channel: 11,
                frequency: "2.4 GHz".to_string(),
                security: "WPA2-PSK".to_string(),
                vendor: None,
            },
        ];
        
        self.networks = mock_networks;
        self.last_scan = Some(std::time::Instant::now());
    }
    
    pub fn networks(&self) -> &[NetworkInfo] {
        &self.networks
    }
    
    pub fn clear(&mut self) {
        self.networks.clear();
        self.last_scan = None;
    }
    
    pub fn analyze_channels(&self) -> u8 {
        if self.networks.is_empty() {
            return 6; // القناة الافتراضية
        }
        
        let mut channel_count: HashMap<u8, usize> = HashMap::new();
        
        for network in &self.networks {
            *channel_count.entry(network.channel).or_insert(0) += 1;
        }
        
        // البحث عن أقل قناة ازدحامًا (2.4 GHz)
        let channels_24ghz = [1, 6, 11];
        let mut best_channel = 6;
        let mut min_count = usize::MAX;
        
        for &ch in &channels_24ghz {
            let count = channel_count.get(&ch).copied().unwrap_or(0);
            if count < min_count {
                min_count = count;
                best_channel = ch;
            }
        }
        
        best_channel
    }
}

impl Default for WifiScanner {
    fn default() -> Self {
        Self::new()
    }
}
