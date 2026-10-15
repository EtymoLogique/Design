# ADR 0026 — Un dépôt public pour le catalogue brut, sans aucune logique

- **Statut** : Accepté, partiellement remplacé par l’[ADR 0027](0027-catalogue-statique-et-donnees-joueur.md) : les recettes d’un fascicule paru sont publiées dans l’artefact ; le dépôt du catalogue est public, et c’est voulu
- **Portée** : organisation des dépôts, pipeline de contenu, publication des fascicules
- **Remplace partiellement** : [ADR 0024](0024-architecture-logicielle-et-hebergement.md), pour le contenu du dépôt du catalogue (sources éditoriales et artefact serveur)
- **Complète** : [ADR 0005](0005-pipeline-de-contenu.md) (pipeline), [ADR 0017](0017-licences.md) (licences), [ADR 0025](0025-medias-statiques-et-publication-programmee.md) (publication programmée)

## Contexte

Le projet est libre et son dépôt public ([ADR 0017](0017-licences.md)). Le catalogue est le jeu lui-même. Cet ADR le prévoyait d’abord privé, pour ne pas dévoiler les recettes sur GitHub ; l’[ADR 0027](0027-catalogue-statique-et-donnees-joueur.md) a renoncé à ce secret, et le dépôt du catalogue est public, volontairement. L’[ADR 0024](0024-architecture-logicielle-et-hebergement.md) prévoyait un dépôt privé pour « les sources éditoriales non publiées et l’artefact serveur », sans dire ce qu’il peut contenir d’autre.

Deux risques opposés :

- mettre du code dans le dépôt du catalogue le séparerait du code libre et fermerait une partie du jeu, contre l’esprit de l’AGPL, et ferait diverger deux bases de code ;
- mettre le contenu dans le dépôt de l’application mêlerait code et données, pour un contenu qui n’a pas le même rythme.

La publication programmée ([ADR 0025](0025-medias-statiques-et-publication-programmee.md)) demande en outre de préparer plusieurs fascicules à part, dans un dépôt de données.

## Décision

### Deux dépôts, une frontière nette

| Dépôt | Visibilité | Contenu |
|---|---|---|
| Dépôt de l’application (celui-ci) | Public | Tout le code : PWA, API, schéma du catalogue, compilateur, validations, outil de migration, infrastructure, dossier de conception, un jeu d’essai |
| Dépôt du catalogue (Content) | Public | Des données seulement : sources éditoriales, médias, manifeste serveur daté, migrations déclaratives, et la CI qui les publie |

- La règle tient en une phrase : **le dépôt du catalogue contient des données, jamais de logique**.
- Le dépôt du catalogue ne contient ni l’artefact public ni l’artefact serveur : ce sont des produits de compilation, déposés par la CI dans l’Object Storage ([ADR 0024](0024-architecture-logicielle-et-hebergement.md)), jamais commités.

### Le dépôt du catalogue : des données, rien d’autre

- **Sources éditoriales** : briques, mots, recettes, fiches, sources, poids et raretés des plis, garanties, plis de jalon, légendaires, dans le format d’édition validé par le schéma du dépôt public ([ADR 0005](0005-pipeline-de-contenu.md), [ADR 0013](0013-schema-des-briques.md)).
- **Médias bruts** : jaquettes ([ADR 0016](0016-plis-et-jaquettes-par-fascicule.md)) et silhouettes ([ADR 0023](0023-textures-des-cartes.md)), sous des noms de travail lisibles. Le compilateur leur donne leur nom opaque à la publication ([ADR 0025](0025-medias-statiques-et-publication-programmee.md)).
- **Manifeste serveur** : les versions du catalogue et leur date d’effet ([ADR 0025](0025-medias-statiques-et-publication-programmee.md)).
- **Migrations de progression déclaratives** : des tables de correspondance (ancien identifiant, nouvel identifiant, ou retrait), jamais un script. L’outil qui les applique est dans le dépôt public.
- **CI** : le seul code toléré. Elle se limite à appeler les outils du dépôt public, dans une version épinglée (image de conteneur ou version publiée), pour valider, compiler, déposer les artefacts et les médias, et mettre à jour le manifeste serveur. Aucune règle de jeu, aucune validation, aucune transformation des données n’y est écrite.
- Les secrets de déploiement vivent dans Secret Manager et dans les secrets de la CI, jamais dans l’un ou l’autre dépôt ([ADR 0024](0024-architecture-logicielle-et-hebergement.md)).

