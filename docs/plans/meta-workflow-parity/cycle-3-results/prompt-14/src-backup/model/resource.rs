use std::sync::Mutex;

/// Represents an allocated resource entry.
#[derive(Debug, Clone)]
pub struct ResourceAllocation {
    pub model_name: String,
    pub ram_mb: u64,
    pub vram_mb: u64,
}

/// Manages system resource allocation for model instances.
///
/// Thread-safe via internal Mutex.
pub struct ResourceManager {
    total_ram_mb: u64,
    total_vram_mb: u64,
    allocated_ram_mb: Mutex<u64>,
    allocated_vram_mb: Mutex<u64>,
}

impl ResourceManager {
    pub fn new(total_ram_mb: u64, total_vram_mb: u64) -> Self {
        tracing::debug!(
            total_ram_mb = total_ram_mb,
            total_vram_mb = total_vram_mb,
            "ResourceManager initialized"
        );

        Self {
            total_ram_mb,
            total_vram_mb,
            allocated_ram_mb: Mutex::new(0),
            allocated_vram_mb: Mutex::new(0),
        }
    }

    pub fn can_allocate(&self, ram_mb: u64, vram_mb: u64) -> bool {
        let allocated_ram = *self.allocated_ram_mb.lock().unwrap();
        let allocated_vram = *self.allocated_vram_mb.lock().unwrap();

        let ram_available = allocated_ram + ram_mb <= self.total_ram_mb;
        let vram_available = allocated_vram + vram_mb <= self.total_vram_mb;

        tracing::debug!(
            requested_ram_mb = ram_mb,
            requested_vram_mb = vram_mb,
            allocated_ram_mb = allocated_ram,
            allocated_vram_mb = allocated_vram,
            total_ram_mb = self.total_ram_mb,
            total_vram_mb = self.total_vram_mb,
            ram_available = ram_available,
            vram_available = vram_available,
            "Checking resource availability"
        );

        ram_available && vram_available
    }

    pub fn allocate(&mut self, name: &str, ram_mb: u64, vram_mb: u64) -> Result<(), String> {
        if !self.can_allocate(ram_mb, vram_mb) {
            return Err(format!(
                "Insufficient resources: requested RAM={}MB VRAM={}MB, available RAM={}MB VRAM={}MB",
                ram_mb,
                vram_mb,
                self.available_ram(),
                self.available_vram()
            ));
        }

        let mut allocated_ram = self.allocated_ram_mb.lock().unwrap();
        let mut allocated_vram = self.allocated_vram_mb.lock().unwrap();

        *allocated_ram += ram_mb;
        *allocated_vram += vram_mb;

        tracing::debug!(
            model = %name,
            allocated_ram_mb = ram_mb,
            allocated_vram_mb = vram_mb,
            total_allocated_ram_mb = *allocated_ram,
            total_allocated_vram_mb = *allocated_vram,
            "Resources allocated"
        );

        Ok(())
    }

    pub fn deallocate(&mut self, name: &str, ram_mb: u64, vram_mb: u64) {
        let mut allocated_ram = self.allocated_ram_mb.lock().unwrap();
        let mut allocated_vram = self.allocated_vram_mb.lock().unwrap();

        let new_ram = allocated_ram.saturating_sub(ram_mb);
        let new_vram = allocated_vram.saturating_sub(vram_mb);

        let ram_diff = *allocated_ram - new_ram;
        let vram_diff = *allocated_vram - new_vram;

        *allocated_ram = new_ram;
        *allocated_vram = new_vram;

        if ram_diff > 0 || vram_diff > 0 {
            tracing::debug!(
                model = %name,
                deallocated_ram_mb = ram_diff,
                deallocated_vram_mb = vram_diff,
                total_allocated_ram_mb = new_ram,
                total_allocated_vram_mb = new_vram,
                "Resources deallocated"
            );
        }
    }

    pub fn available_ram(&self) -> u64 {
        self.total_ram_mb - *self.allocated_ram_mb.lock().unwrap()
    }

    pub fn available_vram(&self) -> u64 {
        self.total_vram_mb - *self.allocated_vram_mb.lock().unwrap()
    }

    pub fn allocated_ram(&self) -> u64 {
        *self.allocated_ram_mb.lock().unwrap()
    }

    pub fn allocated_vram(&self) -> u64 {
        *self.allocated_vram_mb.lock().unwrap()
    }

    pub fn utilization_percent(&self) -> (f64, f64) {
        let ram_util = (*self.allocated_ram_mb.lock().unwrap() as f64 / self.total_ram_mb as f64) * 100.0;
        let vram_util = (*self.allocated_vram_mb.lock().unwrap() as f64 / self.total_vram_mb as f64) * 100.0;
        (ram_util, vram_util)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resource_manager_initializes_with_zero_allocations() {
        let manager = ResourceManager::new(16000, 24000);

        assert_eq!(manager.available_ram(), 16000);
        assert_eq!(manager.available_vram(), 24000);
        assert_eq!(manager.allocated_ram(), 0);
        assert_eq!(manager.allocated_vram(), 0);
    }

    #[test]
    fn can_allocate_returns_true_for_sufficient_resources() {
        let manager = ResourceManager::new(16000, 24000);

        assert!(manager.can_allocate(8000, 12000));
    }

    #[test]
    fn can_allocate_returns_false_for_insufficient_ram() {
        let manager = ResourceManager::new(16000, 24000);

        assert!(!manager.can_allocate(17000, 1000));
    }

    #[test]
    fn can_allocate_returns_false_for_insufficient_vram() {
        let manager = ResourceManager::new(16000, 24000);

        assert!(!manager.can_allocate(1000, 25000));
    }

    #[test]
    fn allocate_succeeds_for_sufficient_resources() {
        let mut manager = ResourceManager::new(16000, 24000);

        let result = manager.allocate("test-model", 8000, 12000);
        assert!(result.is_ok());

        assert_eq!(manager.allocated_ram(), 8000);
        assert_eq!(manager.allocated_vram(), 12000);
        assert_eq!(manager.available_ram(), 8000);
        assert_eq!(manager.available_vram(), 12000);
    }

    #[test]
    fn allocate_fails_for_insufficient_resources() {
        let mut manager = ResourceManager::new(16000, 24000);

        let result = manager.allocate("test-model", 17000, 1000);
        assert!(result.is_err());
    }

    #[test]
    fn deallocate_reduces_allocations() {
        let mut manager = ResourceManager::new(16000, 24000);

        manager.allocate("test-model", 8000, 12000).unwrap();
        manager.deallocate("test-model", 4000, 6000);

        assert_eq!(manager.allocated_ram(), 4000);
        assert_eq!(manager.allocated_vram(), 6000);
        assert_eq!(manager.available_ram(), 12000);
        assert_eq!(manager.available_vram(), 18000);
    }

    #[test]
    fn deallocate_saturates_at_zero() {
        let mut manager = ResourceManager::new(16000, 24000);

        manager.allocate("test-model", 8000, 12000).unwrap();
        manager.deallocate("test-model", 10000, 15000);

        assert_eq!(manager.allocated_ram(), 0);
        assert_eq!(manager.allocated_vram(), 0);
    }

    #[test]
    fn utilization_percent_calculates_correctly() {
        let mut manager = ResourceManager::new(16000, 24000);

        manager.allocate("test-model", 4000, 6000).unwrap();

        let (ram_util, vram_util) = manager.utilization_percent();
        assert!((ram_util - 25.0).abs() < 0.01);
        assert!((vram_util - 25.0).abs() < 0.01);
    }
}
