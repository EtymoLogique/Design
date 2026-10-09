# ADR 0058 — Langues d’emprunt, toujours sans variété

- **Statut** : Proposé
- **Remplace partiellement** : [ADR 0013](0013-schema-des-briques.md), pour le nombre de langues du MVP (trois) ; l’absence de variété reste en vigueur
- **Complète** : [ADR 0006](0006-architecture-multilingue.md) (langues et écritures), [ADR 0023](0023-textures-des-cartes.md) (silhouette de chaque langue)

## Contexte

L’[ADR 0013](0013-schema-des-briques.md) fixe trois langues pour le MVP : grec ancien, latin et français. Il prévoit de se réévaluer quand un contenu réel l’exige.

C’est le cas. La relecture des chaînes étymologiques des fascicules 1 à 4 dans le TLFi (issue Content #14) montre des mots qui ne viennent ni du grec ni du latin par la voie directe : *biologie* et *écologie* sont empruntés à l’allemand, *baromètre*, *microphone* et *télépathie* à l’anglais, *télescope* passe sans doute par l’italien. Sans ces langues, la chaîne s’arrête au mauvais endroit, ou elle ment.

Le TLFi distingue aussi des variétés : bas latin, latin médiéval, latin scientifique, ancien français. L’[ADR 0013](0013-schema-des-briques.md) les a écartées pour ne pas fragmenter le codex en cartes de langue peu remplies.

## Décision

### Une langue d’emprunt naît d’un mot publié

- Le catalogue peut déclarer d’autres langues que les trois du MVP, quand **une chaîne étymologique d’un mot publié** passe par elles, d’après une source de référence.
- Aucune langue n’est ajoutée « pour plus tard » : une langue sans unité ne sert à rien et coûte une silhouette.
- Les premières sont l’**anglais**, l’**allemand** et l’**italien**. L’arabe, le persan ou le francique viendront avec le premier mot publié qui les demande, avec leur écriture et leur convention de translittération si elles ne s’écrivent pas en alphabet latin ([ADR 0006](0006-architecture-multilingue.md)).

### Toujours sans variété

- Le bas latin, le latin médiéval, le latin chrétien et le latin scientifique restent du **latin** ; l’ancien et le moyen français restent du **français**, sous forme de graphie ancienne ; l’anglo-américain reste de l’**anglais**.
- La précision va dans l’explication détaillée ou dans la `note_simplification` du fait, comme le prévoit l’[ADR 0013](0013-schema-des-briques.md).

### Une langue d’emprunt est une langue comme les autres

- Elle a son nom en toutes lettres, sa lettre emblème, sa langue parente quand elle descend d’une langue du catalogue (l’italien descend du latin), et sa **silhouette** ([ADR 0023](0023-textures-des-cartes.md)) dès qu’un fascicule la cite.
- Elle a sa carte dans le codex. Comme les autres, cette carte regroupe les formes découvertes dans cette langue : elle se remplit avec les étymons des mots trouvés.
- Elle n’est jamais une brique, et un mot anglais ou allemand n’a pas de carte propre : c’est un étymon ([ADR 0013](0013-schema-des-briques.md)).
- La parenté des langues (`langue_parente_id`) reste une généalogie : un emprunt ne crée jamais de parenté entre deux langues.

## Options envisagées

### Garder trois langues

Écartée : la chaîne de *biologie* devrait sauter l’allemand et prétendre que le français a emprunté le mot au grec, ce que la source dément.

### Ajouter aussi les variétés du latin et du français

Écartée, pour les raisons de l’[ADR 0013](0013-schema-des-briques.md) : des cartes de langue presque vides, une frontière discutable entre deux variétés, un coût éditorial sans gain pour le joueur.

### Déclarer d’avance une liste de langues

Écartée : une langue sans mot publié n’a rien à montrer, et chacune demande une silhouette et une relecture.

## Conséquences

### Positives

- Les chaînes suivent la source, y compris les emprunts modernes à l’anglais et à l’allemand.
- Le codex gagne des cartes de langue qui racontent les routes des mots, sans variété à départager.
- Chaque nouvelle langue a un déclencheur clair : un mot publié qui la cite.

### Négatives

- Une silhouette et une lettre emblème à dessiner par langue ajoutée.
- Une carte de langue peut rester presque vide si un seul mot passe par elle.
- Le rattachement d’une variété à sa langue simplifie l’histoire : la note doit le dire.

## Critères de réévaluation

- Une carte de langue reste vide ou quasi vide pour la plupart des joueurs : regrouper ses mots sous une autre présentation.
- Les variétés reviennent souvent dans les explications et les joueurs les demandent : rouvrir les variétés par un nouvel ADR.
- Une langue non latine arrive (arabe, persan) : vérifier son écriture, sa direction et sa convention avant la saisie.
