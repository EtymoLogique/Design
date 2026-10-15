# ADR 0074 — Ouverture de la boutique : médiateur, TVA, territoires et plafond mensuel

- **Statut** : Proposé, à faire valider par le conseil avant la boutique en euros
- **Portée** : après le MVP, avant l’ouverture des achats en euros (M7)
- **Complète** : [ADR 0018](0018-sabliers-et-boutique.md) (revue par territoire, plafond de dépense par défaut), [ADR 0073](0073-boutique-support-durable-et-pieces-comptables.md) (pièces comptables, TVA enregistrée par achat)

## Contexte

Le [dossier d’ouverture de la boutique](../dossier-ouverture-boutique.md) liste ce qui manque avant la vente de sabliers en euros : un médiateur, un régime de TVA, la liste des territoires ouverts et un plafond mensuel par défaut. Il propose chaque valeur. Ce relevé n’est pas un avis juridique : le conseil confirme chaque point.

## Décision

### Médiateur de la consommation

- Le jeu adhère à **CM2C**, médiateur généraliste référencé par la CECMC, ouvert à tout professionnel. Ses coordonnées figurent dans les CGV, sur la page de la boutique et dans les mentions légales.
- À l’adhésion, on confirme par écrit qu’il couvre la vente en ligne de contenus numériques et on relève son coût. Sinon, on choisit un autre médiateur de la liste de la CECMC.
- **Aucun lien vers la plateforme européenne de règlement en ligne des litiges** : elle a fermé le 20 juillet 2025.

### TVA

- Le jeu vend lui-même, au prix TTC affiché, **TVA française à 20 %** pour tous les acheteurs.
- Le serveur cumule les ventes aux particuliers des autres États membres. À **8 000 €** (80 % du seuil de 10 000 €), une alerte prévient le porteur du projet ; au franchissement, il s’inscrit au guichet unique OSS et la TVA du pays de l’acheteur s’applique à partir de la vente qui dépasse le seuil.
- Repli, si l’OSS pèse plus que la commission : un revendeur officiel (« merchant of record »), par un ADR qui remplace celui-ci.

### Territoires

- **La boutique ouvre dans tous les pays**, y compris la Belgique, les Pays-Bas et hors de l’Union. C’est une décision du porteur du projet, prise contre la prudence du relevé : l’[ADR 0018](0018-sabliers-et-boutique.md) demandait une revue juridique par territoire avant l’ouverture.
- Le **risque assumé** : la Belgique a jugé en 2018 que des coffres à butin payants relèvent des jeux de hasard (amende et peine de prison possibles) ; aux Pays-Bas, le Conseil d’État a annulé en 2022 la sanction contre un éditeur, mais la question reste politique. Un sablier n’est pas un tirage ; le conseil confirme qu’il n’est pas un coffre à butin.
- Le **blocage par territoire reste** : le pays se déduit côté serveur, un territoire fermé voit la boutique verrouillée sans effet sur l’échange d’encre ni sur le reste du jeu. Il sert à fermer un pays dès que le conseil, un régulateur ou un magasin d’applications le demande.
- La liste des territoires fermés est un paramètre versionné ([ADR 0009](0009-contenu-et-equilibrage.md)), **vide au lancement**, et figure dans les CGV.
- **TVA hors Union** : la TVA française de 20 % ne vaut que pour les acheteurs de l’Union. Pour un acheteur hors Union, la TVA ou la taxe locale (par exemple la taxe sur les services numériques) peut exiger une inscription dans son pays : le conseil et l’expert-comptable fixent, pays par pays, ce que le prix TTC affiché devient.

### Plafond mensuel par défaut

- **10 € TTC par mois et par compte** sur le web, modifiable par le titulaire du moyen de paiement. L’écran d’achat affiche le montant déjà dépensé ce mois-ci.
- Sur les magasins d’applications, le contrôle parental de la plateforme s’applique.

### Accessibilité

- Le jeu se déclare **microentreprise** au sens de l’European Accessibility Act, hypothèse que le conseil confirme. Il tient malgré tout les critères de l’identité visuelle sur l’écran d’achat (contraste 4,5:1, cibles de 44 px, couleur jamais seul signal).

## Options envisagées

### Revendeur officiel dès le départ

Reporté : commission plus forte, CGV et médiateur à réécrire, pour un seuil de TVA probablement non atteint la première année.

### Fermer la Belgique, les Pays-Bas et le hors Union au lancement

Écarté par le porteur du projet, qui ouvre partout. C’était l’option prudente : elle évitait le risque belge et les taxes locales, au prix d’un marché plus étroit.

### Plafond mensuel plus haut ou absent

Écarté : le panier moyen est de 3,50 € TTC ; 10 € couvre l’usage normal et borne la dépense d’un mineur.

## Conséquences

### Positives

- Chaque point du dossier a une valeur, le conseil n’a plus qu’à valider.
- Le revenu reste net de toute TVA étrangère tant que le seuil n’est pas franchi.

### Négatives

- Un risque pénal et administratif en Belgique, et des obligations fiscales hors Union, sans avis du conseil.
- Le suivi du seuil de TVA est un compteur de plus côté serveur.

## Critères de réévaluation

- Le conseil, un régulateur ou un magasin d’applications refuse le médiateur, un territoire ou le plafond : fermer le territoire concerné.
- Les ventes hors France approchent 8 000 € par an.
- Le coût d’adhésion au médiateur dépasse 1 % du chiffre d’affaires de la boutique.
- Évolution de la loi belge ou néerlandaise sur les coffres à butin.
