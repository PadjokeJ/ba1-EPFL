#introprog 

Cacher une image/du texte cachés dans une autre image.
Et de les trouver

# Objectifs

Auto-eval
Reflexes de test
Debugger
Git/Collabo
Entrainement MP02

## Git 

voir [[Gnugen - git 01]]

Permet de travailler en collabo, en gardant un historique complet des modifications.

**Le but n'est pas de se focus sur git**

# Représentation données

## Texte

UTF-8 texte sur 8 -> 32 bits - le plus commun ajd
ASCII texte sur 8 bits (les premiers 128 chars)

==DANS LE MP01 NOUS UTILISONS UTF-8==
On utilisera des tableaux de byte (8 bits)

Dans Text.java (fourni), on a les fonctions fournies :
``toBytes()`` et ``toString()``

## Image

Pixels représentés en ARGB (alpha, rouge, gvert, bleu)
chaque sous couleur a 8 bits. (0 - 255)

## Cryptographie

Chiffrer du texte/image en choses pas reconnaissable
### Vocab

- plaintext = message à décrypter
- cipher text = message chiffré
- cipher = système de chiffrement/déchiffrement
- key = info secrète pour chiffrer/déchiffrer le message
### Ciphers

(Exemples en Main.java, décris dans la donnée de cours)
- César
	- Décalage de lettres dans l'alphabet par une constante (key)
	- Facile à casser, peu de possibilités (key = \[-128, 127\])
- Vigenère
	- Presque comme césar, mais le chiffrement dépend de la lettre
	- On a un mot qui est la clef, et on additionne message + clef (si ça overflow ya modulo)
	- Pour trouver on fait (CipherText - Key) mod max
	- On répète la clef
- OTP
	- On XOR les bits du message et ceux de la clef, en répettant la clef
	- CipherText xor Key donne la réponse
- CBC
	- On a un PAD, on découpe le message en petits bout de la même taille du PAD
	- XOR entre les blocs et le PAD. Le ciphertext du premier PAD sera utilisé comme PAD du deuxième bloc, et ainsi de suite
- XOR

## Stéganographie

Cacher dans une image

En modifiant qu'un seul bit (le dernier LeastSignificantBit) d'une couleur, il n'y a que dalle de différence

DEUX PROCÉDURES

- cacher une image noir/blanc dans le dernier bit d'une image. 
	L'image à cacher doit être plus petit ou égale en dimension de l'image originale.
	Les positions des pixels restent les même
- cacher du texte dans une image via le LSB de l'image
	On a donc l'encodage en bits d'un texte qu'on cache dans une image.

# Mise en place

- [Énoncé](https://proginsc.epfl.ch/wwwhiver/mini-projet1-2025/descriptif.html)
- [Énoncé complet](https://proginsc.epfl.ch/wwwhiver/mini-projet1-2025/crypto-stegano-fr.pdf)
- [Code original](https://proginsc.epfl.ch/wwwhiver/mini-projet1-2025/crypto-stegano.zip)
- [Mise en place](https://proginsc.epfl.ch/wwwhiver/mini-projet1-2025/setup-IntelliJ/setup-IntelliJ.html)

Les tests sont dans Main.java
Le reste du code est dans dossier utils, stegano et crypto, et le fichier Challenge.java

## Ordre

1. utils
2. crypto / stegano
3. Challenge.java

## Rat - Pelle

Syntaxe NomDeClasse.nomDeMethode()

!! go regarder Helper.java pour aide

# Correcteur auto

- Bien tester
- Que soumettre du code qui compile
- Faire gaffe aux cas limite
- JAMAIS UTILISER ``System. /*...*/``
- nommer fichier ``submission.zip``

EST LENT
tests sont randoms et peuvent marcher *parfois*