struct Anuality {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub interest_rate: f64,
    pub terms_year: i32,
    pub term_years: i32,
    pub pay_frequency: i32,
    pub payment_amount: f64,
    pub annual_payment: f64,
}

impl Anuality {
    pub fn new(id: i32, name: String, description: String, interest_rate: f64, 
        term_years: i32, pay_frequency: i32, payment_amount: f64, annual_payment: f64) -> Self {
        Anuality {
            id,
            name,
            description,
            interest_rate,
            terms_year,
            term_years,
            pay_frequency,
            payment_amount,
            annual_payment
        }
    }

    pub fn calc_money_value_compound_interest(&self, ir: f64, term_years: i32) -> f64 {
        let mut sum = 0.0;
        for i in 0..(self.term_years - 1) {
            sum = sum + self.annual_payment * (1.0 + ir / 100.0).powi(i);
        }
        sum
    }
}