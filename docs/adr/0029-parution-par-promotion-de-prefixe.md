# ADR 0029 — Parution par promotion de préfixe, dans un seul bucket

- **Statut** : Proposé
- **Portée** : stockage du catalogue publié, droits sur l'Object Storage et tâche de parution
- **Remplace partiellement** :
  - [ADR 0025](0025-medias-statiques-et-publication-programmee.md), pour les fichiers d'un fascicule à venir, déposés à l'avance dans le stockage public, et pour l'horloge de l'API qui décide seule de la bascule ;
  - [ADR 0027](0027-catalogue-statique-et-donnees-joueur.md), pour le dépôt à l'avance protégé par des chemins qui ne se devinent pas.
- **Complète** : [ADR 0024](0024-architecture-logicielle-et-hebergement.md) (hébergement), [ADR 0028](0028-gitflow-des-depots.md) (un bucket par environnement)

## Contexte

L'[ADR 0027](0027-catalogue-statique-et-donnees-joueur.md) publie le catalogue dans l'Object Storage, lisible par nom et sans listage. Un fascicule prêt y est déposé à l'avance ([ADR 0025](0025-medias-statiques-et-publication-programmee.md)). Jusqu'à sa date, seul le secret de ses chemins le protège : un chemin en hash de contenu ne se devine pas, mais il suffit qu'il fuite, par un journal, une capture ou une erreur de CI.

L'[ADR 0025](0025-medias-statiques-et-publication-programmee.md) le reconnaît : « un nom opaque n'est pas un contrôle d'accès ». Il prévoit de ne déposer les médias qu'à la date d'effet si des fichiers à venir circulaient. Ce qu'il faut garantir, c'est qu'**aucun fichier d'un fascicule ne se lise avant sa parution**, même par qui connaît son chemin. Et cela sans deuxième bucket, que l'[ADR 0027](0027-catalogue-statique-et-donnees-joueur.md) a justement supprimé.

Chez Scaleway, une politique de bucket au format `2023-04-17` ne contient que des autorisations : tout ce qu'elle n'autorise pas est refusé, et une règle `Deny` n'a aucun effet. Elle peut viser un préfixe (`bucket/prefixe/*`) et borner un listing par la condition `s3:prefix`. Elle prime sur les ACL d'objet, et aucune condition ne porte sur les tags d'un objet.

## Décision

### Un bucket, deux préfixes

| Préfixe | Contenu | Lecture anonyme |
|---|---|---|
| `public/` | Index global, manifestes, cartes, recettes, exclusions et médias des fascicules **parus** | Par nom, sans listage |
| `staging/` | Les mêmes fichiers, pour les fascicules **à venir**, et le calendrier des parutions | Jamais |

- Les chemins sous chaque préfixe gardent la forme de l'[ADR 0027](0027-catalogue-statique-et-donnees-joueur.md) : `index.json`, puis manifestes et ressources en hash de contenu.
- Le **calendrier des parutions**, `staging/parutions.json`, liste les fascicules en attente, leur date d'effet en UTC et le chemin de leur manifeste. Comme l'index, il ne cite que des manifestes : seul le manifeste cite les ressources de son fascicule (cartes, recettes, exclusions, médias). Il remplace le manifeste serveur daté de l'[ADR 0025](0025-medias-statiques-et-publication-programmee.md), qui n'a plus de place dans un artefact unique.

### Une liste blanche, fixée par l'infrastructure

La politique du bucket est écrite et appliquée par le dépôt d'infrastructure. Personne d'autre ne la modifie. Elle n'autorise que ceci :

| Identité | Droits |
|---|---|
| Anonyme (client, API) | `GetObject` sur `public/*` |
| CI du dépôt privé du catalogue | Dépôt, lecture et suppression d'objets sous `staging/*`, et listing borné à `staging/` |
| Tâche de parution | `GetObject` sur `staging/*`, `GetObject` et `PutObject` sur `public/*` |
| Infrastructure | Gestion du bucket et lecture de sa configuration, comme avant |

- Tout le reste est refusé, y compris le listing anonyme et toute écriture dans `public/` par la CI du catalogue. Même une CI mal réglée ne peut donc rien publier en avance.
- La tâche de parution a sa propre identité et sa propre clé, rangée dans Secret Manager. Elle ne peut modifier ni la politique, ni les droits, ni un autre bucket.
- Aucune ACL d'objet, aucun tag et aucune règle datée ne servent au contrôle d'accès.

