# ADR 0049 — La date du prochain fascicule est publique

- **Statut** : Proposé
- **Remplace partiellement** : [ADR 0029](0029-parution-par-promotion-de-prefixe.md), pour ce que l’index dit d’un fascicule à venir : sa date de parution, et rien d’autre ; le calendrier `staging/parutions.json` reste illisible
- **Complète** : [ADR 0014](0014-sources-et-fascicules.md) (un fascicule ne dévoile pas son contenu)

## Contexte

Le fascicule mensuel est le rendez-vous du jeu. Pourtant, le joueur ne sait pas quand paraît le suivant : le calendrier des parutions reste illisible dans `staging/` ([ADR 0029](0029-parution-par-promotion-de-prefixe.md)), et l’index public ne cite que les fascicules parus. Un joueur qui a complété son fascicule ne sait pas s’il doit attendre deux jours ou deux semaines.

## Décision

- L’index global publie un champ **`prochaine_parution`** : la date et l’heure, en UTC, du prochain fascicule du calendrier. Rien d’autre de ce fascicule : ni numéro, ni nom, ni jaquette, ni nombre de mots.
- La tâche de parution l’écrit avec l’index, à partir du calendrier, à chaque passage. Un changement du calendrier la relance, pour que la date publiée reste juste.
- La PWA affiche cette date sur le choix du fascicule de l’écran des plis et en tête du codex : « Prochain fascicule : 1ᵉʳ novembre ».
- Sans fascicule au calendrier, le champ est absent et rien ne s’affiche.
- Un report change simplement la date affichée.

## Options envisagées

### Ne rien publier avant la parution

Écartée : le joueur ne sait pas quand revenir, alors que la date ne trahit aucun contenu.

### Montrer la jaquette en avant-première

Écartée : la jaquette ne trahit rien par construction ([ADR 0016](0016-plis-et-jaquettes-par-fascicule.md)), mais aucun fichier d’un fascicule à venir ne doit se lire avant sa parution ([ADR 0029](0029-parution-par-promotion-de-prefixe.md)).

### Annoncer, à la parution, combien de familles du joueur le fascicule prolonge

Écartée : l’annonce d’une parution s’en tient aux nombres de mots, de préfixes, de suffixes et de langues ([ADR 0014](0014-sources-et-fascicules.md)).

## Conséquences

### Positives

- Le joueur sait quand revenir, et l’attente d’un fascicule devient un rendez-vous.
- Aucun contenu à venir n’est lisible : seule une date sort du calendrier.

### Négatives

- Une date publique engage : un retard se voit.
- La tâche de parution (dépôt App) et le format de l’index changent ; le client ne doit rien déduire d’autre de ce champ.

## Critères de réévaluation

- Des reports fréquents rendent la date peu fiable.
- Des joueurs déduisent quelque chose du contenu à venir à partir de la date.
