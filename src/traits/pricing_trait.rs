pub trait PricingTrait {
    fn calculate_price(&self, current_price: f64) -> f64;
}