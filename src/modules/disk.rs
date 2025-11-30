use sysinfo::{DiskExt, System, SystemExt};
use std::collections::HashMap;
use crate::core::{MetricCollector, MetricData, MetricValue};

pub struct DiskCollector;

impl DiskCollector {
    pub fn new() -> Self {
        DiskCollector
    }
}

impl MetricCollector for DiskCollector {
    fn collect(&self) -> Result<MetricData, Box<dyn std::error::Error>> {
        let mut sys = System::new_all();
        sys.refresh_disks_list();

        let mut total: u64 = 0;
        let mut used: u64 = 0;
        let mut free: u64 = 0;

        for disk in sys.disks() {
            total += disk.total_space();
            free += disk.available_space();
        }

        used = total - free;

        let mut map = HashMap::new();
        map.insert("total".to_string(), MetricValue::Integer(total as i64));
        map.insert("used".to_string(), MetricValue::Integer(used as i64));
        map.insert("free".to_string(), MetricValue::Integer(free as i64));

        Ok(MetricData {
            timestamp: std::time::SystemTime::now(),
            metrics: map,
        })
    }

    fn name(&self) -> &'static str {
        "disk"
    }
}
