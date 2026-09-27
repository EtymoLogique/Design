# ADR 0027 — Catalogue statique par fascicules, recettes publiques, serveur qui fait foi

- **Statut** : Proposé
- **Remplace partiellement** :
  - [ADR 0003](0003-recettes-de-fusion.md), pour le secret des recettes verrouillées ;
  - [ADR 0024](0024-architecture-logicielle-et-hebergement.md), pour les deux artefacts (public et serveur), la projection du contenu par joueur et le bucket privé du catalogue ;
  - [ADR 0025](0025-medias-statiques-et-publication-programmee.md), pour la correspondance entre cartes et médias, tenue dans l'artefact serveur ;
  - [ADR 0026](0026-depot-prive-du-catalogue-sans-logique.md), pour la règle qui n'envoie jamais les recettes au client.
- **Précise** : [ADR 0015](0015-codex-fascicules-et-legendaires.md) (le secret des légendaires tient à l'interface, pas aux fichiers)
- **Complète** : [ADR 0005](0005-pipeline-de-contenu.md) (forme de l'artefact), [ADR 0007](0007-etat-et-economie-autoritaires.md) (données du joueur), [ADR 0010](0010-observabilite-et-vie-privee.md) (ce qui n'est jamais stocké)

## Contexte

L'[ADR 0024](0024-architecture-logicielle-et-hebergement.md) compile deux artefacts : un **public**, presque vide, et un **serveur**, qui contient tout le reste. Le client ne reçoit que la projection calculée pour chaque joueur. L'[ADR 0025](0025-medias-statiques-et-publication-programmee.md) garde dans l'artefact serveur le lien entre une carte et sa silhouette, et l'[ADR 0026](0026-depot-prive-du-catalogue-sans-logique.md) refuse d'envoyer une recette au client.

Ce montage protège un secret fragile. La fiche d'un mot découvert donne déjà sa composition, donc sa recette. Une recette encore inconnue se partage en une phrase sur un forum. Pour ce secret, le serveur doit pourtant :

- tenir un second artefact et un bucket privé ;
- calculer et servir une projection de contenu pour chaque joueur ;
- servir chaque fiche par l'API, au lieu du CDN.

Ce qu'il faut vraiment garantir, c'est qu'**aucun fascicule ne se lise avant sa parution** et que **l'état du joueur ne soit jamais falsifiable**. Le serveur fait foi, que les recettes soient publiques ou non.

## Décision

### Un seul artefact, public, en trois niveaux

Le pipeline de l'[ADR 0005](0005-pipeline-de-contenu.md) compile, pour chaque version, **un seul artefact**. Il est déposé dans l'Object Storage public, derrière le CDN, avec le **listing refusé** ([ADR 0025](0025-medias-statiques-et-publication-programmee.md)).

| Niveau | Fichier | Contenu |
|---|---|---|
| 1 | **Index global**, `index.json` | Version active, référentiels communs (langues, écritures, conventions et leurs libellés), liste des **fascicules parus** : numéro, date de parution, chemin du manifeste. |
| 2 | **Manifeste de fascicule** | Numéro, date, jaquette, compteurs hors légendaires, paramètres du pli (briques tirables, raretés, poids), jalons, et le chemin de chacune de ses ressources. |
| 3 | **Ressources** | Cartes (brique ou mot), recettes, exclusions, médias (jaquette, silhouettes). |

- **Chemins en hash de contenu** : chaque manifeste et chaque ressource est rangé sous l'empreinte SHA-256 de son contenu, comme les médias de l'[ADR 0025](0025-medias-statiques-et-publication-programmee.md). Un fichier publié ne change jamais : il se met en cache sans limite, et une brique reprise par un fascicule suivant garde le même fichier.
- **L'index global est le seul fichier modifiable.** Il prend la place du manifeste public de l'[ADR 0025](0025-medias-statiques-et-publication-programmee.md) : il ne liste que les fascicules déjà parus, jamais un fascicule à venir.
- **Parution programmée** : elle suit l'[ADR 0025](0025-medias-statiques-et-publication-programmee.md). Les fichiers sont déposés à l'avance. À la date d'effet, une tâche planifiée ajoute le fascicule à l'index global et applique les migrations. Avant cette date, aucun chemin n'est publié, et l'empreinte d'un contenu inconnu ne se devine pas.
- **Corrections** : une ressource corrigée a un nouveau hash, donc un nouveau manifeste, donc un nouvel index. Les versions restent immuables ([ADR 0002](0002-graphe-linguistique-editorial.md)).
- **Jamais dans l'artefact** : `note_interne`, brouillons, faits rejetés ou `non_retenue`, état de relecture, imports du Wiktionnaire, manifeste serveur daté. Ils restent dans le dépôt privé ([ADR 0026](0026-depot-prive-du-catalogue-sans-logique.md)).

### Ce qui change dans les ADR 0024, 0025 et 0026

- **ADR 0024** : plus d'artefact serveur, plus de bucket privé du catalogue, plus de projection de contenu par joueur. L'API lit le même artefact public, le garde en mémoire et relit l'index global. La projection du joueur ne contient plus que son **état** (réserve, découvertes, énergie, encre, sabliers, compteurs) : des identifiants et des nombres, jamais de texte du catalogue.
- **ADR 0025** : la correspondance entre une carte et sa silhouette est dans le manifeste de son fascicule, pas dans un artefact serveur. Les médias gardent leur nom opaque, et le service worker ne précache toujours aucun lot.
- **ADR 0026** : le dépôt privé reste privé et ne contient toujours que des données. Mais ce qu'il compile pour un fascicule paru, recettes comprises, est public.
- L'hébergement, le backend Rust, les tables PostgreSQL, l'identité, la sécurité et la publication programmée de ces ADR restent en vigueur.

### Les recettes sont publiques, le serveur fait foi

- Remplace, dans l'[ADR 0003](0003-recettes-de-fusion.md), la phrase « le serveur ne renvoie pas l'identifiant ou les ingrédients secrets d'une recette verrouillée ». Une fois le fascicule paru, ses recettes sont lisibles dans ses fichiers.
- En ligne, le client peut **prévisualiser** une fusion à partir des recettes publiques, pour répondre sans attendre. Hors connexion, la tentative attend toujours le réseau ([ADR 0024](0024-architecture-logicielle-et-hebergement.md)) : aucun résultat n'est présenté comme acquis.
- **Le serveur fait foi** pour toute règle et toute mutation ([ADR 0007](0007-etat-et-economie-autoritaires.md)). Dans une seule transaction, il :
  - valide la recette, son ordre et ses préconditions ;
  - vérifie que la réserve contient un exemplaire de chaque brique utilisée ;
  - consomme les exemplaires, ajoute la découverte au codex et donne les récompenses.
- Il en va de même pour l'énergie, les plis, la garantie et le filet, l'encre, les sabliers et leurs plafonds, les indices et les jalons.
- Si le serveur contredit la prévisualisation, le client affiche la réponse du serveur.

### Les légendaires restent secrètes dans l'interface

- Précise l'[ADR 0015](0015-codex-fascicules-et-legendaires.md) : ni silhouette ni compteur ne trahissent une légendaire inconnue **à l'écran**. Les fichiers d'un fascicule paru, eux, la contiennent.
- Le manifeste ne porte que des compteurs hors légendaires. La ligne « Légendaires inconnues » de l'écran des plis n'affiche que la somme de leurs chances.

### Les données du joueur : les tables de l'ADR 0024

La progression reste dans les tables relationnelles PostgreSQL de l'[ADR 0024](0024-architecture-logicielle-et-hebergement.md). Le [modèle de données](../modele-donnees.md#7-données-du-joueur) en donne le schéma logique et y ajoute l'historique des plis, les indices, les jalons atteints et les migrations. Les propositions de fusion ne sont **jamais stockées** : la télémétrie ne compte que les résultats ([ADR 0010](0010-observabilite-et-vie-privee.md)).

## Options envisagées

### Garder deux artefacts et la projection de contenu (ADR 0024)

Écarté : un second artefact, un bucket privé et une projection de contenu par joueur, pour protéger des recettes que les fiches découvertes et le bouche-à-oreille dévoilent déjà.

### Chemins hachés par une clé privée du backend

Les chemins ne se liraient qu'avec l'aide du serveur, même après la parution. Mais le backend devrait transmettre chaque chemin, et changer de clé renommerait tous les fichiers. Écarté : le hash de contenu suffit à garantir la seule chose retenue, rien avant la date.

### Un seul fichier par version du catalogue

Écarté : chaque correction ou parution ferait retélécharger tout le catalogue.

### Résolution des fusions par le client seul

Écartée : réserve, codex, encre et sabliers deviendraient falsifiables ([ADR 0007](0007-etat-et-economie-autoritaires.md)).

## Conséquences

### Positives

- Un seul artefact à compiler, déposer et relire. Plus de bucket privé ni de projection de contenu.
- Les fiches, recettes et médias passent par le CDN, sans appel à l'API.
- En ligne, la table répond tout de suite. Le serveur garde seul l'autorité sur l'état du joueur.
- Un fascicule se prépare à l'avance et paraît seul, à la date prévue.

### Négatives

- Dès la parution, un joueur curieux peut lire toutes les recettes et légendaires d'un fascicule, et les partager.
- Le client et le serveur doivent lire les recettes de la même façon, sinon la prévisualisation est démentie. Un schéma commun et des tests partagés limitent ce risque.
- Un fascicule paru ne s'efface pas des caches : seule une nouvelle version peut le corriger.

## Critères de réévaluation

- Des solutions publiées dès la parution réduisent nettement la part de joueurs qui cherchent par eux-mêmes : revenir aux deux artefacts de l'[ADR 0024](0024-architecture-logicielle-et-hebergement.md).
- La prévisualisation du client est souvent démentie par le serveur.
- Un besoin de secret après la parution (événement, légendaire à durée limitée) : étudier des chemins hachés par une clé privée.
