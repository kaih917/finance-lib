struct BasicTimeValue{
    base_value: f64,
    time_month: i32,
    yearly_ir: f64,
}

impl BasicTimeValue {
    pub fn new(base_value: f64, time: i32, yearly_ir: f64) -> Self {
        BasicTimeValue {
            base_value,
            time,
            yearly_ir
        }
    }

    pub fn calc_money_value_compound_interest(&self) -> f64 {
        self.base_value * (1.0 + ((self.yearly_ir/100.0)/12)).powi(self.time_month)
    }

    pub fn calc_money_value_simple_interest(&self) -> f64 {
        self.base_value * (1.0 + (self.yearly_ir/100.0) * (self.time_month as f64 / 12.0))
    }
}