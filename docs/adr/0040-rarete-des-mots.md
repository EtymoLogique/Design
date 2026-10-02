# ADR 0040 — Rareté des mots, dérivée de leurs briques

- **Statut** : Proposé, partiellement remplacé par l’[ADR 0043](0043-legendaires-chance-fixe-hors-garanties-et-secretes.md) : un mot légendaire n’a ni silhouette ni forme de rareté avant d’être trouvé. Le reste du présent ADR reste en vigueur.
- **Complète** : [ADR 0011](0011-plis-raretes-et-doublons.md) (raretés), [ADR 0015](0015-codex-fascicules-et-legendaires.md) (légendaires secrètes), [ADR 0035](0035-plis-d-affixes-ponderes-par-la-rarete.md) (plis d’affixes)
- **Remplace partiellement** : la règle du codex selon laquelle une carte de mot ne porte jamais de rareté ([codex.html](../codex.html))

## Contexte

La rareté ne qualifie aujourd’hui que les briques élémentaires : elle fixe leur chance au pli et l’encre de leurs doublons. Une carte de mot n’en porte aucune. Pourtant, écrire un mot qui exige une brique rare demande plus de plis et plus de patience qu’un mot fait de briques communes. Rien ne le dit au joueur, et rien ne l’en récompense.

Depuis l’ADR 0035, un pli ne tire jamais un mot : la rareté d’un mot ne peut donc pas servir au tirage. Elle ne peut que se lire et récompenser.

## Décision

### La rareté d’un mot se déduit de ses briques

- La rareté d’un mot est la **plus haute rareté parmi les briques qu’exige sa recette**. Elle n’est pas éditée : aucune donnée à saisir, aucune relecture de plus.
- Un emplacement à variantes compte pour sa variante **la moins rare** ; un mot à plusieurs recettes, pour sa recette **la moins rare**. La rareté dit ce que le mot coûte au plus court.
- Seuls les préfixes et les suffixes ont une rareté publiée au pli. Un mot posé comme brique compte comme **commun** : il a déjà été gagné.
- Un mot qui exige une légendaire est donc légendaire. Il reste secret tant que la légendaire est inconnue ([ADR 0015](0015-codex-fascicules-et-legendaires.md)).
- Le serveur (`etymo_domaine::pli::rarete_mot`) et la PWA (`rareteMot`) appliquent la même règle.

### Elle se voit

- La carte d’un mot au codex, et sa silhouette, portent la forme de sa rareté, comme celle d’un affixe.
- La forme d’un mot n’est **jamais Ambre** : l’Ambre reste la couleur des plis et des briques qu’ils donnent. Elle prend une teinte de son fond, sans contour : plus foncée sur un fond clair, plus claire sur un fond foncé (le bandeau Encre d’une carte de mot, la fiche en thème sombre). Jamais de blanc.
- La fiche de découverte affiche la forme et son libellé. L’annonce grandit avec la rareté, comme à l’ouverture d’un pli : « Rare ! », « Légendaire ! », et la forme tourne en apparaissant, sans halo.

### Elle rapporte de l’encre

- La première découverte d’un mot rapporte des gouttes selon sa rareté : paramètre `encre_decouverte` de l’équilibrage du fascicule qui publie le mot ([ADR 0009](0009-contenu-et-equilibrage.md)).
- Valeurs de départ, à mesurer : commune 2, peu commune 3, rare 5, légendaire 10.
- Le serveur crédite l’encre dans la transaction de la découverte, avec le motif `decouverte`. Une commande rejouée ne paie pas deux fois. Un mot déjà connu ne rapporte rien.

## Options envisagées

### Rareté éditée mot par mot

Écartée : une donnée de plus à saisir et à relire, qui peut contredire les briques du mot (un mot « commun » fait d’une brique rare).

### Rareté la plus haute de toutes les variantes

Écartée : une variante rare rendrait rare un mot qu’on peut écrire avec des briques communes.

### Affichage seul, sans encre

Écartée : la rareté d’un mot ne changerait rien au jeu, et l’effort d’obtenir une brique rare ne serait pas récompensé à la table.

## Conséquences

### Positives

- Le joueur voit quels mots demandent le plus de patience, et il en est récompensé.
- Aucune donnée nouvelle dans la couche linguistique ; la couche ludique ne gagne qu’un barème.
- La rareté ne dépend toujours pas du niveau de confiance ([ADR 0009](0009-contenu-et-equilibrage.md)).

### Négatives

- Une nouvelle source d’encre : le rythme des sabliers et des indices ([ADR 0038](0038-indices-a-prix-croissant-et-format-des-jalons.md), [ADR 0039](0039-plafonner-l-achat-des-sabliers-pas-leur-usage.md)) est à recalculer.
- Changer la rareté d’un affixe change celle des mots qui l’exigent, y compris ceux déjà découverts.
- Un manifeste antérieur, sans `encre_decouverte`, ne paie rien à la découverte.

## Critères de réévaluation

- Des joueurs qui gardent leurs briques rares pour les mots rares au lieu de composer : baisser le barème.
- Des mots ingrédients rares à obtenir ([ADR 0035](0035-plis-d-affixes-ponderes-par-la-rarete.md)) : leur donner leur propre rareté dérivée.
