use std::process::Command;

#[derive(Debug, Clone)]
pub struct TestResult {
    pub passed: bool,
    pub message: String,
    pub latency: Option<u64>,
}

pub struct DiagnosticReport {
    pub tests: Vec<(String, TestResult)>,
    pub timestamp: chrono::DateTime<chrono::Local>,
}

pub struct SystemInfo {
    pub os: String,
    pub adapter: String,
    pub mac: String,
    pub link_speed: String,
}

pub struct DiagnosticsEngine {
    last_report: Option<DiagnosticReport>,
}

impl DiagnosticsEngine {
    pub fn new() -> Self {
        Self { last_report: None }
    }
    
    pub fn test_ping(&mut self, target: &str) -> TestResult {
        let output = Command::new("ping")
            .arg(if cfg!(windows) { "-n" } else { "-c" })
            .arg("4")
            .arg(target)
            .output();
        
        match output {
            Ok(output) => {
                let passed = output.status.success();
                let message = if passed {
                    format!("✅ الاتصال بـ {} ناجح", target)
                } else {
                    format!("❌ فشل الاتصال بـ {}", target)
                };
                
                TestResult {
                    passed,
                    message,
                    latency: None,
                }
            }
            Err(e) => TestResult {
                passed: false,
                message: format!("خطأ في تنفيذ ping: {}", e),
                latency: None,
            },
        }
    }
    
    pub fn run_full_diagnostics(&mut self) -> DiagnosticReport {
        let mut tests = Vec::new();
        
        // اختبار البوابة
        tests.push(("Ping Gateway".to_string(), self.test_ping("192.168.1.1")));
        
        // اختبار DNS
        tests.push(("Ping Google DNS".to_string(), self.test_ping("8.8.8.8")));
        tests.push(("Ping Cloudflare DNS".to_string(), self.test_ping("1.1.1.1")));
        
        // اختبار الإنترنت
        tests.push(("Internet Connectivity".to_string(), self.test_ping("google.com")));
        
        let report = DiagnosticReport {
            tests,
            timestamp: chrono::Local::now(),
        };
        
        self.last_report = Some(report.clone());
        report
    }
    
    pub fn get_system_info(&self) -> SystemInfo {
        SystemInfo {
            os: std::env::consts::OS.to_string(),
            adapter: "Wi-Fi".to_string(),
            mac: "AA:BB:CC:DD:EE:FF".to_string(),
            link_speed: "866.7 Mbps".to_string(),
        }
    }
    
    pub fn last_report(&self) -> Option<&DiagnosticReport> {
        self.last_report.as_ref()
    }
}

impl Default for DiagnosticsEngine {
    fn default() -> Self {
        Self::new()
    }
}
