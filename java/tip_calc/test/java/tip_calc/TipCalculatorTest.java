package tip_calc;

import org.testng.Assert;
import org.testng.annotations.DataProvider;
import org.testng.annotations.Test;

public class TipCalculatorTest {

    @DataProvider(name = "data-provider")
    public Object[][] dpMethod(){
        return new Object[][] {
                {100.0,15.0,15.0,115.0},
                {11.25,15.0,1.69,12.94}
        };
    }

    @Test (dataProvider = "data-provider")
    public void testCalulatePaymentDetails(double bill, double tipPercentage, double tip, double totalAmount) {
        var tipCalculator = new TipCalculator();
        var paymentDetails = tipCalculator.calulatePaymentDetails(bill, tipPercentage);

        Assert.assertEquals((double) paymentDetails.tip(), tip);
        Assert.assertEquals((double) paymentDetails.totalAmount(), totalAmount);
    }
}