### Le dépôt public : tout le reste

- Le schéma du format d’édition et de l’artefact, le compilateur, toutes les validations de l’[ADR 0005](0005-pipeline-de-contenu.md) (références, sources, atteignabilité, recettes ambiguës, plis) et l’outil qui applique les migrations.
- Un **jeu d’essai** : un petit catalogue, fictif ou déjà publié, assez complet pour tester chaque validation, compiler un artefact et faire tourner l’API et la PWA en local, sans accès au dépôt du catalogue.
- Les exemples du dossier de conception restent des illustrations. Ils ne préfigurent aucun fascicule à venir.
- Toute évolution du format se fait d’abord dans le dépôt public. Le dépôt du catalogue adopte la nouvelle version en changeant l’épinglage de sa CI.

### Licences et ouverture

- Le jeu servi en ligne ne dépend d’aucun code fermé : l’obligation de l’AGPL ([ADR 0017](0017-licences.md)) est remplie par le seul dépôt public.
- Le contenu reste sous CC BY-SA 4.0. Ce qui est révélé aux joueurs (fiches des cartes découvertes, gloses, jaquettes, silhouettes) peut être repris sous cette licence. La date de révélation est retenue, pas la licence ([ADR 0024](0024-architecture-logicielle-et-hebergement.md)).
- Le dépôt du catalogue est **public**, à toute étape : l’[ADR 0027](0027-catalogue-statique-et-donnees-joueur.md) publie déjà les recettes d’un fascicule paru, et le secret tient à l’interface, non au dépôt. Ses sources ne contiennent aucun secret : les secrets de déploiement vivent hors des dépôts.

## Options envisagées

### Tout dans le dépôt de l’application

Simple, mais le code et le catalogue n’ont ni le même rythme ni les mêmes relecteurs, et chaque fascicule en préparation encombrerait l’historique du code. Écarté.

### Compilateur et validations dans le dépôt du catalogue

Plus pratique pour faire évoluer le format et le contenu ensemble, mais une partie du jeu deviendrait fermée, invérifiable et impossible à reprendre par un fork. Écarté.

### Sous-module Git dans le dépôt de l’application

Il lierait les deux dépôts dans un seul arbre et ferait dépendre chaque clone de l’avancement du catalogue. Écarté.

### Contenu chiffré dans le dépôt de l’application

Un seul dépôt, mais la clé deviendrait un secret de plus à protéger, pour un contenu que l’artefact publie de toute façon à la parution. Écarté.

### Garder le dépôt du catalogue privé

Il garderait les fascicules en préparation hors de vue, mais il ferait d’un secret de dépôt une protection que l’[ADR 0027](0027-catalogue-statique-et-donnees-joueur.md) a jugée inutile, et il fermerait aux contributeurs la lecture du catalogue. Écarté.

## Conséquences

### Positives

- Le catalogue se prépare et se livre à part du code, dans un dépôt de données simple à relire.
- Tout le code reste libre, vérifiable et reprenable : aucune règle ne se cache dans le dépôt du catalogue.
- Un contributeur travaille entièrement dans le dépôt public, grâce au jeu d’essai.
- Le dépôt du catalogue, fait de données, est simple à relire et à auditer, par tous.

### Négatives

- Deux dépôts à tenir, et un épinglage de version à suivre à chaque évolution du format.
- Le jeu d’essai doit rester assez riche pour couvrir toutes les validations.
- Les fascicules en préparation sont lisibles sur GitHub : la découverte d’un fascicule peut être gâchée avant sa parution.
- Un fork peut reprendre le catalogue sous CC BY-SA 4.0, mais pas le nom ni le logo ([ADR 0017](0017-licences.md)).
- Le contenu ne reçoit pas de contributions publiques directes : une proposition passe par une issue, puis par une saisie dans le dépôt du catalogue.

## Critères de réévaluation

- Une règle de jeu ou une validation apparaît dans la CI du dépôt du catalogue : la déplacer dans le dépôt public.
- Des contributeurs veulent proposer du contenu directement : étudier un circuit de contribution qui ne dévoile rien.
- Le contenu d’un fascicule fuite depuis le dépôt du catalogue : revoir ses accès et ses secrets.
