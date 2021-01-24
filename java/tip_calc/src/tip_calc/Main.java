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
        primaryStage.setTitle("Hello World");
        primaryStage.setScene(new Scene(root, 300, 275));
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

        var tip = Math.round(tipPercent*bill) / 100.0;

        System.out.format("The tip is %.2f%nThe total is %.2f%n", tip, bill+tip);
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
