use crate::traits::pricing_trait::PricingTrait;

pub struct CommodityPrototype {
    name: String,
    description: String,
    quantity: u32,
    mutual_date: String,
    quantity_unit: String,
    target_price: f64,
}

impl CommodityPrototype {
    pub fn new(name: String, description: String, quantity: u32, mutual_date: String
    , quantity_unit: String, target_price: f64) -> Self {
        CommodityPrototype {
            name,
            description,
            quantity,
            mutual_date,
            quantity_unit,
            target_price,
        }
    }
}

impl PricingTrait for CommodityPrototype {
    fn calculate_price(&self, current_price: f64) -> f64 {
        current_price * self.quantity as f64
    }
}