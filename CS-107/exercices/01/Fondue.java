import java.util.Scanner;
public class Fondue {
  public static void main(String[] args) {
    
    Scanner inp = new Scanner(System.in);

    int BASE = 4;
    float cheese = 800f;
    float water = 2f;
    float garlic = 2f;
    float bread = 400f;
    
    System.out.println("Good choice of fondue :)");
    System.out.println("How many folks are expected to eat this fondue? (Negative numbers will not change the recipe, you can't yet cook with anitmater afaik)");
    System.out.print("people : ");
    
    int folks = inp.nextInt();

    if (folks - BASE > -4) {
      float ratio = (float) (folks) / (float) BASE;
      cheese *= ratio;
      water *= ratio;
      garlic *= ratio;
      bread *= ratio;
    }
    
    System.out.println();
    System.out.println("To make REAL fondue, you need:");
    System.out.println("- " + String.valueOf(cheese) + " grams of Vacherin");
    System.out.println("- " + String.valueOf(water) +  " dl water");
    System.out.println("- " + String.valueOf(garlic) + " units (i forgor word) of garlic");
    System.out.println("- " + String.valueOf(bread) +  " grams of bread");
    System.out.println("- UNLIMITED PEPPER!!");

  }
}
