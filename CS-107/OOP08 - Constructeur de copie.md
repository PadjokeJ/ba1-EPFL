#introprog 

Pour faire une copie (sans que ça soit la même référence), il faut créer un constructeur de copie, qui n'existe pas par défaut (contrairement à pleins d'autres langages)

```Java
public NomClasse(NomClasse choseACopier) {
	param_1 = choseACopier.param_1;
	/* ... */
	param_n = choseACopier.param_n;
}
```

[[OOP09 - Fin de vie, affectation et comparaison d'objets]]