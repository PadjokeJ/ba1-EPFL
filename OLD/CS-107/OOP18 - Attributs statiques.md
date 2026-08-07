#introprog 

On a pour l'instant trois "types" de variables.
Les locales (dans une méthode), les paramètres des méthodes et les variables d'instance. -> Attributs.

Si on veut faire en sorte qu'une variable soit la même pour toute instance de la classe, alors il faut utiliser le mot clef `static`

Elles appartiennent ainsi à la classe et y sont disponibles :
```Java
// ->> variableStatique = 0;
NomClasse.variableStatique = 52; 

/* ou avec l'instance d'une classe */
NomClasse classe = new NomClasse();
classe.variableStatique; // ->> = 52;
```

>[!Warning] Les variables statiques modifiées pour une instance sont modifiées pour toutes les autres instances et pour la classe

On utilise les attributs statiques notamment pour les constantes.
```Java
class Trigonometry {
	public static final double PI = 3.1415927;
}
```

Exemple concret :

La fonction `println` est issue de `out` qui est une variable statique de `System`
```Java
class System {
	/* ... */
	static PrintStream out = new PrintStream( /* ... */ );
	/* ... */
}
class PrintStream {
	/* ... */
	void println ( /* ... */ ) {
		/* ... */
	}
	/* ... */
}
```

[[OOP19 - Méthodes statiques]]