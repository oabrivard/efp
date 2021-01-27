package tip_calc;

import javafx.event.ActionEvent;
import javafx.fxml.FXML;
import javafx.scene.control.Label;
import javafx.scene.control.Slider;
import javafx.scene.control.TextField;
import javafx.util.StringConverter;
import java.util.Locale;

public class TipCalcController {
    @FXML private TextField BillText;
    @FXML private TextField TipText;
    @FXML private Label TipLabel;
    @FXML private Label TotalLabel;
    @FXML private Slider TipSlider;

    public void initialize() {
        TipText.textProperty().bindBidirectional(TipSlider.valueProperty(), new StringConverter<Number>()
        {
            @Override
            public String toString(Number t)
            {
                return String.format(Locale.CANADA,"%.2f",t);
            }

            @Override
            public Number fromString(String string)
            {
                return Double.parseDouble(string);
            }

        });
    }

    @FXML protected void OnCalculateClick(ActionEvent event) {
        try {
            var bill = Double.parseDouble(BillText.getText());

            if (bill > 0.0) {
                try {
                    var tipPercent = Double.parseDouble(TipText.getText());

                    if (tipPercent > 0.0) {
                        var tip = Math.round(tipPercent*bill) / 100.0;

                        TipLabel.setText(String.format(Locale.CANADA,"The tip is %.2f",tip));
                        TotalLabel.setText(String.format(Locale.CANADA,"The total is %.2f",bill+tip));

                        return;
                    }
                    //Fall back to error message just after the try clause if value <= 0
                }
                catch (NumberFormatException e) {
                    // Swallow th exception and fall back to error message just after the try clause
                }

            }
            //Fall back to error message just after the try clause if value <= 0
        }
        catch (NumberFormatException e) {
            // Swallow th exception and fall back to error message just after the try clause
        }

        TipLabel.setText("Please enter positive numbers");
        TotalLabel.setText("");
    }
}
