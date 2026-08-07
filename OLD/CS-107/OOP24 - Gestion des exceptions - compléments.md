#introprog 

Relancement : 
```Java
try {
	int a = 1 / 0;
} catch (ArithmeticException e) {
	System.out.println("div par 0 !");
	
	throw e;
}
```
-> Traitement partiel

# Traitement ou déclaration :

Une fonction peut soit 
- traiter une exception dans un try-catch
- déclarer dans son entête qu'elle peut lancer une exception

```Java
public void nomMethode() throws Exception
```

# Exceptions spécialisées

Vu que ce sont des classes, ont peut hériter de la classe `Exception` pour créer nos propres exceptions

[[OOP25 - Etude de cas - présentation et modélisation du problème]]