# ADR 0048 — Bêta ouverte, puis ouverture officielle à quatre fascicules

- **Statut** : Proposé
- **Complète** : [ADR 0028](0028-gitflow-des-depots.md) (la bêta tourne en production) ; [ADR 0022](0022-fascicules-de-20-a-30-mots.md) (un fascicule par mois) ; le [plan financier](../plan-financier.md) (calendrier et budget de lancement)

## Contexte

Le plan financier prévoyait une mise en production en janvier 2027, avec le fascicule 1 et le lancement presse le même mois. Un nouveau joueur n’aurait alors trouvé que 20 à 30 mots : un joueur assidu les épuise en moins d’un mois ([ADR 0022](0022-fascicules-de-20-a-30-mots.md)), au moment même où sa fidélité est la plus fragile.

Il faut à la fois une masse de contenu à l’ouverture officielle, et une épreuve du jeu en conditions réelles avant de dépenser pour faire venir des joueurs.

## Décision

### Une bêta ouverte

- La bêta est **ouverte à tous**. Elle tourne en production, déployée par un tag `v*.*.*` ([ADR 0028](0028-gitflow-des-depots.md)), et l’interface la signale par la mention « bêta ». L’environnement hors-prod reste réservé aux essais.
- Les fascicules paraissent pendant la bêta, au rythme habituel.
- Les joueurs de la bêta **gardent toute leur progression** à l’ouverture officielle.
- La mesure de la rétention ([ADR 0050](0050-cohortes-de-retention-et-statistiques-du-joueur.md)) est en place dès le premier jour de la bêta.

### La taille critique avant l’ouverture officielle

- **Critère** : à l’ouverture, un nouveau joueur assidu ne doit pas pouvoir épuiser le catalogue en un mois.
- Estimation de ce qu’il trouve son premier mois, à confirmer par simulation : 55 à 60 mots. Environ 34 grâce aux plis (75 plis par mois, [ADR 0022](0022-fascicules-de-20-a-30-mots.md)), une quinzaine grâce aux jalons, au démarrage généreux ([ADR 0046](0046-demarrage-genereux-cadeau-de-bienvenue-et-quetes-initiales.md)) et à la carte de lecteur ([ADR 0045](0045-quetes-du-jour-carte-de-lecteur-et-quetes-au-long-cours.md)), et les découvertes immédiates permises par les graines des fascicules ([ADR 0030](0030-reserve-de-depart-par-fascicule.md)).

| Fascicules à l’ouverture | 2 | 3 | 4 |
|---|---|---|---|
| Mots disponibles, hors légendaires | environ 50 | environ 75 | environ 100 |
| Marge d’un joueur assidu le premier mois | aucune : il manque de mots avant le fascicule suivant | 15 à 20 mots | environ 40 mots |

- **Minimum : 3 fascicules ; cible : 4**, soit environ 100 mots. Avec la règle des tiers ([ADR 0047](0047-un-tiers-facile-un-tiers-moyen-un-tiers-difficile.md)), 3 fascicules offrent déjà environ 25 mots faciles.
- L’ouverture officielle attend aussi que la bêta atteigne ses seuils, fixés avant son ouverture. Valeurs proposées : au moins 80 % de tutoriels terminés, au moins 25 % de joueurs revenus au 7ᵉ jour, et moins de 10 % de joueurs actifs restés plus de deux jours sans découverte possible.

### Calendrier

- **Janvier 2027** : la bêta ouvre avec les fascicules 1 et 2, tous deux prêts d’après le plan financier.
- **Février 2027** : fascicule 3.
- **Mars 2027** : fascicule 4 et ouverture officielle, pendant la Semaine de la langue française et de la Francophonie, le temps fort que recommande l’[étude de marché](../etude-de-marche.md).
- Aucun coût éditorial de plus, mais l’avance de production tombe de deux fascicules à un. Le budget de lancement (achat média, presse et créateurs de contenu) passe de janvier à mars.

## Options envisagées

### Une bêta fermée

Écartée : 50 à 150 testeurs ne disent rien de l’arrivée réelle des joueurs, et la bêta ouverte prépare déjà l’audience de l’ouverture.

### Ouvrir officiellement avec deux fascicules

Écartée : un joueur assidu manquerait de mots avant la fin de son premier mois.

### Lancer en janvier, comme le prévoyait le plan

Écartée : un seul fascicule à l’arrivée des joueurs de la presse, et aucune mesure avant de dépenser pour eux.

### Produire plus vite pour ouvrir plus tôt

Écartée : deux fascicules par mois doublent la charge éditoriale, environ un mot sourcé par jour aujourd’hui ([ADR 0014](0014-sources-et-fascicules.md)).

## Conséquences

### Positives

- À l’ouverture, le catalogue tient un joueur assidu plus d’un mois, et chaque fascicule offre ses entrées faciles.
- Le jeu est éprouvé en conditions réelles avant les dépenses de lancement.
- L’ouverture tombe sur le temps fort annuel de la langue française.

### Négatives

- Les défauts de la bêta se voient en production, par de vrais joueurs. Les paramètres d’équilibrage peuvent changer en cours de route, puisqu’ils sont versionnés ([ADR 0009](0009-contenu-et-equilibrage.md)).
- Avec un seul fascicule d’avance, un mois de retard éditorial ne s’absorbe plus.
- Le profil invité ([ADR 0007](0007-etat-et-economie-autoritaires.md)) peut se perdre si le navigateur efface son stockage : un joueur de la bêta perdrait alors sa progression. Les comptes viendront plus tard.
- Le plan financier change de calendrier : moins de nouveaux joueurs en janvier, davantage en mars.

## Critères de réévaluation

- La bêta n’atteint pas ses seuils : reporter l’ouverture officielle plutôt que dépenser pour faire venir des joueurs.
- Les joueurs consomment le catalogue nettement plus vite que l’estimation : viser plus de quatre fascicules à l’ouverture.
- Un retard éditorial menace la parution mensuelle pendant la bêta.
