#introprog 

Dès Java 8, le concept des classes a été augmenté. 
On peut désormais
--> Déclarer des méthodes avec des définitions par défaut
--> Déclarer des méthodes statiques

## Méthodes par défaut

>[!Todo] Il faut utiliser le mot clef `default`

>[!Example] Example
>```Java
>interface NomInterface {
>	default void methodeParDefaut() { /* ... */ }
>}
>```


>[!Warning] Il est possible de faire des ambiguïtés -> 4 règles fondamentales

1. **Les définitions par défaut s'héritent**
   Il n'est pas nécessaire de les redéfinir -> sont inclues automatiquement dans une classe quand implémentées
2. **Une classe n'est pas obligée de redéfinir les méthodes par défaut d'une interface**
   Cela causait des problèmes en voulant mettre à jour / ajouter de nouvelles fonctions aux interfaces
3. **Les méthodes de la classe ont la préséance sur les méthodes d'interface**
   Les méthodes par défaut des interfaces sont ignorées si une méthode avec les mêmes entêtes sont définies dans la classe. 
   On peut cependant utiliser les méthodes via le mot clef `super`
   
   ```Java
   class NomClasse implements NomInterface {
	   public void methodeParDefaut() {
		   NomInterface.super.methodeParDefaut();
	   }
   }
   ```
   
4. **C'est à la classe de lever l'ambiguïté si deux interfaces ont un méthode de même entête**
   Si il y a un conflit, c'est la classe qui doit choisir quelle fonction par défaut elle implémente
   -> Redéfinir quelle méthode sera utilisée

>[!Question] Mais alors quand utiliser une classe abstraite ou une interface ?

Les interfaces n'ont pas de constructeurs.
On utilise des interfaces si on veut établir un lien fonctionnel entre deux classes

[[OOP22 - Gestion des exceptions - intro]]