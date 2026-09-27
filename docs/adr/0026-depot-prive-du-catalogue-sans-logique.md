# ADR 0026 — Un dépôt privé pour le catalogue brut, sans aucune logique

- **Statut** : Accepté, partiellement remplacé par l’[ADR 0027](0027-catalogue-statique-et-donnees-joueur.md) : les recettes d’un fascicule paru sont publiées dans l’artefact ; le dépôt, lui, reste privé
- **Portée** : organisation des dépôts, pipeline de contenu, publication des fascicules
- **Remplace partiellement** : [ADR 0024](0024-architecture-logicielle-et-hebergement.md), pour le contenu du dépôt privé (sources éditoriales et artefact serveur)
- **Complète** : [ADR 0005](0005-pipeline-de-contenu.md) (pipeline), [ADR 0017](0017-licences.md) (licences), [ADR 0025](0025-medias-statiques-et-publication-programmee.md) (publication programmée)

## Contexte

Le projet est libre et son dépôt public ([ADR 0017](0017-licences.md)). Or le catalogue est le jeu lui-même : lire les recettes, les mots ou les silhouettes d’un fascicule sur GitHub, c’est se gâcher la découverte. L’[ADR 0024](0024-architecture-logicielle-et-hebergement.md) prévoit déjà un dépôt privé pour « les sources éditoriales non publiées et l’artefact serveur », sans dire ce qu’il peut contenir d’autre.

Deux risques opposés :

- mettre du code dans le dépôt privé fermerait une partie du jeu, contre l’esprit de l’AGPL, et ferait diverger deux bases de code ;
- mettre le contenu dans le dépôt public dévoilerait chaque fascicule dès sa préparation, et les recettes pour toujours.

La publication programmée ([ADR 0025](0025-medias-statiques-et-publication-programmee.md)) demande en outre de préparer plusieurs fascicules à l’abri des regards.

## Décision

### Deux dépôts, une frontière nette

| Dépôt | Visibilité | Contenu |
|---|---|---|
| Dépôt de l’application (celui-ci) | Public | Tout le code : PWA, API, schéma du catalogue, compilateur, validations, outil de migration, infrastructure, dossier de conception, un jeu d’essai |
| Dépôt du catalogue | Privé | Des données seulement : sources éditoriales, médias, manifeste serveur daté, migrations déclaratives, et la CI qui les publie |

- La règle tient en une phrase : **le dépôt privé contient des données, jamais de logique**.
- Le dépôt privé ne contient ni l’artefact public ni l’artefact serveur : ce sont des produits de compilation, déposés par la CI dans l’Object Storage ([ADR 0024](0024-architecture-logicielle-et-hebergement.md)), jamais commités.

### Le dépôt privé : des données, rien d’autre

- **Sources éditoriales** : briques, mots, recettes, fiches, sources, poids et raretés des plis, garanties, plis de jalon, légendaires, dans le format d’édition validé par le schéma du dépôt public ([ADR 0005](0005-pipeline-de-contenu.md), [ADR 0013](0013-schema-des-briques.md)).
- **Médias bruts** : jaquettes ([ADR 0016](0016-plis-et-jaquettes-par-fascicule.md)) et silhouettes ([ADR 0023](0023-textures-des-cartes.md)), sous des noms de travail lisibles. Le compilateur leur donne leur nom opaque à la publication ([ADR 0025](0025-medias-statiques-et-publication-programmee.md)).
- **Manifeste serveur** : les versions du catalogue et leur date d’effet ([ADR 0025](0025-medias-statiques-et-publication-programmee.md)).
- **Migrations de progression déclaratives** : des tables de correspondance (ancien identifiant, nouvel identifiant, ou retrait), jamais un script. L’outil qui les applique est dans le dépôt public.
- **CI** : le seul code toléré. Elle se limite à appeler les outils du dépôt public, dans une version épinglée (image de conteneur ou version publiée), pour valider, compiler, déposer les artefacts et les médias, et mettre à jour le manifeste serveur. Aucune règle de jeu, aucune validation, aucune transformation des données n’y est écrite.
- Les secrets de déploiement vivent dans Secret Manager et dans les secrets de la CI, jamais dans l’un ou l’autre dépôt ([ADR 0024](0024-architecture-logicielle-et-hebergement.md)).

