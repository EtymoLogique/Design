# ADR 0038 — Indices à prix croissant et format des jalons

- **Statut** : Accepté, partiellement remplacé par l'[ADR 0039](0039-plafonner-l-achat-des-sabliers-pas-leur-usage.md) : les sabliers de jalon s'ajoutent sans plafond de détention, aucun n'est perdu. Le reste du présent ADR reste en vigueur.
- **Remplace partiellement** : [ADR 0020](0020-deux-plis-en-attente-et-sabliers.md), pour le prix de l’indice ; le reste de l’ADR 0020 reste en vigueur
- **Complète** : [ADR 0012](0012-briques-rationnees.md) et [ADR 0004](0004-progression-atteignable.md), pour le format des jalons d’un fascicule ; [ADR 0018](0018-sabliers-et-boutique.md), pour les sabliers donnés par un jalon

## Contexte

L’ADR 0020 fixe un indice à 30 gouttes, sans distinguer ses cinq niveaux : famille ou langue du résultat, nature des deux briques, présence d’une transformation, emplacement d’une brique connue, solution. Au même prix, le joueur n’a aucune raison de prendre un indice vague plutôt que la solution, et la déduction perd son intérêt.

Les jalons, eux, sont annoncés depuis l’ADR 0012 : ils offrent un pli au contenu déterministe, et certains donnent des sabliers. Leur format n’a jamais été défini, et le manifeste d’un fascicule publie une liste de jalons toujours vide. Le jalon J4 de la roadmap de l’App les livre avec les sabliers et les indices.

## Décision

### Prix des indices

| Niveau | Contenu | Prix |
|---|---|---|
| 1 | Famille ou langue du résultat | 5 gouttes |
| 2 | Nature des deux briques (préfixe, suffixe ou mot) | 10 gouttes |
| 3 | Présence d’une transformation | 20 gouttes |
| 4 | Emplacement d’une brique détenue | 40 gouttes |
| 5 | Solution | 80 gouttes |

- Le prix **double à chaque niveau** : 155 gouttes pour les cinq niveaux d’une recette.
- Les niveaux 1 à 3 coûtent 35 gouttes ensemble, près des 30 gouttes de l’ADR 0020, soit environ deux plis et demi d’encre.
- La solution seule coûte environ 13 plis d’encre, près d’une semaine de jeu gratuit : elle reste un dernier recours.
- Un indice porte sur **une recette encore à découvrir**, choisie parmi les silhouettes du codex. Une recette légendaire, cachée par l’[ADR 0015](0015-codex-fascicules-et-legendaires.md), n’en reçoit pas.
- Les niveaux s’achètent **dans l’ordre**, un à la fois, et chacun ne se paie **qu’une fois**. Un niveau acquis reste lisible.
- Le barème est un paramètre d’équilibrage ([ADR 0009](0009-contenu-et-equilibrage.md)). Les niveaux gratuits, débloqués par les essais, le temps ou les objectifs, restent prévus mais ne sont pas dans le MVP.

### Format des jalons

Un fascicule déclare ses jalons dans son fichier éditorial, et le manifeste publié les reprend tels quels :

| Champ | Rôle |
|---|---|
| `id` | Identifiant opaque `jal_…`, unique. |
| `decouvertes` | Nombre de mots de ce fascicule découverts qui atteint le jalon. Strictement positif et strictement croissant d’un jalon au suivant. |
| `pli` | Briques du pli de jalon, déterministe : un exemplaire de chacune. Préfixes et suffixes du fascicule seulement, comme le pli tiré ([ADR 0035](0035-plis-d-affixes-ponderes-par-la-rarete.md)). |
| `sabliers` | Sabliers donnés, 0 ou plus. |

- Un jalon a **une seule condition** : un nombre de découvertes dans le fascicule. Les autres objectifs (famille, langues, transformations) viendront plus tard.
- Un jalon donne un pli, des sabliers, ou les deux ; jamais rien.
- Le serveur accorde un jalon **une seule fois**, dans la transaction de la découverte qui l’atteint, ou à la lecture de l’état si un jalon est publié après coup.
- Le pli de jalon suit les règles du pli ([ADR 0012](0012-briques-rationnees.md)) : une brique dont la réserve est pleine se change en encre. Il ne consomme pas d’énergie et ne compte ni dans la garantie de nouveauté, ni dans le filet.
- Les sabliers donnés s’ajoutent à ceux détenus, dans la limite de 36 ([ADR 0018](0018-sabliers-et-boutique.md)). Au-delà, ils sont perdus pour l’instant.
- La récompense est **affichée à l’avance** dans l’onglet « Objectifs » du codex : un jalon n’est jamais un tirage.
- L’intégration des jalons au point fixe d’atteignabilité ([ADR 0004](0004-progression-atteignable.md)) viendra avec l’étape 5 du pipeline.

## Options envisagées

### Garder un prix unique de 30 gouttes

Écartée : la solution coûterait autant que la famille du résultat, et un joueur bloqué irait droit à la solution.

### Prix linéaire (10, 20, 30, 40, 50 gouttes)

Écartée : 150 gouttes au total, mais les premiers niveaux coûtent plus cher qu’aujourd’hui et la solution ne se distingue pas assez des niveaux qui la précèdent.

### Conditions de jalon libres (familles, langues, transformations)

Écartée pour le MVP : elles demandent un langage de conditions et leur validation. Le nombre de découvertes suffit à rythmer un fascicule.

### Récompense de jalon tirée au hasard

Écartée : l’ADR 0012 veut un pli de jalon déterministe, déclaré dans le graphe d’obtention, pour garantir les chemins critiques.

## Conséquences

### Positives

- Le joueur paie selon ce qu’il apprend : un indice vague reste abordable, la solution coûte cher.
- Les jalons ont un format vérifiable par le validateur et lisible par le serveur, sans nouveau langage.
- Les récompenses des jalons s’annoncent à l’avance, sans hasard.

### Négatives

- Les cinq niveaux d’une recette coûtent 155 gouttes, cinq fois plus qu’un indice de l’ADR 0020.
- Des sabliers de jalon au-delà de 36 détenus sont perdus tant qu’aucun compteur en attente n’existe.
- Un jalon n’est pas encore pris en compte par la validation d’atteignabilité.

## Critères de réévaluation

- Des joueurs bloqués qui n’achètent jamais le premier niveau : baisser le barème.
- Des joueurs qui achètent la solution sans passer par les niveaux 1 à 3 : relever l’écart.
- Un fascicule qui a besoin d’un objectif autre que le nombre de découvertes.
- Un jalon qui donne assez de sabliers pour dépasser régulièrement 36 détenus.
