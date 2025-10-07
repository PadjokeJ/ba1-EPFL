#introprog 

Est une chaîne de caractères.

Le type ``char`` :
Une variable qui stocke qu'un seul caractère.

On peut donc utiliser plusieurs caractères pour former un ``String``. Comme pour les tableaux, une variable de type ``String``stocke une référence à un emplacement de mémoire. ==Il ne suffit donc pas d'utiliser l'opérateur ``==`` pour vérifier l'égalité des ``String``, ou ``=`` pour en affecter==

```java 
String bar = "bar";
String foo = "foo";

bar = foo; // Désormais bar pointe vers la même zone de mémoire que bar
```

 >[!Warning] Attention
 >Si on déclare deux chaines à la même valeur, elles pointent vers la même adresse. 
 >```java 
 >String foo = "foo";
 >String foo2 = "foo";
 >assert foo == foo2; //true
 >```
 
 En imprimant une chaîne à la console