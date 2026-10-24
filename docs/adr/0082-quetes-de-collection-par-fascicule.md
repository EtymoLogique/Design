# ADR 0082 — Quêtes de collection par fascicule

- **Statut** : Proposé
- **Complète** : [ADR 0045](0045-quetes-du-jour-carte-de-lecteur-et-quetes-au-long-cours.md) (une condition fermée de plus, au niveau du fascicule), [ADR 0042](0042-page-quetes-et-recompenses-a-recuperer.md) (un bloc de plus dans l’onglet « Fascicules »)

## Contexte

Une fois ses jalons atteints, rien ne pousse le joueur à **terminer** un fascicule. Il lui faut un objectif lisible et fini : « tous les préfixes », « tous les suffixes », « tous les mots communs ». Une quête « ligne du codex complétée » a été écartée, car la ligne dépend de la largeur de l’écran.

Le besoin initial parlait de « toutes les cartes de difficulté X ». Or la difficulté n’est jamais affichée ([ADR 0047](0047-un-tiers-facile-un-tiers-moyen-un-tiers-difficile.md)), et les fascicules parus n’ont que 0 à 2 mots difficiles. La rareté des mots, elle, est affichée ([ADR 0040](0040-rarete-des-mots.md)) et donne des quêtes jouables : de 19 à 56 mots communs, de 14 à 37 peu communs, 3 ou 4 rares quand un fascicule en a assez. Le propriétaire a retenu la rareté.

## Décision

### Cinq critères, une liste fermée

| Critère | Membres | Récompense par défaut |
|---|---|---|
| Tous les préfixes | les préfixes du fascicule, reprises comprises | 20 gouttes d’encre, 1 sablier |
| Tous les suffixes | les suffixes du fascicule, reprises comprises | 20 gouttes d’encre, 1 sablier |
| Tous les mots communs | ses mots résultats de rareté commune | 20 gouttes d’encre |
| Tous les mots peu communs | ses mots de rareté peu commune | 30 gouttes d’encre, 1 sablier |
| Tous les mots rares | ses mots de rareté rare | 40 gouttes d’encre, 1 sablier |

- Un fascicule déclare ses quêtes de collection dans le catalogue, chacune avec un identifiant de palier, son critère et sa récompense. Un critère au plus une fois par fascicule.
- Les **membres** se calculent à la compilation : jamais une brique légendaire, jamais un mot légendaire ni un mot qui en exige une ([ADR 0043](0043-legendaires-chance-fixe-hors-garanties-et-secretes.md)). Une quête de moins de **3 membres** n’est pas déclarée.
- La rareté d’un mot est celle de l’[ADR 0040](0040-rarete-des-mots.md) : la plus haute rareté des briques de sa recette la plus accessible.

### Progression et récompense

- Le serveur compte les membres au codex du joueur et ne renvoie que les quêtes des fascicules **débloqués** : un fascicule verrouillé n’expose aucun total ([ADR 0069](0069-deblocage-progressif-des-fascicules.md)).
- Une quête complète attend que le joueur la récupère, comme un palier ([ADR 0042](0042-page-quetes-et-recompenses-a-recuperer.md), [ADR 0045](0045-quetes-du-jour-carte-de-lecteur-et-quetes-au-long-cours.md)) : une commande idempotente, une seule fois, refusée tant que la quête est incomplète ou le fascicule verrouillé.
- Aucune liste des cartes manquantes : la quête dit combien, jamais lesquelles.

### Affichage

Dans l’onglet « Fascicules » de la page Quêtes, un bloc « Collection » sous les jalons de chaque fascicule débloqué : l’intitulé, « 4 / 6 », la jauge et la récompense annoncée d’avance. Une quête à récupérer compte dans la pastille de l’onglet.

## Options envisagées

### Toutes les cartes d’une difficulté

Demandé au départ. Écartée : la difficulté n’est jamais affichée (ADR 0047), et elle serait presque vide.

### Une ligne du codex complétée

Écartée : la ligne dépend de la largeur de l’écran.

### Calculer les membres dans l’App

Pas de champ de plus dans le catalogue. Écartée : la règle de rareté et l’exclusion des légendaires vivraient en deux endroits, et le validateur ne pourrait pas refuser une quête trop petite.

## Conséquences

### Positives

- Un objectif fini par fascicule, compréhensible sans explication.
- Aucune table nouvelle : la récupération réutilise celle des paliers.

### Négatives

- Les mots rares avancent lentement, puisqu’il faut plusieurs exemplaires d’une même brique rare : les récompenses sont à simuler avant de publier.
- Trois quêtes trop petites sont écartées des fascicules actuels : mots rares de Rosette et de Boustrophédon, mots communs d’Ogham.

## Critères de réévaluation

- Une quête n’est presque jamais terminée : revoir sa récompense ou la retirer.
- Les joueurs demandent la liste des manquantes : à trancher contre l’ADR 0051, qui vend les indices.
