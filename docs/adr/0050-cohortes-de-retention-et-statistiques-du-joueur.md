# ADR 0050 — Cohortes de rétention et statistiques du joueur, sans donnée personnelle

- **Statut** : Proposé
- **Complète** : [ADR 0010](0010-observabilite-et-vie-privee.md) (observabilité respectueuse de la vie privée) ; [ADR 0048](0048-beta-ouverte-et-ouverture-a-quatre-fascicules.md) (les seuils de la bêta)

## Contexte

Le plan financier raisonne en cohortes mensuelles : 30 % des joueurs d’un mois encore actifs le mois suivant, 10 % au 8ᵉ mois. La bêta ouverte ([ADR 0048](0048-beta-ouverte-et-ouverture-a-quatre-fascicules.md)) se juge sur des seuils de rétention. Or la télémétrie prévue renouvelle son identifiant pseudonyme tous les 30 jours : une cohorte ne se suit pas au-delà d’un mois, ce que l’[ADR 0010](0010-observabilite-et-vie-privee.md) notait déjà (« identifiants limités moins adaptés aux cohortes longues »).

Le joueur, de son côté, ne voit sa progression que dans le codex, fascicule par fascicule. Rien ne lui montre le chemin parcouru d’un mois à l’autre.

## Décision

### Des cohortes sans donnée nouvelle

- **Rétention mensuelle** : la table `joueur` contient déjà `cree_le` et `derniere_activite`. Une tâche mensuelle compte, pour chaque mois d’arrivée, les joueurs actifs pendant le mois qui s’achève, et ne garde que ces nombres.
- **Retour au 1ᵉʳ, au 7ᵉ et au 30ᵉ jour** : une tâche quotidienne fait le même calcul, jour par jour, pour les profils créés 1, 7 et 30 jours plus tôt.
- **Passage d’un mois au suivant** : une colonne de plus, `mois_activite_precedente`, mise à jour quand l’activité d’un joueur change de mois. La tâche mensuelle compte les joueurs actifs du mois dont l’activité précédente date du mois d’avant.
- **Télémétrie** : chaque événement porte le mois d’arrivée du joueur, comme une propriété énumérée. L’identifiant pseudonyme se renouvelle au début de chaque mois civil plutôt que tous les 30 jours : un joueur a exactement un identifiant par mois.
- **Seuil** : aucun agrégat de moins de 50 joueurs n’est affiché ni exporté. Les durées de conservation restent documentées ([ADR 0010](0010-observabilite-et-vie-privee.md)).

Indicateurs suivis, chacun justifié comme le demande l’ADR 0010 :

| Indicateur | Pour décider de |
|---|---|
| Retour au 1ᵉʳ, au 7ᵉ et au 30ᵉ jour | l’ouverture officielle ([ADR 0048](0048-beta-ouverte-et-ouverture-a-quatre-fascicules.md)), le démarrage généreux ([ADR 0046](0046-demarrage-genereux-cadeau-de-bienvenue-et-quetes-initiales.md)) |
| Rétention mensuelle par cohorte, passage d’un mois au suivant | les hypothèses du plan financier |
| Jours joués par mois | le rythme des plis et des quêtes |
| Joueurs actifs restés plus de deux jours sans découverte possible | le filet d’utilité ([ADR 0012](0012-briques-rationnees.md)) |
| Tampons par semaine, quêtes du jour terminées, séances de ricochets | les quêtes et les ricochets ([ADR 0044](0044-ricochets-reviser-et-maitriser-les-cartes.md), [ADR 0045](0045-quetes-du-jour-carte-de-lecteur-et-quetes-au-long-cours.md)) |

### Les statistiques du joueur

- Un onglet **« Cabinet »** du codex, fusionné avec les objectifs (familles), montre en tête au joueur ses propres nombres, jamais comparés à ceux des autres :
  - jours de jeu ;
  - mots trouvés ;
  - langues rencontrées ;
  - mot au plus long voyage, celui dont la filiation traverse le plus de langues ;
  - cartes maîtrisées ([ADR 0044](0044-ricochets-reviser-et-maitriser-les-cartes.md)) ;
  - légendaires trouvées ([ADR 0043](0043-legendaires-chance-fixe-hors-garanties-et-secretes.md)) ;
  - cartes de lecteur remplies ([ADR 0045](0045-quetes-du-jour-carte-de-lecteur-et-quetes-au-long-cours.md)).
- Une donnée de plus dans `joueur` : `jours_de_jeu`, le nombre de jours distincts où le joueur est venu.

## Options envisagées

### Un identifiant durable pour la télémétrie

Écartée : l’ADR 0010 limite la portée des identifiants, et les tâches d’agrégation donnent les cohortes sans suivre personne.

### Garder la rotation de l’identifiant tous les 30 jours

Écartée : à cheval sur deux mois, un même joueur compterait deux fois, et aucune cohorte ne se suivrait au-delà d’un mois.

### Montrer au joueur des statistiques collectives

Écartée lors de la revue des pistes de rétention d’octobre 2026 : pas de nombres comme « 12 % des joueurs ont trouvé ce mot ».

### Donner des titres au joueur

Écartée lors de la même revue. Des titres tirés du jeu (*logophile*, *philologue*) dévoileraient en outre des mots du catalogue.

### Une carte des routes des mots dans « Votre cabinet »

Déplacée : elle rejoint une évolution dédiée, avec la vue Filiation du codex ([potentiel d’évolution](../potentiel-evolution.md)).

## Conséquences

### Positives

- Les hypothèses du plan financier deviennent mesurables dès la bêta.
- Aucun identifiant durable, et les agrégats ne gardent rien d’individuel.
- Le joueur voit le chemin parcouru d’un mois à l’autre.

### Négatives

- Deux colonnes de plus dans `joueur`, deux tâches planifiées et une table d’agrégats à purger selon les durées de conservation.
- Les petites cohortes ne s’affichent pas : les premières semaines de la bêta donnent peu de chiffres.
- La télémétrie prévue au jalon J7 de la roadmap de l’App change : renouvellement au mois civil et mois d’arrivée.

## Critères de réévaluation

- Une analyse nécessaire à une décision s’avère impossible avec ces agrégats.
- Un agrégat permet de distinguer un petit groupe de joueurs.
- Peu de joueurs ouvrent « Votre cabinet ».
