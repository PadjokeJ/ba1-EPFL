#introprog 

En programmation procédurale (ou impérative), les données et les traitements sont séparées dans un programme. 

>[!Example] Exemple
>Un programme qui calcule la surface d'un rectangle :
>```Java
>class Surface {
>	public static void main(String[] args) {
>		double largeur = 3.0;
>		double hauteur = 4.0;
>		
>		System.out.println("Surface du rectangle : " + surface_calc(double largeur, double hauteur));
>	}
>	
>	static double surface_calc(double l, double h) {
>		return l * h;
>	}
>}
>```
>Ici, il n'y a pas de lien entre les valeurs el la fonction ``surface_calc()``

En OOP, on regroupe les données et les fonctions dans une seule même entité, appelée objet. Cela permet d'avoir de la cohérence.
Ce système repose sur 4 principes :
1. L'encapsulation
2. L'abstraction
3. L'héritage
4. Le Polymorphisme

# Encapsulation et abstraction

## Encapsulation

Consiste à regrouper dans un seul et même objet les données et les traitements qui y agissent. 
Les variables se nomment *attributs*
Les fonctions se nomment *méthodes*
Ont les défini dans une *classe* qui est un nouveau type de données

## Abstraction

Identifie les caractéristiques et mécanismes communs aux objets utilisés, ce qui permet de donner une définition plus générique ou "abstraite" de l'ensemble. 


Ces deux concepts collaborent pour rendre l'utilisation des classes (pour un programmeur lambda qui n'est pas le concepteur) plus simple, car il n'a que besoin d'interfacer avec l'objet, car les détails de l'implémentations sont "cachés". 

[[OOP2 - Classes, Objets, Attributs et Méthodes]]