# ADR 0066 — Aucune araignée dans les visuels

- **Statut** : Accepté
- **Portée** : tous les visuels du jeu et de ses supports : silhouettes des cartes, jaquettes, illustrations, icônes, maquettes, captures et communication
- **Complète** : [ADR 0023](0023-textures-des-cartes.md) (silhouettes des cartes), [ADR 0016](0016-plis-et-jaquettes-par-fascicule.md) (jaquettes)

## Contexte

La peur des araignées est l’une des phobies les plus répandues. Une silhouette d’araignée suffit à mettre mal à l’aise une partie des joueurs, alors que le codex se veut un lieu qu’on a plaisir à parcourir, et une carte trouvée reste affichée longtemps.

La question s’est posée avec le fascicule n° 11 : la première silhouette de *zoopsie* (hallucination où l’on voit des animaux) montrait une araignée au bout de son fil. Rien n’interdisait ce sujet.

## Décision

- **Aucun visuel ne représente une araignée**, ni en entier, ni en partie (pattes, toile, fil d’araignée), ni de façon stylisée. Cela vaut pour les silhouettes des cartes, les jaquettes, les illustrations, les icônes, les maquettes et tout support de communication.
- Un mot dont le sens touche aux araignées (*arachnophobie*, *arachnide*) garde sa place dans le catalogue : sa silhouette choisit un autre sujet, par exemple la réaction de la personne ou un objet du contexte.
- La règle s’ajoute aux « Formes à éviter » du skill `silhouette` du dépôt de contenu, et se vérifie à chaque revue de planche.

## Options envisagées

### Laisser le choix au cas par cas

Écartée : un dessin d’araignée réussi reste une araignée. La gêne ne dépend pas de la qualité du dessin.

### Interdire tous les animaux qui suscitent une phobie courante

Écartée pour l’instant : serpents, insectes ou rongeurs servent des mots du catalogue (*nécrophage*, *zoopsie*) et se dessinent sans effet de surprise. L’araignée est le cas le plus fréquent et le plus net.

## Conséquences

### Positives

- Aucun joueur ne tombe sur une araignée en ouvrant son codex.
- La règle est simple à appliquer et à vérifier en revue.

### Négatives

- Quelques mots perdent leur sujet le plus évident et demandent un dessin moins direct.

## Critères de réévaluation

- Des retours de joueurs signalent un malaise devant d’autres sujets (serpents, insectes) : étendre la liste.
- Un mot du catalogue ne peut se dessiner lisiblement sans araignée : trancher ce cas dans un nouvel ADR.
