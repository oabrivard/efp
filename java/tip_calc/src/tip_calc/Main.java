package tip_calc;

import javafx.application.Application;
import javafx.application.Platform;
import javafx.fxml.FXMLLoader;
import javafx.scene.Parent;
import javafx.scene.Scene;
import javafx.stage.Stage;

import java.util.Scanner;

public class Main extends Application {

    @Override
    public void start(Stage primaryStage) throws Exception{
        Parent root = FXMLLoader.load(getClass().getResource("tip_calc.fxml"));
        primaryStage.setTitle("Tip Calc");
        primaryStage.setScene(new Scene(root, 265, 165));
        primaryStage.show();
    }

    private static double readPositiveDouble() {
        Scanner in = new Scanner(System.in);

        for(;;) {
            try {
                var line = in.nextLine();
                var value = Double.parseDouble(line);

                if (value > 0.0)
                    return value;

                //Fall back to error message just after the try clause if value <= 0
            }
            catch (NumberFormatException e) {
                // Swallow th exception and fall back to error message just after the try clause
            }

            System.out.print("Please enter a value >= 0 : ");
        }
    }

    private static void consoleUI() {
        System.out.print("What is the bill? ");
        var bill = readPositiveDouble();

        System.out.print("What is the tip percentage? ");
        var tipPercent = readPositiveDouble();

        var tipCalculator = new TipCalculator();
        var paymentDetails = tipCalculator.calulatePaymentDetails(bill, tipPercent);

        System.out.format("The tip is %.2f%nThe total is %.2f%n", paymentDetails.tip(), paymentDetails.totalAmount());
    }

    public static void main(String[] args) {
        if (args.length > 0 && args[0].equals("gui")) {
            launch(args);
        }
        else {
            consoleUI();
            Platform.exit();
        }
    }
}
