#introprog 

# Classes

Une classe est un nouveau type de données (variables), dont les instances sont des "objets". On définit une classe :
```Java
class NomDeLaClasse {
	//...
}
```

Une fois que cette classe est définie, il est possible de déclarer des variables avec ce nouveau type :
```Java
NomDeLaClasse nomDeLObjet;
```

Similairement aux [[I17 - Tableaux|tableaux]], et aux [[I18 - String|strings]], la variable contient la référence vers une instance de l'objet. 

# Attributs

On peut déclarer des attributs à l'intérieur d'une classe :
```Java
class NomDeLaClasse {
	int nom_attribut1;
	String nom_attribut2;
	//...
}
```

On peut accéder aux attributs d'une classe :
```Java
nomDeLObjet.nom_attribut1;
```

# Initialisation

Pour créer une instance d'une classe, on utilise le mot clef ``new`` :
```Java
NomDeLaClasse nomDeLObjet = new NomDeLaClasse();
```

Dans une nouvelle instance, tout les attributs ont des valeurs nulles:
- doubles : $0.0$
- int : $0$
- booléens : $0$
- objets : ```null```

# Méthodes

Les méthodes sont les fonctions déclarées à l'intérieur des objets. On a la syntaxe:
```Java
//...
type_retour nomMethode(type_param1 nom_param1/*, ...*/) {
	//...
	return variable_type_retour;
}
//...
```

Pour utiliser ces fonctions, on le fait de manière similaire aux attributs.
```Java
nomDeLObjet.nomMethode(valeur1 /*, ...*/);
```

>[!Example] Exemple
>```Java
>class Main {
>	public static void main(String[] args) {
>		Rectangle rect1 = new Rectangle();
>		rect1.hauteur = 3.0;
>		rect1.largeur = 4.0;
>		
>		System.out.println("Surface du rectangle : " + rec1.surface());
>	}
>	
>	class Rectangle {
>		double largeur;
>		double hauteur;
>		double surface() {
>			return hauteur * largeur;
>		}
>	}
>}
>```

[[OOP3 - public et private]]