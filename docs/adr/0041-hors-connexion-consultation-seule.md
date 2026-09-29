# ADR 0041 — Hors connexion, la PWA ne sert qu'à consulter

- **Statut** : Accepté
- **Portée** : MVP, dépôt App (PWA) et dépôt design
- **Remplace partiellement** : [ADR 0001](0001-pwa-responsive.md) (préparation d'une fusion et tentative conservée hors connexion), [ADR 0007](0007-etat-et-economie-autoritaires.md) (« opérations en attente rejouables »), [ADR 0024](0024-architecture-logicielle-et-hebergement.md) (la table se prépare hors connexion)

## Contexte

Les ADR 0001, 0007 et 0024 promettaient une préparation de la table hors connexion et une file d'opérations rejouables. Aucun code de file n'existe. Le serveur fait foi pour chaque commande, et le client n'a pas les recettes : une file ne ferait que différer un résultat que le joueur ne peut pas voir.

## Décision

Hors connexion, le joueur **consulte** : codex, réserve, fiches, plis prêts, table déjà saisie (affichée, conservée en local). Il ne peut rien commander : ni pose ou déplacement de brique, ni fusion, ni ouverture de pli, ni sablier, ni indice, ni achat. Aucune file, aucun rejeu côté client. Toute mutation exige le réseau.

Le `409` suivi d'une resynchronisation ([ADR 0024](0024-architecture-logicielle-et-hebergement.md)) reste valable, y compris après la parution d'un fascicule. L'idempotence serveur par clé de commande reste valable. Aucune migration de progression : la resynchronisation suffit.

## Conséquences

- Positives : pas de file à tester ni à réconcilier ; aucun résultat local ambigu.
- Négatives : pas de jeu sans réseau.

## Critères de réévaluation

Les tests montrent que l'attente du réseau gêne le jeu : rouvrir la question, par un nouvel ADR.
