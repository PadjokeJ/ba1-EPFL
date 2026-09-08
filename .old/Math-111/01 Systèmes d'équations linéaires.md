#alglin
See [[00.1 Introduction|00.1 Introduction]] 
# Contenu du cours :

- Vecteurs
- Espaces vectoriels $\mathbb{R}^n$, $V_n$, etc...
- [[#Matrices]]
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

Système d'équations linéaires

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


## Equations à deux inconnues

est de la forme $ax + by = c$ | $a$, $b$, $c$ $\in \mathbb{R}$

$ax + by = c$ peut être représentée par une droite dans le plan $\mathbb{R}^2$ 

donc avec 2 inconnues on peut le représenter par une collection de droites. 
Solution = point d'intersection de toutes les droites

> [!Info] Point d'intersection étant un point commun aux droites

## Equation à trois inconnues

de la forme $ax + by + cz = d$ | $a$, $b$, $c$, $d$ $\in \mathbb{R}$

Peut être représentée dans le plan $\mathbb{R}^3$ par des plans à deux dimensions (sys d'axe à trois dimensions)

Solution = point d'intersections de tous les plans

Si les plans 2D se croisent sur une droite, il y a un nombre infini de solutions

--- 

On constate que dans les deux types d'équations, l'ensemble des solutions est soit: 
- l'ensemble vide
- un nombre infini de solution
- une seule solution

### Démonstration : 

Théorème : Un système d'équations linéaires à $n$ innconnues à coeffs. réels satisfait à l'une de ces trois conditions: 
1. il n'y a aucune solutions
2. il y a un nombre infini de solutions
3. il n'y a qu'un seule solution

Preuve : Il suffit de voir que si le système possède deux solutions, alors il en possède une infinité.  (? what) (PS : ah ok, its for linear equations with more than 2 dimensions, this makes sense)

Soient $(a_1, ... , a_n)$ et $(b_1, ..., b_n)$ deux solutions du système

On considère $a_{i1}x + ... + a_{in}x = b_i$
On a 
$a_{i1}\alpha_1 + a_{i2}\alpha_2 + ... + a_{in}\alpha_n$
$-(a_{i1}\beta_1 + a_{i2}\beta_2 + ... + a_{in}\beta_n)$
--- 
$a_{i1}(\alpha_1 - \beta_1) + a_{i2}(\alpha_2 - \beta_2) + ... + a_{in}(\alpha_n - \beta_n) = 0$ 

> [!Warning]
> Incomplete ! go watch [this video](https://app.courseware.epfl.ch/learning/course/course-v1:EPFL+Algebre123+2025/block-v1:EPFL+Algebre123+2025+type@sequential+block@b01b380fb7ff4e2da97431d15a089f0e/block-v1:EPFL+Algebre123+2025+type@vertical+block@eff19aeb86474ca78c74f703029073cc)


## Opérations élémentaires

>[!Hint] **Rappel :** $S$ est l'ensemble de solution d'un sys. d'eq. lin.

1. Permutation de deux équations dans le système
	- On obtient un nouveau système $s'$ avec deux équations échangées
	- Une solution du système $s$ est également une solution du système $s'$ 
	- $S=S'$
2. Multiplier toute une équation par un réel non nul (donc multiplier tous les coefficients)
	- On obtient un nouveau système $s''$ où une équation est mutlipliée par un entier
	- Une solution du système $s$ est également une solution du système $s''$ 
	- $S = S''$
3. On rajoute à une équation un multiple d'une autre 
	- On obtient un nouveau système $s'''$ avec une équation $l_i$ + $\lambda l_j$ 
	- Une solution du système $s$ est également une solution du système $s'''$
	- $S = S'''$ 
On peut combiner ces trois opérations pour trouver les solutions du premier système d'éq lin

# Matrices

Un tableau rectangulaire avec $a_{ij} \in \mathbb{R}$
s'appelle une matrice $m\times n$ à coefficients réels

>[!Info] $m$ lignes, $n$ colonnes


$m \times n$ : taille
$a_{ij}$ = coeff à l'intersection de la $i$ ^e ligne et $j$ ^e colonne
$A = a_{ij}$

Soient $A$ et $B$ deux matrices.  
$A = B$ $\iff$ $m_B = m_A$ et pour tout $i, j$, $a_{ij}=b_{ij}$  
Soit un sys d'eq. lin. avec $n$ inconnus et $m$ équations
On associe *la matrice des coeffs.*  $$ A=\left(\begin{matrix}
a_{11} & ... & a_{1n} \\
\vdots & \ddots & \vdots \\
a_{m1} & ... & a_{mn} \\
\end{matrix}\right)$$
On associes *la matrice augmentée* $$ A=
\left(\begin{array}{ccc|c}
a_{11} & ... & a_{1n} & b_1 \\
\vdots & \ddots & \vdots & b_i \\
a_{m1} & ... & a_{mn} & b_m \\
\end{array}\right)$$
On indique l'[opération élémentaire](#Opérations%20élémentaires) qu'on fait sur une flèche "$\to$" entre les matrices, du style :
1. $L_i \leftrightarrow L_j$ 
2. $L_i = \lambda L_j$
3. $L_i = \lambda L_j + L_i$ 


## Matrices échelonnées

soit une matrice $A$ 
$A$ est échelonnée si : 
- le premier coeff. non nul dans la ligne $i+1$  doit se trouver à droite du premier coeff. non nul dans la ligne $i$ --> Pivot
- toute ligne nulle (tout $a_{ij} = 0$) n'est suivie que d'autres lignes nulles

>[!example]- Exemple
 >$$ A=\left(\begin{matrix} 1 & 1 & 3 \\ 0 & 2 & 4 \\ 0 & 0 &   \\ \end{matrix}\right)$$
### Matrice échelonnée réduite

$A$ est échelonnée réduite si :
- chaque pivot est égal à $1$
- le seul coeff. non nul dans une colonne est le pivot de la colonne

>[!Example]- Exemple
 >$$A=\left(\begin{matrix} 1 & 0 & 3 \\ 0 & 1 & 4 \\ 0 & 0 & 0 \\ \end{matrix}\right)$$

### résolution d'un sys d'eq avec une matrice

En échelonnant une matrice associée à un système d'équations, on peut facilement trouver une solution au systeme. en déclarant un $x_n$ à une variable arbitraire $r$, et autres si besoin

Il est plus facile de résoudre avec une matrice échelonnée réduite, car chaque pivot vaut une valeur de la dernière colonne


>[!Hint] Remarque
>On échelonne une matrice en utilisant les [[#Opérations élémentaires]]

