use serde::Serialize;
use sysinfo::{DiskExt, System, SystemExt};
use crate::metrics::MetricCollector; // मान लो 'metrics' crate में 'MetricCollector' है

// 1. डेटा स्ट्रक्चर: हम जो डेटा कलेक्ट करेंगे, वह ऐसा दिखेगा
#[derive(Debug, Serialize)]
pub struct DiskMetric {
    pub name: String,
    pub total_space_gb: f64,
    pub used_space_gb: f64,
    pub free_space_gb: f64,
    pub file_system: String,
}

// 2. MetricCollector को इम्प्लीमेंट करना
impl MetricCollector for DiskMetric {
    // यह फ़ंक्शन sysinfo का इस्तेमाल करके डिस्क की जानकारी कलेक्ट करेगा
    fn collect() -> Result<Vec<Self>, String> {
        let mut sys = System::new();
        // Disks की लिस्ट को रिफ्रेश करते हैं
        sys.refresh_disks_list();
        
        let mut metrics = Vec::new();
        
        // हर डिस्क के लिए लूप चलाओ
        for disk in sys.disks() {
            // बाइट्स को GB में कन्वर्ट करने के लिए
            const BYTE_TO_GB: f64 = 1024.0 * 1024.0 * 1024.0;
            
            let total_bytes = disk.total_space();
            let available_bytes = disk.available_space();
            let used_bytes = total_bytes - available_bytes;
            
            // डेटा को स्ट्रक्चर में भर दो
            let metric = DiskMetric {
                name: disk.name().to_string_lossy().into_owned(),
                total_space_gb: total_bytes as f64 / BYTE_TO_GB,
                used_space_gb: used_bytes as f64 / BYTE_TO_GB,
                free_space_gb: available_bytes as f64 / BYTE_TO_GB,
                file_system: String::from_utf8_lossy(disk.file_system()).into_owned(),
            };
            metrics.push(metric);
        }

        if metrics.is_empty() {
            Err("Koi disk metrics collect nahi hue.".to_string())
        } else {
            Ok(metrics)
        }
    }
}