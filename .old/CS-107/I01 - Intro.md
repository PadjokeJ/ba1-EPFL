#introprog 
La programmation consiste à écrire des séquences d'informations qui seront traduites en instructions qu'un ordinateur executera. 

Écrivons le programme le plus basic, "Hello World":

```java
public class Main {
	public static void main(String[] args) {
		System.out.println("Hello World!");
	}
}
```
Ce programme ne fait que d'afficher 
```bash
Hello World!
```
dans la ligne de commande.
## L'anatomie d'un programme en java

Le Java est structuré par des **classes** qu'on a ici en ``public class``, qu'on a nommé main. À l'intérieur des classes on trouvera des instructions qui seront interprétées par l'ordinateur. ==Il est impératif que le nom de la classe soit le même que le nom du fichier==

Dans notre classe, on retrouve une déclaration de **fonction**, ici ``public static void``, nommée "main", qui prend comme arguments un ``String[] args``
Une fonction nommée main sera toujours celle qui sera appelée au lancement du programme. L'intérieur des déclarations de fonctions est toujours contenu par ``{}``. 

Nous avons ensuite la fonction ``System.out.println`` qui affiche son contenu dans la ligne de commande. 

[[I02 - Variables]]