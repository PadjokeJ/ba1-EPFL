import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner scan = new Scanner(System.in);
    
    int currYear = 2025;

    System.out.println("What is your age?");
    System.out.print("age : ");
    
    int age = scan.nextInt();

    System.out.println("You were most likely born in " + String.valueOf(currYear - age));
  }
}
