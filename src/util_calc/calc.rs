pub fn power_of(base : f64, power : i32) -> f64{
    if power == 0 {
        1.0
    } else {
        let mut pow : i32 = power;
        let mut result : f64 = base;
        while pow > 1 {
            result = result * base;
            pow = pow - 1;
        }
        result
    }
}

pub fn power_calc_err1(base : f64, power : i32) -> Result<f64, String>{
    if power < 0 {
        Err("Negative power is not supported".to_string())
    } else {
        Ok(power_of(base, power))
    }
}

pub fn power_calc_err2(base : f64, power : i32) -> Result<f64, String>{
    let t = power_calc_err1(base, power)?;
    Ok(t)
}

pub fn power_of_v2(base : f64, power : i32) -> f64 {
    let mut result : f64 = base;

    if power == 0 {
        1.0
    } else if power %2 == 0 {
        result = power_of_v2(base, power / 2);
        result * result
    } else {
        base * power_of_v2(base, power - 1)
    }
}

pub fn optimize(power : f64, exp_num : i32) -> f64 {
    power.powi(exp_num)
}
