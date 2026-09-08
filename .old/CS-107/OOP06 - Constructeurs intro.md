#introprog 

Pour initialiser les attributs à des valeurs de base, il faut bien pouvoir initialiser. Mais est-ce possible de le faire facilement avec des valeurs autres que celles par défaut ?

En Java, on peut utiliser des constructeurs pour faire cela. 

```Java
NomClasse (Type param1, /*...*/, Type param_n) {
	//initialisation des paramètres
}
```
Ils n'ont pas de type de retour, et doivent être du même nom que la classe. Ils peuvent être surchargés (plusieurs fois la fonction avec différents paramètres)

On déclarerait ainsi une classe :
```Java
NomClasse nomObjet = new NomClasse(param1, /*...**/, param_n);
```

[[OOP07 - Constructeurs par défaut]]