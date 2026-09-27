# ADR 0030 — Réserve de départ déclarée par chaque fascicule

- **Statut** : Proposé
- **Portée** : couche ludique du catalogue, manifeste de fascicule, données du joueur
- **Précise** : [ADR 0012](0012-briques-rationnees.md) (réserve de départ), [ADR 0004](0004-progression-atteignable.md) (graines comme sources), [ADR 0009](0009-contenu-et-equilibrage.md) (paramètres d'équilibrage)
- **Complète** : [ADR 0027](0027-catalogue-statique-et-donnees-joueur.md) (contenu du manifeste), le [modèle de données](../modele-donnees.md) (couche ludique et données du joueur)

## Contexte

L'[ADR 0012](0012-briques-rationnees.md) donne au joueur une **réserve de départ** : quelques briques, en quelques exemplaires, qui permettent les premières découvertes sans ouvrir de pli. Le game-design en fait des **graines** et fixe une règle d'amorçage : au moins trois recettes immédiatement réalisables, avec assez d'exemplaires pour les réaliser toutes.

Rien ne dit pourtant où ces graines sont déclarées, ni quand le joueur les reçoit. Le modèle de données les range dans la couche ludique sans leur donner de forme. L'application ne peut donc pas accorder la réserve de départ, et le validateur ne peut pas vérifier la règle d'amorçage.

Les fascicules paraissent chaque mois ([ADR 0022](0022-fascicules-de-20-a-30-mots.md)) et chacun est autonome : il déclare toutes les unités dont il a besoin. Un joueur qui arrive au troisième fascicule doit pouvoir commencer comme celui qui était là au premier.

## Décision

### Les graines appartiennent au fascicule

- Un fascicule déclare ses **graines** : une liste de briques, chacune avec un nombre d'exemplaires de 1 à 5 (le plafond de l'[ADR 0012](0012-briques-rationnees.md)).
- Une graine est une brique d'une unité **déclarée par ce fascicule**, comme résultat ou comme ingrédient.
- Une brique n'est semée qu'une fois par fascicule.
- Les graines sont un paramètre d'équilibrage ([ADR 0009](0009-contenu-et-equilibrage.md)) : les changer ne touche à aucun fait linguistique.

### Le validateur vérifie l'amorçage

En avertissement :

- avec ses seules graines, un fascicule permet au moins **trois recettes**, toutes réalisables **ensemble** avec les exemplaires semés ;
- le premier fascicule (le plus petit numéro) déclare des graines.

Un fascicule suivant peut ne rien semer : il s'appuie alors sur les plis et sur les briques déjà acquises.

### Le manifeste les publie

Le manifeste de fascicule porte `graines` : `brique_id` et `exemplaires`, triés par brique. Comme les recettes, elles sont lisibles dès la parution ([ADR 0027](0027-catalogue-statique-et-donnees-joueur.md)).

### Le serveur les accorde une fois par fascicule

- Le serveur accorde à un joueur les graines de **chaque fascicule paru** qu'il n'a pas encore reçues, à sa première synchronisation où ce fascicule est paru, dans une seule transaction.
- Chaque exemplaire accordé est un `mouvement` de motif `depart`. Une table `graine_accordee` (joueur, fascicule) garantit qu'un fascicule ne sème qu'une fois, même si la commande est rejouée.
- La réserve reste plafonnée à 5 exemplaires par brique. Au-delà, rien n'est accordé pour cette brique. Les graines ne rapportent jamais d'encre.
- Accorder des graines incrémente la `version_etat` du joueur, comme toute mutation.

## Options envisagées

### Une réserve de départ globale, dans la configuration de l'application

Plus simple, mais l'équilibrage sortirait du catalogue et de sa validation. Chaque changement demanderait un déploiement de l'API. Écarté.

### Des graines accordées seulement à la création du profil

Un joueur arrivé avant un fascicule ne recevrait jamais les graines de ce fascicule. Chaque fascicule devrait alors se jouer sans amorçage pour les anciens, avec amorçage pour les nouveaux. Écarté : le semis par fascicule traite tous les joueurs de la même façon.

### Des graines sous forme de pli de jalon déterministe

Les plis de jalon ([ADR 0012](0012-briques-rationnees.md)) donnent un contenu déterministe, mais ils coûtent un geste au joueur et n'existent pas encore. Écarté pour la réserve de départ ; les jalons restent l'outil des récompenses suivantes.

## Conséquences

### Positives

- La règle d'amorçage est vérifiée à chaque validation du catalogue.
- Chaque fascicule reste autonome, pour les nouveaux comme pour les anciens joueurs.
- L'équilibrage de la réserve de départ se change sans déploiement de l'API.

### Négatives

- Un joueur ancien reçoit des exemplaires gratuits à chaque fascicule qui sème : les graines d'un fascicule suivant doivent rester modestes.
- Une table et un motif de mouvement de plus dans les données du joueur.

## Critères de réévaluation

- Les graines des fascicules suivants déséquilibrent le rythme des joueurs anciens (plus de découvertes sans pli que prévu par la simulation de l'[ADR 0012](0012-briques-rationnees.md)).
- Les jalons et leurs plis déterministes suffisent à amorcer chaque fascicule.
