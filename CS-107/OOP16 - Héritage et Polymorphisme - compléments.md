#introprog 

Les constructeurs ne sont pas polymorphiques. Cependant il est possible d'invoquer une méthode polymorphique dans un constructeur.

```Java
abstract class A {
	public abstract void runWhenCreated();
	
	public A() {
		runWhenCreated();
	}
}
class B extends A {
	private int b;
	public B() {
		b = 1;
	}
	public void runWhenCreated() {
		assert b == 0; // --> this is true, because runWhenCreated runs implicitly
	}
}
```

## Super Classe `Object`

Tous les objets héritent en réalité de la classe `Object`, qui contient des fonctions tel `toString()` et `equals`

## Surcharge et redéfinition

Il existe en Java deux moyens d'avoir des méthodes avec le même nom
- surcharge (overload)
- redéfinition (overriding)
Si il y a plusieurs méthodes de même nom dans une seule classe -> surcharge
Si c'est une redéfinition de la méthode de la superclasse -> redéfinition

## Méthode `equals`

La méthode `equals` prend le type générique `Object` comme argument, car on ne peut pas changer les types ou arguments des fonctions redéfinies. 

[[OOP17 - Le modificateur `final`]]