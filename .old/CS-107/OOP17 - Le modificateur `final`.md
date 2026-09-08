#introprog 

Le modificateur `final` s'applique soit à des variables, méthodes, classes, etc...
On l'écrit après le modificateur \[public|private|protected\]
```Java
public final type nomFonction() {}
```

Ces fonctions ne peuvent pas être redéfinies dans des sous-classes 

Pour des variables, on peut utiliser ce mot clef afin de n'être capable que d'affecter la variable qu'une seule fois. On en fait ainsi une constante.

Cependant si un objet est passé en tant que argument dénoté par final dans une fonction. On peut quand même appelé des fonctions qui affectent des valeurs dans l'objet. Cela veut dire que le mot clef `final` n'empêche pas la modification, mais juste l'affectation

[[OOP18 - Attributs statiques]]