# ADR 0035 — Plis d’affixes, pondérés par la seule rareté

- **Statut** : Accepté, partiellement remplacé par l’[ADR 0043](0043-legendaires-chance-fixe-hors-garanties-et-secretes.md) : les légendaires n’ont plus de poids ; elles se partagent une chance fixe de 3 % par pli. Le reste du présent ADR reste en vigueur.
- **Remplace partiellement** : [ADR 0011](0011-plis-raretes-et-doublons.md), pour le poids du type et la répartition du poids entre les briques d’une même classe ; [ADR 0016](0016-plis-et-jaquettes-par-fascicule.md), pour les briques que tire le pli d’un fascicule

## Contexte

L’ADR 0011 pondère une brique par le poids de sa rareté multiplié par le poids de son type (les suffixes à 1,25), puis répartit ce poids entre les briques de même rareté et de même type. L’ADR 0016 fait tirer au pli d’un fascicule toutes ses briques, nouvelles ou reprises.

La recette de J3 sur le fascicule d’essai en a montré deux effets :

- un mot posé comme brique (*mener*, *biologie*) sortait du pli comme un affixe, alors qu’un mot se découvre ;
- une brique seule dans sa classe prenait tout le poids de la classe : *mener*, seul mot commun, avait 23 % de chances, contre 14 % pour chaque suffixe commun.

## Décision

### Seuls les préfixes et les suffixes se tirent

- Le pli d’un fascicule ne tire que parmi ses **préfixes et ses suffixes**. Un mot n’est jamais tiré d’un pli, même s’il sert d’ingrédient à une recette.
- La garantie de nouveauté et le filet d’utilité choisissent eux aussi parmi ces seules briques.

### Le poids d’une brique est le poids de sa rareté

- La chance d’une brique est proportionnelle au **poids de sa rareté**, et à lui seul : commune 62, peu commune 26, rare 10, légendaire 2.
- Il n’y a plus de poids de type, ni de répartition du poids entre les briques d’une même classe. Deux briques de même rareté ont toujours la même chance.
- Les chances par type restent affichées : elles se déduisent des chances de chaque brique.
- Les poids restent des paramètres versionnés d’équilibrage ([ADR 0009](0009-contenu-et-equilibrage.md)).

## Options envisagées

### Garder le poids de type et la répartition par classe (ADR 0011)

Écartée : la chance d’une brique dépend du nombre de ses voisines de classe, ce qui la rend difficile à comprendre et fait d’une brique isolée la plus fréquente du pli.

### Laisser les mots dans le pli avec un poids de type faible

Écartée : un mot se découvre à la table ; le tirer d’un pli brouille la frontière entre découverte et collection.

## Conséquences

### Positives

- Une règle lisible : la rareté seule décide, et deux briques de même rareté ont la même chance.
- Le pli ne distribue que des affixes : les mots restent des découvertes.

### Négatives

- Un mot qui sert d’ingrédient ne s’obtient plus par un pli : il doit venir de la réserve de départ ([ADR 0030](0030-reserve-de-depart-par-fascicule.md)) ou d’une autre règle à décider. La validation d’atteignabilité ([ADR 0012](0012-briques-rationnees.md)) doit le vérifier.
- Les suffixes ne sortent plus plus souvent que les préfixes. Le rythme des fascicules est à recalculer.

## Critères de réévaluation

- Un mot ingrédient impossible à obtenir en quantité suffisante : décider comment une découverte donne des exemplaires de son mot.
- Des tests qui montrent trop peu de suffixes pour composer : revoir les raretés éditées plutôt que réintroduire un poids de type.
