# ADR 0045 — Quêtes du jour, carte de lecteur et quêtes au long cours

- **Statut** : Proposé
- **Remplace partiellement** : [ADR 0042](0042-page-quetes-et-recompenses-a-recuperer.md), pour le contenu de l’onglet « Journalières », jusqu’ici annoncé « bientôt disponible », et pour le nombre d’onglets de la page Quêtes, qui passe à trois ; le reste de l’ADR 0042, dont la récupération des récompenses, reste en vigueur ; [ADR 0039](0039-plafonner-l-achat-des-sabliers-pas-leur-usage.md), pour la seule source d’encre : la carte de lecteur et les quêtes au long cours en donnent aussi, en quantité bornée
- **Complète** : [ADR 0038](0038-indices-a-prix-croissant-et-format-des-jalons.md) (d’autres conditions que le nombre de découvertes, pour les seules quêtes au long cours ; les jalons ne changent pas) ; [ADR 0044](0044-ricochets-reviser-et-maitriser-les-cartes.md) (les ricochets du jour)

## Contexte

L’[ADR 0042](0042-page-quetes-et-recompenses-a-recuperer.md) a créé la page Quêtes avec un onglet « Journalières » vide, dont le contenu attendait un ADR. Le game design évoque aussi un défi quotidien construit à partir des briques possédées.

Deux moments menacent la fidélité du joueur : le jour où il n’a rien de particulier à faire, et la fin du mois, quand son fascicule est complet. Il lui faut une raison de revenir chaque jour, et des objectifs qui durent au-delà d’un fascicule.

La boussole refuse la pression et la peur de manquer : une récompense gagnée n’expire jamais ([ADR 0042](0042-page-quetes-et-recompenses-a-recuperer.md)), le design système exclut la culpabilisation, et se tromper ne coûte rien.

## Décision

### Trois quêtes par jour

- Chaque jour, trois **quêtes du jour**, une de chaque sorte :
  - **table** : une déduction à la table de fusion ;
  - **mémoire** : des ricochets ([ADR 0044](0044-ricochets-reviser-et-maitriser-les-cartes.md)) ;
  - **adresse** : découper des mots ou relier des paires.
- Elles se renouvellent à **minuit GMT**, la même heure pour tous. Une quête non faite disparaît au renouvellement, sans pénalité.
- Le serveur les tire à la première visite après minuit, **seulement parmi celles que l’état du joueur rend faisables** ce jour-là. Si la réserve ne permet aucune découverte, la quête de table cède sa place à une seconde quête de mémoire. Aucune quête ne dépend de l’énergie ni du hasard.
- Chaque quête demande un acte de réflexion : déduire, se souvenir, découper. Jamais un simple geste, comme retourner une carte ou ouvrir un pli.
- Elles portent sur les cartes à revoir et sur les découvertes récentes, jamais sur un mot légendaire ([ADR 0043](0043-legendaires-chance-fixe-hors-garanties-et-secretes.md)).
- Un échec ne coûte rien : un sans-faute manqué se recommence aussitôt, avec d’autres cartes.
- Le **défi du jour** du game design devient la quête de table : une recette que la réserve permet, donnée par sa seule piste de sens.

| Sorte | Exemple | Proposée seulement si… |
|---|---|---|
| Table | « Trouvez le mot qui veut dire “peur de l’eau” » | la réserve permet cette découverte |
| Table | « Découvrez un mot venu du grec ancien » | la réserve permet une telle découverte |
| Mémoire | « Cinq ricochets sans faute » | toujours |
| Mémoire | « Retrouvez trois mots par leur sens littéral » | au moins trois mots trouvés |
| Mémoire | « Sens littéral ou actuel : quatre bonnes réponses » | au moins deux mots trouvés |
| Mémoire | « Maîtrisez une carte » | une carte n’est plus qu’à un ricochet de la maîtrise |
| Adresse | « Découpez trois mots sans erreur » | au moins trois mots trouvés |
| Adresse | « Paires sans faute » | au moins quatre briques connues |

### La carte de lecteur

- Quand ses trois quêtes du jour sont faites, le joueur gagne un **tampon** sur sa **carte de lecteur**, tamponnée comme une fiche de prêt de bibliothèque. Un tampon par jour au plus.
- Les jours n’ont pas besoin de se suivre : aucune série ne se brise, rien n’expire, la carte ne repart jamais à zéro.
- Au **7ᵉ tampon**, une récompense annoncée d’avance, avec ces valeurs de départ, à simuler : 20 gouttes d’encre, 1 sablier et 1 **pli de lecteur**. Elle attend dans la page Quêtes jusqu’à ce que le joueur la récupère (« Récupérer », [ADR 0042](0042-page-quetes-et-recompenses-a-recuperer.md)), et une nouvelle carte commence.
- Le **pli de lecteur** : le joueur choisit le fascicule ; le pli tire parmi ses briques peu communes et rares, selon leurs poids, et les légendaires y gardent leur chance de 3 % ([ADR 0043](0043-legendaires-chance-fixe-hors-garanties-et-secretes.md)). Ses chances sont affichées. Il ne consomme pas d’énergie et ne compte pour aucune garantie, comme un pli de jalon.
- Pas de flamme qui s’éteint : la carte « 4 / 7 » est le seul signe de régularité. Aucune notification ni aucun rappel ne la concernent.

