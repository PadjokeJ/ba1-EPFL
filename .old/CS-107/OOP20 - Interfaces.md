#introprog 

Java ne tolère que l'héritage simple. Cet à dire qu'une classe ne peut que dériver directement d'une seule classe. Mais que faire si on veut faire que des classes partagent les mêmes caractéristiques (mais pas toutes), d'une super classe.

On fait appel à la notion d'interfaces. Cela impose l'implémentation de méthodes dans une classe, sans "hériter" d'une classe.

```Java
interface nomInterface {
	void methodeInterface();
}
```

Les interfaces ne contiennent que des méthodes abstraites, c'est à dire qu'il faut les re-implémenter pour chaque classe.

Le mot clef `public` n'est pas nécessaire et est même omis car c'est le comportement par défaut d'une interface.

Pour pouvoir ajouter le comportement d'une interface, il faut ==l'implémenter==
```Java
public class NomClasse extends SuperClasse implements nomInterface, nomInterface2, /* ..., */ nomInterfaceN { /* ... */}
```

>[!Check] Les interfaces indiquent qu'une classe "se comporte comme" l'interface.

>[!warning] Il n'y a pas de constructeurs pour les interfaces

On peut cependant déclarer des variables de type de l'interface.
```Java
NomInterface inter;
NomClasse objet = new NomClasse();
inter = objet;
```

[[OOP21 - Interfaces - Java 8]]