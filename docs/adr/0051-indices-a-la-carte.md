# ADR 0051 — Indices à la carte : silhouette, sens littéral, définition

- **Statut** : Accepté
- **Remplace partiellement** : [ADR 0038](0038-indices-a-prix-croissant-et-format-des-jalons.md), pour la section « Prix des indices » : les cinq niveaux ordonnés disparaissent. La partie sur les jalons reste en vigueur.
- **Complète** : [ADR 0023](0023-textures-des-cartes.md) : la silhouette d’une carte à trouver devient un indice payant.

## Contexte

Les cinq niveaux de l’ADR 0038 (langue, nature des briques, transformation, brique connue, solution) se suivent dans un ordre imposé et portent des métadonnées sèches, identiques pour tous les mots. Le joueur n’a aucun choix, et les premiers niveaux n’apprennent presque rien. Une carte du codex porte pourtant trois informations qui parlent d’elles-mêmes : sa silhouette, son sens littéral et sa définition.

## Décision

Un indice porte sur **une recette encore à découvrir**, comme avant. Les trois indices **s’achètent dans l’ordre** : silhouette, définition, puis sens littéral. Un indice n’est vendu qu’une fois le précédent obtenu, et chacun ne se paie qu’une fois par recette.

| Indice | Contenu | Prix |
|---|---|---|
| Silhouette | La silhouette de la carte, jusqu’ici cachée tant que le mot n’est pas trouvé, montrée comme une carte du codex sans texte ni bouton | 4 gouttes |
| Définition | La définition du mot | 8 gouttes |
| Sens littéral | Le sens de la composition, par exemple « étude de la vie » | 16 gouttes |

La feuille liste les indices dans cet ordre, du moins cher au plus cher ; seul le suivant à obtenir est achetable. Les boutons de prix sont neutres, avec la goutte de marque : un seul bouton Corail par écran, et un Corail ne porte pas d’icône colorée.

- Les prix sont un paramètre d’équilibrage ([ADR 0009](0009-contenu-et-equilibrage.md)). Le prix double d’un indice à l’autre : les trois coûtent 28 gouttes ensemble.
- Aucun niveau « solution » : le joueur trouve le mot lui-même.
- **Le sens littéral et la définition ne s’affichent plus gratuitement** sur une carte à trouver : la « piste » du codex et la note de la feuille d’indice disparaissent. Ils ne se lisent qu’après l’achat.
- Une recette n’offre que les indices que sa carte porte : une carte sans définition ne vend pas d’indice de définition, une carte sans silhouette commence à la définition.
- Une légendaire cachée ([ADR 0015](0015-codex-fascicules-et-legendaires.md)) ne reçoit toujours aucun indice.
- Les indices déjà achetés sous l’ancien barème sont supprimés, leur sens ayant changé, et leur prix est remboursé en encre.

## Options envisagées

### Garder les cinq niveaux et changer leur contenu

Écartée : les métadonnées des cinq niveaux restent sèches et identiques d’un mot à l’autre.

### Masquer le sens sous des gouttes à essuyer, ou révéler la silhouette lettre par lettre

Écartées après prototype : plus lourdes à lire et à équilibrer qu’un achat direct, sans apporter plus d’information.

### Garder une solution payante

Écartée : le joueur qui abandonne peut déjà obtenir la découverte autrement, et la solution rendait les autres indices inutiles.

## Conséquences

### Positives

- Le prix croît à chaque indice : le joueur paie peu pour un premier coup de pouce, cher pour aller au bout.
- Les indices réutilisent des données éditoriales qui existent déjà, sans écriture supplémentaire par carte.

### Négatives

- Le joueur ne choisit pas l’indice : pour lire le sens littéral, il paie d’abord les deux autres, 28 gouttes en tout.
- Un joueur bloqué n’a plus de solution à acheter.
- Les trois indices dépendent de la qualité de la fiche : une carte sans sens littéral ou sans définition offre moins d’indices.

## Critères de réévaluation

- Des joueurs bloqués qui abandonnent faute de solution : rétablir un dernier recours.
- Un indice jamais acheté : le retirer ou le baisser.
- Une silhouette trop reconnaissable pour son prix : la relever.
