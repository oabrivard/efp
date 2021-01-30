#[derive(PartialEq, Debug)]
pub struct PaymentDetails {
    pub tip: f64,
    pub total_amount: f64
}

pub fn calculate_payment_details(bill: f64, tip_percentage: f64) -> PaymentDetails {
    let tip = (tip_percentage * bill).round() / 100.0;
    let total_amount = bill + tip;

    PaymentDetails {tip, total_amount}
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_calculate_payment_details() {
        let result = calculate_payment_details(100.0,15.0);
        assert_eq!(result, PaymentDetails {tip: 15.0, total_amount: 115.0});

        let result = calculate_payment_details(11.25,15.0);
        assert_eq!(result, PaymentDetails {tip: 1.69, total_amount: 12.94});
    }
}