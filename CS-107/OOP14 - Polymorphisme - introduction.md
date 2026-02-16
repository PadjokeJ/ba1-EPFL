#introprog 

Le polymorphisme permet à un même code de s'adapter aux types de données auquels il s'adapte.

## Résolution des liens

Vu que les sous classes héritent toutes les caractéristiques d'une superclasse, un objet peut donc avoir plusieurs types. 

Dans le code si-dessous, on peut se demander de quelle classe proviendra la fonction appelée
```Java
Player player = new Warrior();
player.meet(anotherPlayer);
```
Il y a deux résolutions possibles pour analyser le code. 
- **Résolution statique** : regarder le type de la variable
- **Résolution dynamique** : regarder l'objet lui même
En java, c'est la résolution dynamique qui prime, c'est donc la fonction spécialisée ``meet()`` qui est définie dans la classe ``Warrior`` qui sera utilisée, et non celle de ``Player``

On appelle cela polymorphisme, car la fonction est exécutée malgré le type de l'objet.

[[OOP15 - Classes et méthodes abstraites]]