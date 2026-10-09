# ADR 0055 — Prononciation des mots : transcription phonétique, sans audio

- **Statut** : Proposé
- **Portée** : modèle de données du catalogue et fiche d’une carte
- **Complète** : [ADR 0006](0006-architecture-multilingue.md) (formes, écritures et translittérations), [ADR 0014](0014-sources-et-fascicules.md) (sources), [ADR 0025](0025-medias-statiques-et-publication-programmee.md) (médias), [ADR 0027](0027-catalogue-statique-et-donnees-joueur.md) (artefact)

## Contexte

Les mots français n’ont ni transcription phonétique ni audio. Seules les formes grecques ont une translittération ([ADR 0006](0006-architecture-multilingue.md)). Or la prononciation éclaire des familles qui ne se ressemblent plus à l’écrit : « aquarium » (/akwaʁjɔm/) garde le son latin que « eau » (/o/) a perdu.

Deux voies existent : une **transcription** (alphabet phonétique international, API), qui est du texte, ou un **enregistrement audio**. Le second suppose de trouver, d’héberger, de licencier et de relire un fichier par mot ; la première se tire d’une donnée déjà structurée. Le catalogue publié est un artefact statique ([ADR 0027](0027-catalogue-statique-et-donnees-joueur.md)), sans service tiers à l’exécution.

## Décision

### Une prononciation écrite en API, jamais d’audio

- Chaque forme **française** d’un mot porte une **prononciation** : une transcription en alphabet phonétique international (API), pour le français standard de France. Une seule prononciation de référence par forme dans le MVP ; pas de variantes régionales.
- Il n’y a **aucun audio**, ni enregistré, ni synthétisé : ni fichier, ni synthèse vocale du navigateur, ni appel à un service tiers.
- Les formes grecques et latines gardent leur translittération ([ADR 0006](0006-architecture-multilingue.md)) et ne reçoivent pas de transcription phonétique dans le MVP.

### Elle vit sur la forme

- Le champ `prononciation` est un attribut de la **forme** ([ADR 0006](0006-architecture-multilingue.md)), au même titre que la translittération. Il ne crée ni brique, ni carte, ni découverte : une prononciation n’est pas collectionnable.
- Il contient le texte API sans les barres obliques (« ɔ » et non « /ɔ/ ») et une référence à sa source ([ADR 0014](0014-sources-et-fascicules.md)). Les barres sont ajoutées à l’affichage.
- Il est compilé dans la **ressource de la carte** de l’[ADR 0027](0027-catalogue-statique-et-donnees-joueur.md), donc publié avec le fascicule, pas avant : une prononciation dévoile le mot comme son orthographe.

### Sa source : Wiktextract, relue

- Les transcriptions sont **extraites des données de Wiktextract** (extraction hors ligne du Wiktionnaire, diffusée sur kaikki.org), une fois, à la préparation du fascicule. Aucune API n’est interrogée à l’exécution, ni par l’application ni par le pipeline.
- Cela **complète sans contredire** l’[ADR 0014](0014-sources-et-fascicules.md) : le Wiktionnaire reste une source de repérage pour les étymologies, dont aucun texte n’est repris. Une transcription est une donnée brève, factuelle et sans enjeu éditorial ; elle est importée comme telle, et seulement elle.
- Chaque transcription est **relue** avant publication, contre un dictionnaire de référence (TLFi ou Académie française) quand il en donne une, et marquée `reperage` sinon. La licence CC BY-SA 4.0 du catalogue est compatible avec celle du Wiktionnaire ([ADR 0017](0017-licences.md)) ; la source est citée dans la liste des sources du fascicule.
- Un mot sans transcription fiable **n’en affiche pas** : l’absence n’est jamais comblée par une génération automatique à partir de l’orthographe.

### Son affichage

- La fiche d’un mot affiche la transcription entre barres obliques, à côté de la forme écrite, en police du design système (couverture API vérifiée dans [typographie](../typographie.html)).
- Le texte est balisé comme du texte API pour les lecteurs d’écran, avec une alternative lisible : l’écriture normale du mot reste toujours à côté. La prononciation n’est **jamais** le seul moyen de comprendre une carte ni de jouer.
- Le jeu n’en tire aucune règle : ni fusion, ni indice, ni jalon ne dépend d’une prononciation.

## Options envisagées

### Audio enregistré (Wikimédia Commons ou voix dédiée)

Plus parlant qu’une transcription, mais il faut interroger l’API de Commons ou produire les voix, relire la licence de chaque fichier, héberger des médias supplémentaires ([ADR 0025](0025-medias-statiques-et-publication-programmee.md)), et ces fichiers manquent pour beaucoup de mots. Écarté : la charge d’intégration et de maintenance dépasse le gain au MVP.

### Synthèse vocale du navigateur

Aucun fichier à produire, mais la voix varie selon l’appareil, la prononciation de mots savants est souvent fausse, et rien ne la garantit hors connexion ([ADR 0041](0041-hors-connexion-consultation-seule.md)). Écarté.

### Aucune prononciation

C’est l’état actuel. Il laisse sans réponse le besoin des familles que l’écrit sépare. Écarté.

### Générer la transcription à partir de l’orthographe

Rapide, mais le français est irrégulier (« oignon », « femme », les noms propres et les mots savants) : l’erreur serait silencieuse dans un jeu qui prétend à la rigueur. Écarté.

## Conséquences

### Positives

- pas de service tiers ni de fichier supplémentaire : le catalogue reste statique et hors connexion ;
- taille négligeable : quelques octets par mot, soit quelques kilooctets par fascicule ;
- le modèle est prêt à recevoir d’autres langues ou un audio plus tard, sans refonte ;
- aucune dépendance à l’exécution.

### Négatives

- l’API est peu lisible pour une partie du public, et mal lu par les lecteurs d’écran : un gain pour certains joueurs, sans effet pour d’autres ;
- un travail de relecture par fascicule (20 à 30 mots, [ADR 0022](0022-fascicules-de-20-a-30-mots.md)) ;
- une donnée de plus à maintenir dans le schéma et le pipeline ([ADR 0005](0005-pipeline-de-contenu.md)).

## Critères de réévaluation

- Des retours de joueurs ou de testeurs montrent que l’API n’est pas comprise : ajouter une aide de lecture, ou reprendre l’audio.
- Un jeu de voix libres couvre la grande majorité du vocabulaire publié, sans API à intégrer : rouvrir l’audio par un nouvel ADR.
- Plus d’une langue d’interface ou de langue jouable est livrée : étendre la prononciation aux autres langues.
