# ADR 0085 — Réseau instable : la même commande se renvoie, la table se prépare hors connexion

- **Statut** : Proposé
- **Portée** : MVP, dépôt App (PWA et API) et dépôt design
- **Remplace partiellement** : [ADR 0041](0041-hors-connexion-consultation-seule.md) (pose, déplacement et retrait d’une brique interdits hors connexion ; « aucun rejeu côté client » précisé)
- **Complète** : [ADR 0007](0007-etat-et-economie-autoritaires.md) (le serveur fait foi, idempotence par clé de commande)

## Contexte

Le propriétaire joue surtout dans le train : réseau lent, pertes de paquets, coupures de quelques secondes dans les tunnels. L’[ADR 0041](0041-hors-connexion-consultation-seule.md) prévoit d’être rouvert « si les tests montrent que l’attente du réseau gêne le jeu » : c’est le cas.

Trois défauts en ressortent :

- une requête n’a aucun délai maximal et n’est jamais renvoyée : une coupure d’une seconde fait échouer une fusion, et le joueur doit recommencer son geste ;
- le navigateur se dit « en ligne » dès qu’il voit une antenne, même si rien ne passe : la PWA ne sait pas dire au joueur que le réseau est instable ;
- hors connexion, la table est inerte, alors que poser une brique ne commande rien au serveur : la table en cours est déjà gardée en local (ADR 0041, « table déjà saisie »).

Côté serveur, une commande déjà traitée est retrouvée par sa clé **avant** le contrôle de `version_etat` : la renvoyer rend la même réponse, sans second effet. Une fusion sans découverte n’est pas enregistrée, mais elle ne change rien : la renvoyer la juge de nouveau, à l’identique.

## Décision

### La même commande, renvoyée tant que le joueur attend

- Chaque geste du joueur (fusionner, ouvrir un pli, utiliser ou échanger des sabliers, acheter un indice, récupérer une récompense, répondre à un ricochet) tire **une** clé de commande. Tant que la réponse n’est pas venue, la PWA renvoie la même commande, avec la même clé et la même `version_etat`.
- Chaque essai s’arrête au bout de **8 s**. Les nouveaux essais suivent un délai croissant (0,5 s, 1 s, 2 s, 4 s, avec une part de hasard), pendant **15 s** au plus.
- Seuls se renvoient une erreur de réseau, un délai dépassé, un `502` ou un `504`, un `503` qui ne vient pas de l’API, et un `429` dont l’attente demandée ne dépasse pas 5 s. Jamais un `400`, `401`, `403` ni `409` : ce sont des réponses du serveur.
- Au-delà, la PWA dit que la connexion s’est perdue, garde la table et propose « Réessayer », qui renvoie **la même clé**.
- Ce renvoi n’est pas un rejeu au sens de l’ADR 0041 : rien n’est gardé pour plus tard, rien ne part sans que le joueur attende la réponse, et le serveur ne voit qu’une commande. **Aucune file** : une commande qui n’a pas abouti n’est jamais envoyée après coup, à la reconnexion ou au lancement suivant.

### Trois états du réseau

- **En ligne** ; **réseau instable** dès qu’un essai échoue ou qu’une réponse met plus de 3 s ; **hors ligne** quand le navigateur se dit hors ligne depuis 2 s, ou quand une commande a épuisé ses essais.
- Hors ligne, la PWA interroge `/sante` à intervalle croissant (2 s, puis jusqu’à 30 s) et revient en ligne à la première réponse.
- Au retour du réseau, et au retour de la PWA au premier plan, l’état du joueur se resynchronise de lui-même (`GET /etat`).
- L’état se lit dans l’en-tête de chaque écran : une pastille avec une icône **et** un texte (« Réseau instable », « Hors ligne »), jamais la couleur seule, puis « De retour en ligne » pendant 2 s. Hors ligne, elle dit depuis quand la partie est à jour.

### La table se prépare hors connexion

- Hors connexion, le joueur **pose, déplace et retire** des briques, ouvre la réserve et la parcourt. La table reste gardée en local.
- « Fusionner », les plis, les sabliers, la boutique, les indices, les récompenses et les ricochets restent inertes : toute mutation exige le réseau, comme le prévoit l’ADR 0041.
- Le reste de l’ADR 0041 demeure : consultation du codex, aucune file, aucun résultat local, `409` suivi d’une resynchronisation.

### L’attente se voit à l’endroit du geste

- Moins d’1 s : seule la prévisualisation (« Vérification de votre hypothèse… »).
- Après 1 s : les briques de la table se rapprochent sans s’emboîter ; pendant un pli, la cire continue de chauffer. Rien n’est annoncé avant la réponse du serveur.
- Après 4 s : « Le réseau est lent, nous insistons… ».
- Une erreur de réseau ne secoue jamais la table et ne prend pas l’apparence d’un échec de jeu : icône hors ligne, « Rien n’est perdu », bouton « Réessayer ».

## Options envisagées

### Garder l’ADR 0041 tel quel

Rejetée : chaque micro-coupure coûte un geste, et la table reste inerte dans un tunnel où le joueur pourrait préparer sa prochaine hypothèse.

### Une file de commandes rejouées à la reconnexion

Rejetée, comme dans l’ADR 0041 : le résultat d’une fusion est la récompense du joueur, et il arriverait hors de son geste ; une file demande aussi de réconcilier des commandes périmées.

### Une nouvelle clé à chaque essai

Rejetée : si la première réponse s’est perdue en route, la seconde commande serait refusée en `409` ; le joueur verrait une erreur pour une fusion pourtant réussie.

### Fusionner hors connexion avec la prévisualisation locale

Rejetée : le serveur fait foi (ADR 0007) ; la prévisualisation ne sait ni la réserve confirmée, ni la consommation, ni la récompense.

## Conséquences

### Positives

- Une coupure de moins de 15 s ne se voit plus : la fusion aboutit.
- Le joueur prépare sa table dans un tunnel et fusionne dès que le réseau revient.
- Le joueur sait si le réseau est instable, plutôt que de croire le jeu en panne.

### Négatives

- Une erreur définitive s’annonce plus tard (jusqu’à 15 s) qu’aujourd’hui.
- La PWA garde des clés de commande en mémoire le temps d’un geste, et deux états de plus (instable, retour en ligne) sont à tester.

## Critères de réévaluation

- Les tests montrent que 15 s d’attente lassent avant d’aboutir : raccourcir la fenêtre.
- Les joueurs réclament de fusionner hors connexion : rouvrir la question de la file par un nouvel ADR.
