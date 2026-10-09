# ADR 0054 — Fascicules de lancement plus gros et plus rapprochés

- **Statut** : Proposé
- **Remplace partiellement** : [ADR 0022](0022-fascicules-de-20-a-30-mots.md), pour la taille et le rythme des dix premiers fascicules (de 50 à 106 mots, à intervalle hebdomadaire) ; les 20 à 30 mots et le fascicule mensuel restent en vigueur à partir du onzième
- **Complète** : [ADR 0048](0048-beta-ouverte-et-ouverture-a-quatre-fascicules.md) (assez de contenu à l’ouverture)

## Contexte

L’[ADR 0022](0022-fascicules-de-20-a-30-mots.md) fixe de 20 à 30 mots par fascicule et un fascicule par mois. L’[ADR 0048](0048-beta-ouverte-et-ouverture-a-quatre-fascicules.md) montre qu’un nouveau joueur assidu épuise un catalogue trop mince en moins d’un mois, au moment où sa fidélité est la plus fragile.

Le catalogue actuel compte dix fascicules et 718 mots. Chacun publie de 50 à 106 mots. Leurs dates sont hebdomadaires (du 1ᵉʳ octobre au 12 novembre 2026), et les fascicules 1 à 4 paraissent le même jour. Le validateur signale cet écart (`fascicule.taille` et `fascicule.chronologie`, issue Content #12). Ce n’est pas une erreur d’édition : c’est un choix, que cet ADR consigne.

## Décision

- Les **dix premiers fascicules** sont des fascicules de lancement : ils publient plus de 30 mots (de 50 à 106 aujourd’hui) et paraissent à intervalle **hebdomadaire**, au lieu de 20 à 30 mots par mois. Les quatre premiers paraissent ensemble, à l’ouverture.
- Le but est de donner **beaucoup de contenu au lancement** de l’application, pour qu’un joueur assidu ne manque pas de mots pendant les premières semaines.
- À partir du **onzième fascicule**, l’[ADR 0022](0022-fascicules-de-20-a-30-mots.md) s’applique de nouveau : de 20 à 30 mots, environ un fascicule tous les 30 jours.
- Les contrôles `fascicule.taille` et `fascicule.chronologie` du validateur ne s’appliquent pas aux dix fascicules de lancement. Le plafond de 30 mots et l’ordre strict des dates restent contrôlés pour les suivants.

## Options envisagées

### Redécouper le catalogue en fascicules de 20 à 30 mots, un par mois

Écartée : il faudrait environ 28 fascicules pour les 718 mots actuels, soit plus de deux ans de parutions. Le lancement ne disposerait que d’un ou deux fascicules, ce que l’[ADR 0048](0048-beta-ouverte-et-ouverture-a-quatre-fascicules.md) écarte déjà.

### Garder la taille actuelle pour tous les fascicules

Écartée : au-delà de 30 mots, un joueur assidu ne complète plus un fascicule avant la parution du suivant ([ADR 0022](0022-fascicules-de-20-a-30-mots.md)). Le codex devient une dette, et la charge éditoriale d’un fascicule de cent mots n’est pas tenable chaque mois.

## Conséquences

### Positives

- Un joueur qui arrive trouve un catalogue large, et la rétention des premières semaines ne dépend plus d’un seul fascicule.
- Le catalogue existant est conservé tel quel, sans redécoupage.
- Le retour à 20 à 30 mots par mois garde la charge éditoriale de l’[ADR 0022](0022-fascicules-de-20-a-30-mots.md) à long terme.

### Négatives

- Un joueur occasionnel ne complétera pas les fascicules de lancement : leurs plis restent ouverts, sans pénalité ([ADR 0016](0016-plis-et-jaquettes-par-fascicule.md)).
- Le rythme chute après le dixième fascicule : le contenu suivant doit être prêt, ou la bascule se verra.
- Le validateur du dépôt Content doit distinguer les fascicules de lancement des autres.

## Critères de réévaluation

- Les joueurs assidus n’achèvent pas les fascicules de lancement avant la parution du suivant : espacer les dates ou réduire la taille.
- La chute de rythme au onzième fascicule fait décrocher les joueurs : étendre la fenêtre de lancement ou relever le plafond ordinaire.
- Une simulation sur les fascicules de lancement contredit l’estimation de l’[ADR 0048](0048-beta-ouverte-et-ouverture-a-quatre-fascicules.md).
