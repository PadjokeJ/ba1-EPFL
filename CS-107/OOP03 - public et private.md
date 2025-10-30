#introprog 

En OOP, il est possible de limiter quelles fonctions sont accessibles aux utilisateurs. On fait la distinction avec les keywords ``public`` et ``private``

## Private

Dit quelle partie d'une classe reste privé, et est un détail d'implémentation. On ne peut pas accéder à ces éléments depuis l'extérieur d'une classe. 

## Public

On indique quels membres de la classes sont accessibles, visibles et utilisables depuis l'extérieur. 
Par défaut, si ce n'est pas précisé, un membre d'une classe public.

## Accesseurs et manipulateurs

Ce sont des fonctions publiques qui permettent d'avoir accès à des éléments privés. 

Accesseur :
- get \[...\]
- permet de connaitre la valeur d'un champ privé
Manipulateur : 
- set \[...\]
- permet de changer la valeur d'un champ privé

## Masquage

C'est quand une variable a le même nom qu'une autre (surtout dans le cas des méthodes). Pour accéder à la valeur d'une classe, et non pas le paramètre de la fonction, on peut utiliser le mot-clef ``this``:
```java
public void method(int x) {
	this.x = x;
}
```
## Mémoire

Comme pour les String et tableaux, les objets sont manipulés via des références de zone de mémoire dans laquelle se contient l'objet. 
>[!Warning] La comparaison ``==``ne fait donc que de comparer les références!!!

[[OOP04 - Encapsulation et Abstraction - Résumé]]