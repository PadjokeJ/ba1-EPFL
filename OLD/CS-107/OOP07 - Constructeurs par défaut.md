#introprog 

Le constructeur par défaut est un constructeur qui n'a pas de paramètres. 

```Java
NomClasse() {/* do sth */} // <- constructeur par défaut
NomClasse(int a) {/* do sth with a*/}
/* ... etc ... */
```
On peut appeler les autres constructeurs dans un constructeur :
```Java
NomClasse() {
	this(valeur1, valeur2 /*, etc...*/);
}
```

On peut également donner une valeur par défaut aux éléments d'une classe

```Java
class NomClasse {
	private double a = 0.0;
	private int b = 3141;

	public NomClasse() {}
	
	public NomClasse(double val, int pi) {
		a = val;
		b = pi;
	}
}
```

[[OOP08 - Constructeur de copie]]