package tip_calc;

public class TipCalculator {
    public PaymentDetails calulatePaymentDetails(double bill, double tipPercentage) {
        var tip = Math.round(tipPercentage*bill) / 100.0;
        return new PaymentDetails(tip, tip+bill);
    }
}
