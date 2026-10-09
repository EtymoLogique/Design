# ADR 0056 — Date de première attestation d’un mot

- **Statut** : Proposé, source de la date remplacée par l’[ADR 0060](0060-wikidata-seule-source-de-verite.md) : elle est lue dans la déclaration « first attested from » du lexème Wikidata, non plus dans le TLFi
- **Portée** : modèle de données du catalogue et fiche d’une carte
- **Complète** : [ADR 0002](0002-graphe-linguistique-editorial.md) (graphe linguistique éditorial), [ADR 0013](0013-schema-des-briques.md) (schéma des briques), [ADR 0014](0014-sources-et-fascicules.md) (sources), [ADR 0027](0027-catalogue-statique-et-donnees-joueur.md) (artefact)

## Contexte

Aucun champ ne dit quand un mot apparaît en français, ni quand un emprunt a eu lieu. C’est pourtant l’information la plus parlante d’une carte : « attesté vers 1175 » dit en trois mots que *philosophie* est un mot du Moyen Âge, et que *psychologie* ne l’est pas.

Le TLFi donne cette date pour chaque mot, au début de sa rubrique « Étymologie et histoire », avec des précisions variables : une année (« 1557 »), une approximation (« ca 1175 »), une borne (« av. 1350 »), un intervalle (« 1225-50 ») ou un siècle (« xiiiᵉ s. »). La première date n’est pas toujours celle du sens actuel : *philosophie* est attesté vers 1175 pour un ensemble de disciplines, et ses autres sens viennent plus tard.

Trois endroits pouvaient porter la date : la forme, l’unité ou la relation `emprunt` / `heritage` (modèle de données, sections 2 et 3).

## Décision

### Elle vit sur l’unité, avec son propre fait

- Une **unité de nature mot**, en **français**, peut porter une **première attestation** : la date à laquelle le mot apparaît pour la première fois dans un texte français, quel que soit son sens.
- L’attestation est un **fait** à part entière ([ADR 0002](0002-graphe-linguistique-editorial.md)), comme une forme ou un sens : sa confiance, sa relecture et ses sources ne sont pas celles de l’unité.
- Une unité a **au plus une** première attestation. Les dates d’une graphie ancienne ou d’un sens plus récent ne sont pas des champs : elles se racontent dans l’explication détaillée.
- La date d’un emprunt ou d’un héritage n’est pas stockée sur la relation : c’est la date d’attestation de l’unité cible. La stocker deux fois laisserait les deux diverger.
- Les étymons grecs et latins n’ont pas de date dans le MVP : leurs attestations se donnent par auteur (Platon, Cicéron), pas par année.

### Sa précision

| Précision | Exemple TLFi | Champs | Affichage |
|---|---|---|---|
| `annee` | 1557 | `annee` | « attesté en 1557 » |
| `vers` | ca 1175 | `annee` | « attesté vers 1175 » |
| `avant` | av. 1350 | `annee` | « attesté avant 1350 » |
| `intervalle` | 1225-50 | `annee`, `annee_fin` | « attesté entre 1225 et 1250 » |
| `siecle` | xiiiᵉ s. | `annee` (une année du siècle) | « attesté au XIIIᵉ siècle » |

- `annee` est un entier, jamais dans le futur ; `annee_fin` n’existe que pour un intervalle, et lui est postérieure.
- Une précision plus fine que la source (« début du xiiiᵉ s. » saisi comme « 1201 ») est interdite : on garde la précision de la source, au besoin en arrondissant vers le siècle.

### Sa source : une référence, jamais le Wiktionnaire seul

- La date vient d’une **source de référence** ([ADR 0014](0014-sources-et-fascicules.md)) : le TLFi, rubrique « Étymologie et histoire », ou un dictionnaire historique de même rang. Sa localisation est l’entrée consultée.
- Le Wiktionnaire peut aider à repérer une date, jamais la justifier : une attestation qui n’a qu’une source de repérage reste un `brouillon` et n’est pas publiée.
- Quand une étude plus récente antidate le mot, la date retenue reste celle de la source de référence, et l’écart se note dans `note_interne`.
- Un mot sans date sûre **n’en affiche pas** : l’absence n’est jamais comblée par une estimation.

### Son affichage

- La fiche d’un mot découvert affiche la date à côté de sa langue (« français, attesté vers 1175 »). Elle est compilée dans la **ressource de la carte** ([ADR 0027](0027-catalogue-statique-et-donnees-joueur.md)) et publiée avec son fascicule, pas avant.
- Une carte inconnue n’affiche pas de date : elle reste en silhouette, sans autre piste que la sienne.
- Le jeu n’en tire aucune règle dans le MVP : ni fusion, ni indice, ni jalon, ni quête ne dépend d’une date. Une frise ou un tri chronologique du codex demanderait un nouvel ADR.

## Options envisagées

### La date sur la forme

Une graphie ancienne aurait sa propre date, ce qui serait plus exact. Écartée : le joueur découvre un mot, pas une graphie, et la plupart des formes du catalogue n’ont qu’une graphie. Le gain ne paie pas un champ de plus par forme.

### La date sur la relation `emprunt` ou `heritage`

Elle dirait quand le mot a changé de langue. Écartée : beaucoup de mots du catalogue sont formés en français, sans relation entrante, et n’auraient pas de date ; pour les autres, la date de la relation est celle de l’attestation de la cible.

### Un champ texte libre (« ca 1175 »)

Rapide à saisir, mais impossible à contrôler, à trier ou à traduire. Écartée.

### Aucune date

C’est l’état actuel. Il prive la carte de son repère le plus concret. Écartée.

## Conséquences

### Positives

- Chaque carte de mot gagne un repère historique sourcé, sans nouvelle brique ni nouvelle règle de jeu.
- La date se relit comme les autres faits, avec sa confiance et sa localisation.
- Le champ est structuré : un tri ou une frise restent possibles plus tard.

### Négatives

- Un fait de plus par mot à saisir et à relire.
- Le contrôle de la précision demande une règle de plus dans le validateur du dépôt Content.
- Les dates du TLFi ont l’âge du TLFi : certaines sont antidatées depuis.

## Critères de réévaluation

- Les joueurs ne regardent pas la date sur la fiche : la retirer de l’affichage, pas du catalogue.
- Un besoin de dater les étymons ou les sens apparaît (vue Filiation, frise) : étendre l’attestation par un nouvel ADR.
- Une source de référence plus récente que le TLFi devient accessible : changer la source par défaut.
