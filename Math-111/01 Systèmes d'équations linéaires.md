# Contenu du cours :

- Vecteurs
- Espaces vectoriels $\mathbb{R}^n$, $V_n$, etc...
- Matrices
	- diagonales
	- symétriques
- Transformations linéaires ($T(v)$, $A{\vec v}$)
- Endomorphismes !! Important !!

# Equations linéaires

avec les variables $x_1$, $x_2$, ..., $x_n$ 
de forme
$a_1 x_1 + a_2 x_2 + \text{ ... } + a_n x_n = b$ 

$a_n$ : coefficients, dans $\mathbb{R}$ ou $\mathbb{C}$ 
$x_n$ : variables
$b_n$ : terme de droite, dans $\mathbb{R}$ ou $\mathbb{C}$ 
> [!Hint] **Rappel** : $\mathbb{N}$ et $\mathbb{Z}$ sont dans $\mathbb{R}$

> [!Warning] 
> Pas d'autre forme

la mutliplication des variables n'est pas une équation linéaire
ie : 
- $3 x_1 x_2 + 2 x_2 = 3$ n'est pas une équation linéaire
- $x_1^{-1}$ non plus

Exo : 
![[eqlin1.png]]

### équations et variables

a $m$ équations (en gros le nombre de lignes)  
a $n$ variables ($x_n$)                                               
	-> $a_{mn}$, $b_m$ 

On peut former une matrice à partir de tous les $a$ et $b$   
matrice dite "augmentée" : $$ \left(
\begin{array}{cc|c}
a_{1,1}&a_{1,2}&b_1\\
a_{2,1}&a_{2,2}&b_2
\end{array}
\right) $$
### Solutions

Une solution est une liste de nombres $s_n$ qui substituent $x_n$ de manière à ce que l'équation soit vérifée

est une liste ordonnée
$(s_1, s_2, \text{ ... }, s_n)$ : $n$-uplet de nombres dans $\mathbb{R}$ ou $\mathbb{C}$ 

La solution $S = \{(s_1, ..., s_n), ..., (\dot s_1, ..., \dot s_n)\}$
	Le nombre de listes dans $S$ indique le nombre de solutions



