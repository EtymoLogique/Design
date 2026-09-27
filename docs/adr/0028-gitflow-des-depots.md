# ADR 0028 — Gitflow des dépôts : master pour la prod, develop pour la hors-prod

- **Statut** : Proposé
- **Portée** : branches, fusions et déploiements des dépôts design, content, app et infra
- **Remplace partiellement** : [ADR 0024](0024-architecture-logicielle-et-hebergement.md), pour la préversion par branche, réservée désormais au dossier de conception
- **Précise** : [ADR 0026](0026-depot-prive-du-catalogue-sans-logique.md) (le code public est réparti entre les dépôts design, app et infra ; le dépôt content reste privé et sans logique)
- **Complète** : [ADR 0025](0025-medias-statiques-et-publication-programmee.md) (la parution programmée s’applique dans chaque environnement)

## Contexte

Le projet vit dans quatre dépôts :

| Dépôt | Contenu |
|---|---|
| design | Dossier de conception : vision, ADR, identité, maquettes, publié sur GitHub Pages |
| content | Catalogue brut, privé, sans logique ([ADR 0026](0026-depot-prive-du-catalogue-sans-logique.md)) |
| app | PWA et API ([ADR 0024](0024-architecture-logicielle-et-hebergement.md)) |
| infra | Infrastructure en code, OpenTofu ou Terraform, chez Scaleway |

L’[ADR 0024](0024-architecture-logicielle-et-hebergement.md) dit seulement que « chaque branche peut avoir sa préversion ». Rien ne fixe quelles branches existent, lesquelles déploient, ni vers quel environnement. Or une préversion par branche de l’application demande une base, un bucket et une API par branche : c’est coûteux, et le catalogue privé risque d’y fuiter.

Deux environnements suffisent au MVP : **hors-prod**, pour éprouver une livraison, et **prod**, celle des joueurs. Chaque dépôt n’a pas besoin du même circuit : le dossier de conception n’a pas de hors-prod, l’application a besoin de versions identifiables.

## Décision

### Branches et environnements par dépôt

| Dépôt | Branches | Hors-prod | Prod |
|---|---|---|---|
| design | `master`, `feature/*` | Chaque branche est publiée sous `/branches/<nom>/` sur GitHub Pages | `master`, publié à la racine de GitHub Pages |
| content | `master`, `develop`, `feature/*` | `develop` publie le catalogue en hors-prod | `master` publie le catalogue en prod |
| app | `master`, `develop`, `feature/*`, tags `vX.Y.Z` | `develop` déploie l’application en hors-prod | Un tag `vX.Y.Z` posé sur `master` déploie l’application en prod |
| infra | `master`, `develop`, `feature/*` | `develop` applique l’infra hors-prod | `master` applique l’infra de prod |

### Règles communes

- Une branche de feature se tire de `develop`, ou de `master` pour design. Elle revient dans sa branche d’origine par une pull request, CI verte.
- `develop` rejoint `master` par une pull request. `master` ne reçoit rien d’autre, et ne contient donc que ce qui est déjà passé en hors-prod.
- `master` et `develop` sont protégées : ni push direct, ni réécriture d’historique.
- Pas de branche `release/*` ni `hotfix/*`. Un correctif urgent suit le même chemin : feature, `develop`, hors-prod, `master`, et, pour app, un tag.
- Seul design publie ses branches de feature. Une branche de feature de content, app ou infra ne déploie rien : sa CI valide et teste, sans publier.

### App : le tag fait la prod

- Les tags suivent SemVer, `vX.Y.Z`, et se posent sur un commit de `master`.
- Une fusion sur `master` sans tag ne déploie rien. Seul le tag déclenche la prod.
- L’image publiée en prod est reconstruite depuis le commit tagué et porte le numéro du tag. Revenir en arrière, c’est redéployer le tag précédent.

### Content : la date fait la parution

- `master` et `develop` publient le même format d’artefact ([ADR 0027](0027-catalogue-statique-et-donnees-joueur.md)), chacun dans l’Object Storage de son environnement.
- La parution programmée ([ADR 0025](0025-medias-statiques-et-publication-programmee.md)) s’applique dans les deux environnements : fusionner un fascicule sur `master` ne le fait pas paraître avant sa date.

### Infra : un code, deux inventaires

- Les modules sont communs aux deux environnements. Aucun fichier de code n’est propre à la prod ou à la hors-prod.
- Chaque environnement a son **inventaire** : variables, dimensionnement, noms, domaines, et son état distant, séparé de l’autre.
- La branche choisit l’inventaire : `develop` applique l’inventaire hors-prod, `master` celui de prod. Une modification de module passe donc en hors-prod avant la prod ; une modification d’inventaire de prod passe par `develop` sans effet en hors-prod.
- Sur une pull request, la CI produit le plan de l’environnement cible, sans l’appliquer.

## Options envisagées

### Trunk-based : une seule branche master pour tous les dépôts

Plus simple, mais chaque fusion part en prod. Le catalogue et l’application n’auraient plus d’endroit où s’éprouver avant les joueurs. Retenu pour design seul, qui n’a pas de hors-prod.

### Gitflow complet, avec branches release et hotfix

Utile quand plusieurs versions vivent en parallèle. Le MVP n’en a qu’une en prod : ces branches doubleraient les fusions et les risques d’oubli (un hotfix non reporté dans `develop`). Écarté.

### Préversion par branche de feature pour tous les dépôts

Promise par l’[ADR 0024](0024-architecture-logicielle-et-hebergement.md). Pour app et infra, elle multiplie bases, buckets et coûts ; pour content, elle expose des fascicules non parus sur des adresses de plus. Écarté hors design.

### App : master déploie la prod à chaque fusion, sans tag

Plus direct, mais une version en prod n’aurait pas de numéro stable, et le retour en arrière passerait par une nouvelle fusion. Écarté.

### Infra : un dossier de code par environnement

Chaque environnement évoluerait librement, mais la hors-prod cesserait d’éprouver le code de la prod. Écarté au profit d’un code unique et de deux inventaires.

## Conséquences

### Positives

- Tout ce qui arrive en prod est d’abord passé en hors-prod, pour les trois dépôts qui déploient.
- Une version de l’application en prod se nomme, se retrouve et se redéploie par son tag.
- La hors-prod éprouve le même code d’infrastructure que la prod ; seules les valeurs changent.
- Deux environnements seulement à payer, surveiller et protéger.

### Négatives

- Une branche de feature d’app ou d’infra ne se voit pas déployée avant sa fusion dans `develop` : les essais concurrents se partagent la hors-prod.
- Un correctif urgent attend le passage par `develop` et la hors-prod.
- `develop` et `master` peuvent diverger si une fusion `develop` → `master` tarde ; il faut la faire souvent.
- Poser un tag reste un geste manuel pour app.

## Critères de réévaluation

- Plus d’une livraison sur quatre de la prod attend qu’un autre essai libère la hors-prod : envisager des préversions éphémères pour app.
- Un incident en prod ne peut pas attendre le passage par `develop` : réintroduire une branche `hotfix/*`.
- Deux versions de l’application doivent vivre en prod en même temps : réintroduire des branches `release/*`.
- Un troisième environnement devient nécessaire (recette éditoriale, démonstration) : ajouter un inventaire et sa branche.
