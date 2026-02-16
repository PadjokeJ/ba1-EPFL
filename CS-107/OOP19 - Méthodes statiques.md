#introprog 

Il est également possible de faire en sorte qu'une méthode soit statique. Comme les variables statiques, elles sont accessibles sans instancier une classe. 

```Java
class Example {
	static void staticMethod() {}
	void normalMethod() {}
}
class Main {
	public static void main(String[] args) {
		Example.staticMethod(); // ok
		// Example.normalMethod(); !! NO !!
		Example a = new Example();
		a.staticMethod(); // possible, but not used
		a.normalMethod(); // ok
	}
}
```

Dans ces méthodes, on ne peut pas utiliser le mot clef `this` ou les variables d'instances.

[[OOP20 - Interfaces]]