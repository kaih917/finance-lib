pub mod commodities;
pub mod traits;

#[cfg(test)]
mod tests {
    use crate::commodities::CommodityPrototype;
    use crate::traits::pricing_trait::PricingTrait;
    

    #[test]
    fn it_works() {
        let _result = CommodityPrototype::new(
            "Gold".to_string(),
            "A precious metal".to_string(),
            100,
            "2024-12-31".to_string(),
            "grams".to_string(),
            14.35,
        );
        // println!("MTM value: {}", _result.calculate_price());

        let mut price_scenario: Vec<(String,f64)> = Vec::new();
        price_scenario.push(("20260201".to_string(),20.4));
        price_scenario.push(("20260301".to_string(),21.2));
        price_scenario.push(("20260401".to_string(),30.12));
        price_scenario.push(("20260501".to_string(),28.6));

        for (date, price) in price_scenario {
            println!("MTM value for {}: {}", date, _result.calculate_price(price));
        }
    }
}
