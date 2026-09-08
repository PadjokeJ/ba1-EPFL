#introprog 

Les exceptions ont comme super-classe les objets `Throwable` 

On a comme méthodes 
`getMessage()` -> contient un message décrivant l'erreur
`printStackTrace()` -> permet d'afficher les appels faits avant l'erreur

![[Pasted image 20251117133401.png]]

Les `Error` sont des erreurs fatales. Ces erreurs sont trop graves, et ne peuvent être gerées

Il existe deux types d'`Exception` ->
- Les "checked" exceptions -> Le programmeur doit gerer ces exceptions
- Les "unchecked" exceptions -> Ne sont pas obligées d'être traitées

Lancement d'erreurs :
- Créer un objet Exception
- Le lancer avec `throw`
```Java
throw new Exception("message d'erreur");
```

Endroits réceptifs :
- Lancer les exceptions dans un endroit réceptif aux erreurs -> blocs try-catch
```Java
try {
	throw new Exception("erreur !");
} catch (Exception e) {
	/* ... */
}
```

Traitement des exceptions 
- Un bloc try peut être associé à plusieurs `catch`
- Un bloc `catch` traite une exception, selon un type d'exception

```Java
catch (TypeException nomVariable) { /*...*/ }
```

Une fois un bloc catch utilisé, tous les autres du try sont sautés

Faire le ménage :
Le mot clef `finally` -> sera toujours exécuté

[[OOP24 - Gestion des exceptions - compléments]]

