#introprog 

Avec l'héritage, il est donc possible de rendre plus généraux des concepts qui sont présents dans plusieurs classes du programme. Il est possible de créer une classe dans laquelle il se trouve une méthode qui devra être définie pour chaque sous classe.

## Méthodes abstraites

La solution à ce problème sont les méthodes abstraites. Elle sont définies par le mot clef ``abstract``, qui ne possèdent pas de corps. 
```Java
public /*or private */ abstract /* type */ void nomMethodeAbstraite();
```
Cette fonction doit être redéfinie pour chaque sous-classe. 

## Classes abstraites

Toute classe possédant au moins une méthode abstraite doit également être une classe abstraite. Ces classes sont donc non-instanciables, et doivent être définie par le mot clef ``abstract``
```Java
public abstract class NomClasse { /*...*/ }
```

## Héritage dans les classes abstraites

Une classe qui hérite d'une super-classe abstraite doit redéfinir les méthodes abstraites pour devenir normale -> pouvoir en créer des instances.

```Java
class NomClasse extends NomClasseAbstraite {
	/* ... */
	public void nomMethodeAbstraite() {
		/* Définition de la méthode abstraite */
	}
}
```

[[OOP16 - Héritage et Polymorphisme - compléments]]