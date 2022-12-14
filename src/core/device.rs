use crate::Split;

pub struct Device {
    pub vendor_id: String,
    pub product_id: String,
    pub description: String,
}

impl Clone for Device {
    fn clone(&self) -> Self {
        Device {
            vendor_id: self.vendor_id.clone(),
            product_id: self.product_id.clone(),
            description: self.description.clone(),
        }
    }
}

impl Device {
    pub fn from(usb_device: &String) -> Device {
        let parts = usb_device.splitn_to_vec(7, ' ');
        let ids = parts.get(5)
            .unwrap()
            .split_to_vec(':');
        let vendor_id = ids.get(0)
            .unwrap()
            .clone();
        let product_id = ids.get(1)
            .unwrap()
            .clone();
        let description = parts.get(6)
            .unwrap()
            .clone();
        Device {
            vendor_id,
            product_id,
            description,
        }
    }
}