
use sysinfo::{System, SystemExt, DiskExt};

use super::{Block, Status};

pub struct StorageBlock;

impl StorageBlock {
    fn status(&self, cap: u64) -> Status {
        match cap {
            0..=95 => Status::Normal,
            96..=98 => Status::Warning,
            _ => Status::Alarm,
        }

    }
}

impl Block for StorageBlock {
    fn make(&self, s: &mut System) -> (&str, String, Status) {
        s.refresh_disks();
        let avail_total = s.get_disks().iter()
            .filter(|x| x.get_mount_point().to_str() == Some("/"))
            .map(|x| (x.get_available_space(), x.get_total_space()))
            .nth(0);
        let (avail, cap) = match avail_total {
            Some((avail, total)) => {
                let avail_gb = avail as f64 /1024.0 /1024.0 /1024.0;
                let cap = avail / total * 100;
                (avail_gb, cap)
            },
            _ => (0.0, 1),
        };
        let status = self.status(cap);
        let symb = '';
        let text = format!("{} {:.1}Gb", symb, avail);

        ("storage", text, status)
    }
}
