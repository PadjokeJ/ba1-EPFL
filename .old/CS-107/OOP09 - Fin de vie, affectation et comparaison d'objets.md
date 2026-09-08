#introprog 

## Fin de vie

On dit qu'un objet quand le programme n'en a plus besoin. Il est donc utile de libérer la zone mémoire utilisée par cette objet, pour l'utiliser pour d'autres choses.

## Affectation et copie

Si on affecte à un objet à un autre sans le cloner, alors ils ne sont pas deux objets distincts.
```Java 
Rectangle r1 = new Rectangle(6.0, 7.0);
Rectangle r2 = r1;
assert r1.hauteur == r2.hauteur; // true
r1.hauteur = 9.0;
assert r2.hauteur == 9.0; //true
```
Il faut donc passer par [[OOP08 - Constructeur de copie|un constructeur de copie]]

## Affichage

Quand on imprime utilisant ``System.out.println()``, seule la référence de la zone mémoire de l'objet est imprimée. 
On peut déclarer une fonction ``String toString() {/*...*/}`` dans l'objet, qui sera utilisée par Java pour pouvoir l'afficher (automatiquement).

## Comparaison

Comme mentionné plus haut, la comparaison ``==`` en java ne fait que de comparer les références, et non les valeurs de l'objet. Il faut donc déclarer une fonction pour pouvoir comparer deux objets. 
```Java
public boolean equals(NomClasse autreObjet) {
	if (autre == null)
		return false;
	return (this.param_1 == autreObjet.param_1 
		&& /* ... */ 
		&& this.param_n == autreObjet.param_n); 
}
```

On peut également utiliser le type prédéfinit ``Object``, qu'on verra dans la prochaine lesson

[[OOP10 - Héritage, concepts]]