### La parution promeut les fichiers, puis l'index

À chaque date d'effet, la tâche planifiée de l'[ADR 0025](0025-medias-statiques-et-publication-programmee.md) :

1. lit le calendrier et retient les fascicules dont la date est passée ;
2. lit le manifeste de chaque fascicule dans `staging/` et copie vers `public/`, sous le même chemin relatif, les ressources qu'il cite, puis le manifeste lui-même. Un fichier déjà présent dans `public/`, par exemple une brique reprise, n'est pas recopié : son contenu est identique ;
3. applique les migrations de progression ;
4. écrit en dernier le nouveau `public/index.json`.

- Tant que l'index n'est pas écrit, rien ne paraît : un fichier promu sans index n'est lisible que par qui connaît déjà son chemin, le jour même.
- La tâche est idempotente : relancée, elle ne recopie rien et réécrit le même index.
- Reporter ou annuler un fascicule revient à changer ou retirer sa ligne du calendrier. Revenir en arrière après la parution consiste à restaurer une version antérieure de l'index, que le versionnement du bucket conserve.
- Les fichiers restent dans `staging/` après la parution. C'est la CI du catalogue qui les supprime.

### L'API suit l'index publié

- L'API lit le catalogue comme le client, sans clé, dans `public/`. Elle ne voit un fascicule qu'une fois l'index publié.
- Cela remplace, dans l'[ADR 0025](0025-medias-statiques-et-publication-programmee.md), la bascule décidée par la seule horloge de l'API. La date d'effet reste celle du calendrier, mais c'est la tâche de parution qui l'exécute. Si elle échoue, le fascicule ne paraît ni pour le client ni pour l'API.
- L'horloge du serveur fait toujours foi pour l'énergie, les sabliers et les plafonds ([ADR 0024](0024-architecture-logicielle-et-hebergement.md)).

## Options envisagées

### Chemins en hash de contenu seulement (ADR 0027)

Aucune infrastructure en plus, mais un chemin qui fuite livre le fichier avant sa date. Écarté : c'est justement le risque à fermer.

### ACL d'objet ou tags, changés par la tâche à la parution

Chez Scaleway, la politique de bucket prime sur les ACL d'objet, et aucune condition ne porte sur les tags. Écarté : ce ne serait pas un contrôle d'accès.

### Politique réécrite par la tâche à chaque parution

La tâche devrait pouvoir écrire la politique, donc s'accorder n'importe quel droit, et l'infrastructure verrait chaque mois une dérive. Écarté.

### Règles datées dans la politique (`aws:CurrentTime`)

Aucune tâche à écrire, mais les dates de parution et les préfixes des fascicules seraient écrits dans le dépôt public de l'infrastructure. Écarté : c'est l'information qu'on protège.

### Un bucket privé pour les fascicules à venir

Même garantie, mais un deuxième bucket, que l'[ADR 0027](0027-catalogue-statique-et-donnees-joueur.md) a supprimé. Écarté : un préfixe suffit.

## Conséquences

### Positives

- Aucun fichier d'un fascicule à venir ne se lit avant sa date, même par qui connaît son chemin. Le critère de réévaluation de l'[ADR 0025](0025-medias-statiques-et-publication-programmee.md) sur les médias en circulation est levé.
- Un seul bucket, une seule politique, fixe et relue avec l'infrastructure.
- Ni la CI du catalogue ni une erreur de chemin ne peuvent publier en avance : seule la tâche de parution écrit dans `public/`.

### Négatives

- La tâche de parution devient le seul chemin de publication. Si elle échoue, rien ne paraît et il faut une alerte ; si elle dépasse sa durée maximale, la parution est retardée.
- La promotion recopie chaque fichier d'un fascicule : un peu de stockage et de temps en plus à chaque parution.
- La tâche détient une clé qui peut écrire dans `public/`. Une fuite de cette clé permettrait de publier en avance ou de remplacer l'index.

## Critères de réévaluation

- Une parution dépasse 5 minutes ou échoue deux fois de suite : revoir la copie, par exemple en ne promouvant que l'index et les fichiers absents de `public/`.
- Scaleway propose une condition sur les tags d'objet ou une identité de charge de travail : réétudier le marquage des objets, ou supprimer la clé statique de la tâche.
- Le besoin de secret s'étend après la parution (recettes, légendaires) : étudier un préfixe lisible par la seule API.
