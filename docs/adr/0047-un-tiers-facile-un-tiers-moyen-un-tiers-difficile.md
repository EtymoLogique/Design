# ADR 0047 — Chaque fascicule : un tiers facile, un tiers moyen, un tiers difficile

- **Statut** : Proposé
- **Complète** : [ADR 0022](0022-fascicules-de-20-a-30-mots.md) (taille d’un fascicule) ; [ADR 0009](0009-contenu-et-equilibrage.md) (équilibrage dans la couche ludique)

## Contexte

Rien ne règle aujourd’hui la difficulté d’un fascicule. Il pourrait ne contenir que des mots transparents, que l’on trouve sans réfléchir, ou que des mots dont la forme et le sens ont dérivé, qui découragent un joueur débutant.

Une difficulté croissante d’un fascicule à l’autre a été envisagée, puis écartée : un joueur qui arrive tard reçoit les graines de tous les fascicules parus ([ADR 0030](0030-reserve-de-depart-par-fascicule.md)) et peut commencer par n’importe lequel. Chaque fascicule doit donc offrir à la fois des entrées faciles et de vrais défis.

## Décision

### La règle des tiers

- Chaque fascicule compte **un tiers de mots faciles, un tiers de mots moyens et un tiers de mots difficiles**, à un mot près.
- Les mots croisés comptent dans ces tiers ; les mots légendaires restent à part, hors des compteurs ([ADR 0043](0043-legendaires-chance-fixe-hors-garanties-et-secretes.md)).

### Trois niveaux

| Niveau | Définition |
|---|---|
| Facile | Le sens actuel est proche du sens littéral, et la recette n’a aucune transformation (*géographie*). |
| Moyen | Un seul écart : une transformation visible (*philanthrope*, où le o de *philo-* tombe), ou un sens actuel qui s’est éloigné du sens littéral (*philologie*). |
| Difficile | Les deux écarts à la fois : la forme se transforme et le sens a dérivé. |

- Chaque mot porte un champ `difficulte` dans la couche ludique, saisi par l’équipe éditoriale avec sa recette et relu avec elle.
- Le validateur avertit quand un fascicule s’écarte des tiers de plus d’un mot.

### Un réglage invisible

- La difficulté n’est jamais affichée au joueur.
- Elle sert à l’équilibrage, et peut guider le choix de la quête de table du jour ([ADR 0045](0045-quetes-du-jour-carte-de-lecteur-et-quetes-au-long-cours.md)).
- La rareté d’un mot ([ADR 0040](0040-rarete-des-mots.md)) reste une autre mesure : elle dit ce que coûtent ses briques, pas ce que demande sa déduction.

## Options envisagées

### Une difficulté croissante d’un fascicule à l’autre

Écartée : on garde une difficulté moyenne dans chaque fascicule, puisque le joueur peut commencer par n’importe lequel.

### Une difficulté calculée sans jugement éditorial

Écartée : le nombre de transformations se calcule, mais l’écart entre sens littéral et sens actuel demande un jugement.

### Afficher la difficulté

Écartée : un signe de plus sur les cartes, et une étiquette « difficile » peut décourager avant même d’essayer.

## Conséquences

### Positives

- Chaque fascicule offre environ huit à dix entrées faciles, pour un joueur qui arrive comme pour un habitué.
- Les défis restent présents dans chaque fascicule, sans courbe à suivre d’un mois à l’autre.

### Négatives

- Une contrainte de plus pour composer un fascicule, en plus de la fermeture et des 20 à 30 mots.
- Un champ de plus à saisir et à relire, sur un jugement qui peut varier d’une personne à l’autre.
- Le skill `nouveau-fascicule` du dépôt de contenu est à mettre à jour.

## Critères de réévaluation

- Les taux de découverte par niveau ne se distinguent pas : la définition des niveaux ne mesure pas la difficulté réelle.
- Les mots difficiles restent introuvables pour la plupart des joueurs, malgré les indices.
