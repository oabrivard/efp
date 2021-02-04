package fr.abrivard.sayinghello;

import java.util.Scanner;

public class App
{
    public static void main( String[] args )
    {
        System.out.println( "What is your name?" );
        var scanner = new Scanner(System.in);
        var name = scanner.nextLine();
        var message = "Hello %s, nice to meet you!".formatted(name);
        System.out.println( message );
    }
}
