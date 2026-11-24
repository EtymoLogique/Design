# ADR 0086 — Douze plis en attente, un par heure

- **Statut** : Proposé
- **Remplace partiellement** : [ADR 0020](0020-deux-plis-en-attente-et-sabliers.md), pour la capacité d'énergie (2 charges) et la durée de recharge en production (12 h). Il remplace aussi la décision « dix plis en attente » (ADR 0062), appliquée par l'application mais jamais versée dans ce dossier.
- **Complète** : [ADR 0039](0039-plafonner-l-achat-des-sabliers-pas-leur-usage.md) (prix et absence de plafond des sabliers, inchangés)

## Contexte

L'application compte 10 charges au plus, une toutes les 12 h en production. Le propriétaire veut un rythme plus vif : davantage de plis par session, et un retour qui récompense une visite dans l'heure plutôt qu'une fois par jour.

## Décision

- L'énergie compte **12 charges au plus** : douze plis peuvent attendre le joueur.
- Une charge revient **toutes les heures** en production. La durée reste configurable (`ETYMO_RECHARGE_S`, 3 600 s en production) ; hors production, elle reste plus courte.
- Quand les douze charges sont prêtes, le temps s'arrête ; ouvrir un pli relance la recharge, comme avant.
- Un **sablier** avance toujours la recharge d'un douzième de sa durée : **5 minutes** en production. Douze sabliers valent toujours un pli ; le prix (10 gouttes) ne change pas.
- À la migration, un joueur à 10 / 10 repart d'une recharge en cours (une charge dans 1 h) ; aucune charge en cours n'attend plus d'une heure.

## Options envisagées

### Sablier fixé à une heure

Un sablier aurait valu un pli entier, pour 10 gouttes : l'encre aurait acheté des plis presque à volonté. Écarté par le propriétaire.

### Garder 10 charges et 12 h

Rythme jugé trop lent par le propriétaire.

## Conséquences

### Positives

- Jusqu'à 24 plis par jour pour un joueur assidu ; douze plis attendent celui qui revient après une demi-journée.
- Le ratio « 12 sabliers = 1 pli » et les lots de la boutique (6, 12, 24) restent cohérents.

### Négatives

- Un sablier ne fait gagner que 5 minutes : son intérêt baisse nettement, et l'encre sert d'abord aux indices.
- Le rythme d'obtention des exemplaires est multiplié par 12 environ : le filet, la garantie de nouveauté et l'épuisement des fascicules arrivent plus vite. L'équilibrage des fascicules (ADR 0022, 0069) est à surveiller.
- Un joueur qui vient une fois par jour perd la moitié des charges possibles (12 sur 24).

## Critères de réévaluation

- Un fascicule entièrement collectionné en quelques jours par la majorité des joueurs actifs : rallonger la recharge.
- Des sabliers qui ne s'utilisent presque plus : revoir leur valeur ou leur prix.