### Le dépôt public : tout le reste

- Le schéma du format d’édition et de l’artefact, le compilateur, toutes les validations de l’[ADR 0005](0005-pipeline-de-contenu.md) (références, sources, atteignabilité, recettes ambiguës, plis) et l’outil qui applique les migrations.
- Un **jeu d’essai** : un petit catalogue, fictif ou déjà publié, assez complet pour tester chaque validation, compiler un artefact et faire tourner l’API et la PWA en local, sans accès au dépôt privé.
- Les exemples du dossier de conception restent des illustrations. Ils ne préfigurent aucun fascicule à venir.
- Toute évolution du format se fait d’abord dans le dépôt public. Le dépôt privé adopte la nouvelle version en changeant l’épinglage de sa CI.

### Licences et ouverture

- Le jeu servi en ligne ne dépend d’aucun code fermé : l’obligation de l’AGPL ([ADR 0017](0017-licences.md)) est remplie par le seul dépôt public.
- Le contenu reste sous CC BY-SA 4.0. Ce qui est révélé aux joueurs (fiches des cartes découvertes, gloses, jaquettes, silhouettes) peut être repris sous cette licence. La date de révélation est retenue, pas la licence ([ADR 0024](0024-architecture-logicielle-et-hebergement.md)).
- Le dépôt privé **reste privé après la parution** d’un fascicule : ses recettes sont l’énigme du jeu et ne sont jamais envoyées au client ([ADR 0024](0024-architecture-logicielle-et-hebergement.md)).

## Options envisagées

### Tout dans le dépôt public

Simple, mais chaque fascicule serait lisible dès sa préparation, et toutes les recettes pour toujours. Écarté.

### Compilateur et validations dans le dépôt privé

Plus pratique pour faire évoluer le format et le contenu ensemble, mais une partie du jeu deviendrait fermée, invérifiable et impossible à reprendre par un fork. Écarté.

### Sous-module Git privé dans le dépôt public

Il lierait les deux dépôts dans un seul arbre, mais exposerait le nom et l’avancement du dépôt privé, et casserait chaque clone public sans accès. Écarté.

### Contenu chiffré dans le dépôt public

Un seul dépôt, mais les noms de fichiers, les tailles et l’historique des commits trahiraient le rythme et l’ampleur des fascicules, et la clé deviendrait un secret de plus à protéger. Écarté.

### Publier le dépôt privé à chaque parution

Il rendrait le contenu entièrement ouvert, mais dévoilerait les recettes des fascicules parus et les légendaires encore inconnues de nombreux joueurs. Écarté.

## Conséquences

### Positives

- Les fascicules se préparent et se livrent sans rien dévoiler sur GitHub.
- Tout le code reste libre, vérifiable et reprenable : aucune règle ne se cache dans le dépôt privé.
- Un contributeur travaille entièrement dans le dépôt public, grâce au jeu d’essai.
- Le dépôt privé, fait de données, est simple à relire et à auditer.

### Négatives

- Deux dépôts à tenir, et un épinglage de version à suivre à chaque évolution du format.
- Le jeu d’essai doit rester assez riche pour couvrir toutes les validations.
- Un fork reprend le code, mais pas le catalogue non révélé : il devra écrire son propre contenu.
- Le contenu ne reçoit pas de contributions publiques directes : une proposition passe par une issue, puis par une saisie dans le dépôt privé.

## Critères de réévaluation

- Une règle de jeu ou une validation apparaît dans la CI du dépôt privé : la déplacer dans le dépôt public.
- Des contributeurs veulent proposer du contenu directement : étudier un circuit de contribution qui ne dévoile rien.
- Le contenu d’un fascicule fuite depuis le dépôt privé : revoir ses accès et ses secrets.
