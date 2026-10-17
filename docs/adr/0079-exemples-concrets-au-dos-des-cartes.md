# ADR 0079 — Exemples concrets au dos des cartes

- **Statut** : Proposé
- **Portée** : modèle de données du catalogue (sens) et verso de la carte agrandie (I-15)
- **Complète** : [ADR 0013](0013-schema-des-briques.md) (schéma des briques), [ADR 0027](0027-catalogue-statique-et-donnees-joueur.md) (artefact), [ADR 0060](0060-wikidata-seule-source-de-verite.md) (aucun texte copié)

## Contexte

Une définition dit ce qu’un mot signifie, pas toujours comment se le représenter. *Polyphone* (« qui fait entendre plusieurs voix ou plusieurs sons à la fois ») reste abstrait tant qu’on n’a pas pensé à un chœur, à un orgue ou à une sonnerie de téléphone. Le joueur qui vient de découvrir un tel mot n’a ni image ni scène à retenir, et le mot s’oublie vite.

Le verso d’une carte trouvée propose aujourd’hui deux contenus : les formes et « En savoir plus » (relation historique, transformations, certitude, sources). Aucun ne donne d’exemple.

## Décision

### Les exemples vivent sur le sens

- Un **sens** peut porter, par locale, une liste d’**exemples** : de 1 à 5 phrases courtes, distinctes, qui décrivent une situation, un objet ou une scène où le sens se voit. Le champ est facultatif ; seuls les mots difficiles à se représenter en reçoivent.
- Les exemples sont des textes d’édition rédigés, jamais copiés ([ADR 0060](0060-wikidata-seule-source-de-verite.md)). Ils partagent le fait de leur sens : même confiance, même relecture, mêmes sources.
- Ils sont compilés dans la **ressource de la carte** ([ADR 0027](0027-catalogue-statique-et-donnees-joueur.md)) et publiés avec leur fascicule, pas avant. La base de progression n’en copie rien.

### Un onglet « Exemples » au verso

- Le verso d’une carte **trouvée** affiche un onglet « Exemples » entre « Formes » et « En savoir plus », seulement si l’un de ses sens porte des exemples. Sans exemple, le verso reste identique à aujourd’hui.
- Une carte inconnue n’affiche rien : les exemples racontent le mot et ne servent jamais de piste.
- L’onglet n’entre pas dans l’adresse : « En savoir plus » reste le seul volet adressable (`plus=1`).
- Le jeu n’en tire aucune règle : ni fusion, ni indice, ni quête ne dépend d’un exemple.

## Options envisagées

### Un champ sur l’unité

Plus simple à saisir. Écartée : un exemple illustre un sens, et un mot polysémique a un exemple par acception.

### Fondre les exemples dans « En savoir plus »

Pas d’onglet de plus. Écartée : « En savoir plus » est déjà long (chemin, relations, transformations, sources) et ne parle pas du sens ; l’exemple doit se trouver d’un geste.

### Une image par mot

Plus parlante qu’une phrase. Écartée pour le MVP : il faudrait dessiner et valider un média par mot ([ADR 0023](0023-textures-des-cartes.md)), alors qu’une phrase se relit comme le reste.

## Conséquences

### Positives

- Un mot abstrait devient une scène à retenir, sans nouvelle règle de jeu.
- Aucune nouvelle table : le champ est facultatif et le schéma reste rétrocompatible.

### Négatives

- Un texte de plus à rédiger et à relire pour chaque mot concerné.
- Un onglet de plus au verso, donc une navigation de plus à tenir au clavier.

## Critères de réévaluation

- Les joueurs n’ouvrent pas l’onglet : fondre les exemples dans « Formes » ou les retirer de l’affichage, pas du catalogue.
- Des exemples par image ou par audio deviennent souhaitables : nouvel ADR sur les médias.