### Les quêtes au long cours

- Un troisième onglet de la page Quêtes, **« Au long cours »** (`#/quetes/au-long-cours`), réunit des objectifs qui traversent les fascicules. Ils n’expirent jamais et ne repartent jamais à zéro.
- Chaque quête a des **paliers** ; chaque palier a sa récompense, affichée d’avance, à récupérer comme un jalon.
- Cinq sortes de quêtes, toutes mesurées sur le codex du joueur :

| Quête | Paliers (exemple) | Ce qui est compté |
|---|---|---|
| Langues | 10, 25, 50 formes trouvées, une quête par langue | les formes de chaque langue, que sa carte compte déjà |
| Familles | 3 familles de 5 mots, puis 1 famille de 10 | les mots formés sur une même brique (familles déduites des recettes) |
| Transformations | un mot de chaque type, puis trois de chaque | élision, voyelle de liaison, allomorphie, adaptation de graphie |
| Ricochets | 10, 50, 150 cartes maîtrisées ; un fascicule entièrement maîtrisé | la maîtrise ([ADR 0044](0044-ricochets-reviser-et-maitriser-les-cartes.md)) |
| Lecteur | 1, 5, 20 cartes de lecteur remplies | les tampons |

- Récompenses : encre, sabliers et plis de lecteur, de plus en plus généreux avec les paliers. Les paliers se règlent pour qu’un joueur assidu reçoive environ une récompense par semaine.
- Un mot légendaire n’entre dans aucun décompte, et aucune quête ne fixe un nombre de légendaires à trouver, qui trahirait combien il en existe. Seule exception permise : « Trouvez votre première légendaire », puisque chaque fascicule en a au moins une ([ADR 0043](0043-legendaires-chance-fixe-hors-garanties-et-secretes.md)).
- Les conditions forment une **liste fermée** : langue, famille, transformation, maîtrise, carte de lecteur. Les quêtes sont déclarées dans le catalogue et contrôlées par le validateur ; le serveur calcule la progression. Il n’y a pas de langage de conditions libre. Les jalons restent définis par le nombre de découvertes ([ADR 0038](0038-indices-a-prix-croissant-et-format-des-jalons.md)).

### Récupérer et faire foi

- Une récompense de tampon ou de palier se récupère par une commande idempotente et transactionnelle, comme un jalon ([ADR 0042](0042-page-quetes-et-recompenses-a-recuperer.md)). Elle n’expire jamais.
- Quêtes du jour, tampons et paliers sont des données du joueur, sous autorité du serveur ([ADR 0007](0007-etat-et-economie-autoritaires.md)). Les définitions viennent du catalogue : quêtes du jour, récompense de la carte de lecteur et quêtes au long cours sont des paramètres d’équilibrage ([ADR 0009](0009-contenu-et-equilibrage.md)).

## Options envisagées

### Des quêtes faites de simples gestes

Écartée : « Retournez une carte » ou « Ouvrez un pli » ne demandent aucune réflexion et deviennent des corvées.

### Des quêtes qui dépendent de l’énergie ou du hasard

Écartée : un joueur qui a ouvert ses deux plis à 23 h n’en a plus avant 11 h le lendemain, et un tirage ne se commande pas.

### Une série de jours consécutifs, ou flamme

Écartée : une série qui se brise culpabilise, ce que refuse le design système. La carte de lecteur récompense la régularité sans punir l’absence.

### Un tampon pour une seule quête, comme dans Pokémon GO

Écartée : les trois quêtes, courtes, font ensemble le tampon du jour.

### Une première carte de lecteur à trois cases

Écartée : la première semaine a ses propres récompenses ([ADR 0046](0046-demarrage-genereux-cadeau-de-bienvenue-et-quetes-initiales.md)).

### Renouveler les quêtes à minuit, heure locale

Écartée : une seule heure pour tous, minuit GMT, reste simple à calculer et à expliquer.

### Un langage de conditions libre

Écartée : il faudrait l’écrire, le valider et l’expliquer. Cinq conditions couvrent les besoins.

## Conséquences

### Positives

- Une raison de revenir chaque jour, sans pression : rien ne se perd si l’on manque un jour.
- Des objectifs qui durent quand le fascicule du mois est complet.
- Aucun coût éditorial : quêtes et paliers se mesurent sur le contenu publié.

### Négatives

- La carte de lecteur ajoute au plus quatre plis par mois, environ 5 % de plus pour un joueur assidu, auxquels s’ajoutent les paliers : le rythme des fascicules est à resimuler ([ADR 0022](0022-fascicules-de-20-a-30-mots.md)).
- Le serveur doit tirer des quêtes faisables et vérifier leur réalisation.
- Minuit GMT tombe à 1 h ou 2 h du matin en France, et en soirée au Québec.
- La page Quêtes, avec ses trois onglets, et la carte de lecteur restent à dessiner.

## Critères de réévaluation

- Une faible part des joueurs actifs termine ses trois quêtes : les quêtes sont trop longues ou trop difficiles.
- Les tests montrent que les quêtes sont vécues comme une corvée.
- Les paliers au long cours récompensent beaucoup plus ou beaucoup moins d’une fois par semaine.
