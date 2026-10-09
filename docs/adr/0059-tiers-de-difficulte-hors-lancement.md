# ADR 0059 — La règle des tiers à partir du onzième fascicule

- **Statut** : Proposé
- **Remplace partiellement** : [ADR 0047](0047-un-tiers-facile-un-tiers-moyen-un-tiers-difficile.md), pour les dix fascicules de lancement ; la règle des tiers, les trois niveaux et le champ `difficulte` restent en vigueur pour tous les autres
- **Complète** : [ADR 0054](0054-fascicules-de-lancement-plus-gros-et-plus-rapproches.md) (fascicules de lancement)

## Contexte

L’[ADR 0047](0047-un-tiers-facile-un-tiers-moyen-un-tiers-difficile.md) demande un tiers de mots faciles, moyens et difficiles dans chaque fascicule. Un mot difficile cumule deux écarts : sa recette porte une transformation, et son sens actuel s’est éloigné du sens littéral.

Les dix fascicules de lancement ([ADR 0054](0054-fascicules-de-lancement-plus-gros-et-plus-rapproches.md)) comptent 718 mots, dont 9 difficiles (après correction d’*épigramme*, sans transformation). Ce n’est pas une erreur de classement : ils sont bâtis sur des composés savants transparents (*biologie*, *géographie*, *thermomètre*), dont les briques gardent leur forme. Les fascicules 1 et 2 n’ont que 7 et 2 recettes avec transformation, donc au plus 7 et 2 mots difficiles possibles, pour 17 attendus. La relecture des fascicules 1 à 4 dans le TLFi (issue Content #13) le confirme.

Respecter les tiers demanderait d’ajouter environ deux cents mots difficiles et de retirer autant de mots faciles, chacun avec ses faits sourcés, sa recette, sa silhouette et son explication, à quelques semaines du lancement.

## Décision

- Les **dix fascicules de lancement** ne sont pas tenus à la règle des tiers. Leurs mots gardent leur champ `difficulte`, saisi et relu selon les trois niveaux de l’[ADR 0047](0047-un-tiers-facile-un-tiers-moyen-un-tiers-difficile.md) ; un classement faux reste une erreur à corriger.
- Le contrôle `fascicule.tiers` du validateur ne s’applique pas aux fascicules 1 à 10.
- À partir du **onzième fascicule**, la règle des tiers s’applique pleinement, à un mot près, légendaires exclues.
- Les mots difficiles ajoutés plus tard à un fascicule de lancement, par exemple par la fermeture, sont les bienvenus : ils ne sont pas obligatoires.

## Options envisagées

### Ajouter des mots difficiles et retirer des mots faciles

Écartée pour le lancement : environ deux cents mots à créer et autant à déplacer, avec leurs faits, leurs silhouettes et leur relecture. Le calendrier de l’[ADR 0054](0054-fascicules-de-lancement-plus-gros-et-plus-rapproches.md) ne le permet pas, et la qualité éditoriale en souffrirait.

### Reclasser des mots pour atteindre les tiers

Écartée : un mot sans transformation ne peut pas être difficile selon l’[ADR 0047](0047-un-tiers-facile-un-tiers-moyen-un-tiers-difficile.md). Reclasser pour passer le contrôle fausserait l’équilibrage qu’il sert à guider.

### Appliquer la règle au catalogue entier plutôt qu’au fascicule

Écartée : elle laisserait chaque fascicule ordinaire libre de déséquilibrer le tout, alors que l’[ADR 0047](0047-un-tiers-facile-un-tiers-moyen-un-tiers-difficile.md) veut des défis dans chaque fascicule.

## Conséquences

### Positives

- Les fascicules de lancement restent tels qu’ils ont été relus.
- Un catalogue de départ surtout transparent apprend la mécanique de fusion aux nouveaux joueurs, qui arrivent tous en même temps au lancement.
- La règle garde toute sa force pour les fascicules mensuels.

### Négatives

- Un joueur assidu trouve peu de vrais défis pendant les dix premières semaines : les indices, les ricochets et les légendaires doivent porter l’intérêt.
- La quête de table du jour, qui peut s’appuyer sur la difficulté ([ADR 0045](0045-quetes-du-jour-carte-de-lecteur-et-quetes-au-long-cours.md)), aura peu de mots difficiles à proposer au début.

## Critères de réévaluation

- Les joueurs assidus jugent les fascicules de lancement trop faciles, ou le taux de découverte y est bien plus haut que dans les suivants : ajouter des mots difficiles aux fascicules de lancement.
- Le onzième fascicule, tenu aux tiers, fait chuter la découverte : revoir la définition des niveaux.
