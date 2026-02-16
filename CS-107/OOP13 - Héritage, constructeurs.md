#introprog 

L'instanciation d'une classe s'accompagne d'une initialisation des attributs. Cette tâche n'est pas toujours faite intégralement par le constructeur d'une sous-classe (exemple : attributs privés.)
Mais on peut les initialiser en utilisant le mot clef ``super``
```Java
SousClasse (/* liste de paramètres */) {
	super(/* liste d'arguments */);
	/* ... reste de l'initialisation */
}
```

Par example avec une classe "carré"

```Java
class Rectangle {
	private double x;
	private double y;
	
	private double height;
	private double width;
	
	public Rectangle(double x, double y, double h, double w) {
		this.x = x;
		this.y = y;
		height = h;
		width = w;
	}
	/* setters, getters, and other manip... */
}
class Square extends Rectangle {
	public Square(double x, double y, double size) {
		super(x, y, size, size);
	}
}
```

## Ordre d'appel de constructeurs

Dans une relation d'héritage, l'initialisation d'une sous-classe appelle d'abord la classe la plus en haut de l'hierarchie. 
![[Pasted image 20251103104139.png]]

[[OOP14 - Polymorphisme - introduction]]