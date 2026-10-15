# ADR 0070 — Comptes facultatifs, connexion par lien et code envoyés par e-mail

- **Statut** : Proposé, durée de conservation des comptes remplacée par l’[ADR 0071](0071-duree-de-conservation-des-comptes-et-des-plis.md) : un compte inactif depuis 3 ans est effacé ; le reste de la section « Supprimer, purger » reste en vigueur
- **Remplace partiellement** : [ADR 0007](0007-etat-et-economie-autoritaires.md), pour le compte, la récupération et le multi-appareil, qu’il réservait à une décision séparée ; [ADR 0024](0024-architecture-logicielle-et-hebergement.md), section « Identité et sécurité », pour le profil invité (un joueur peut désormais avoir plusieurs appareils et un compte)
- **Complète** : [ADR 0010](0010-observabilite-et-vie-privee.md) (aucune adresse dans les journaux ni dans la télémétrie), [ADR 0018](0018-sabliers-et-boutique.md) (le compte sera le prérequis des achats en euros)

## Contexte

Chaque installation de la PWA est un profil invité anonyme ([ADR 0007](0007-etat-et-economie-autoritaires.md)) : un identifiant d’installation et un secret de 256 bits, dont le serveur ne garde que l’empreinte. Si le stockage local disparaît, la progression est perdue. Un joueur ne peut ni changer d’appareil ni en utiliser deux.

Trois besoins arrivent ensemble :

- **récupérer sa progression** et jouer sur plusieurs appareils ;
- **une vue compétitive**, possible plus tard, qui demande un nom affichable.

L’[ADR 0007](0007-etat-et-economie-autoritaires.md) écarte un compte obligatoire au premier lancement : la friction et la collecte de données ne servent pas le jeu. Il demande un ADR dédié avant toute collecte d’identité.

## Décision

### Le compte est facultatif, l’invité reste le point de départ

- Le premier lancement crée toujours un profil invité, comme aujourd’hui. On joue sans jamais donner d’adresse.
- Le joueur crée un compte en **rattachant une adresse électronique à son profil invité** : la progression devient celle du compte, sans copie ni perte.
- Le compte ne donne aucun avantage de jeu : ni brique, ni encre, ni pli.

### Relancer sans contraindre

L’invité est relancé par une carte « Sauvegardez votre progression », toujours refermable et jamais bloquante :

- à la **3ᵉ découverte** ;
- à **chaque jalon atteint** ;
- sinon, **tous les 7 jours de jeu** depuis la dernière relance refermée.

Les Réglages gardent en permanence une entrée « Sauvegarder ma progression ». La relance n’est jamais un dialogue ; elle n’interrompt ni une fusion ni le rituel d’un pli.

### Connexion sans mot de passe : un lien et un code dans le même e-mail

- Le joueur saisit son adresse. Le serveur envoie **un e-mail** qui contient un **lien** et un **code à 6 chiffres**, valables **15 minutes** et à **usage unique**.
- Le lien sert dans un navigateur. Le code sert dans la PWA installée, en particulier sur iOS : le lien s’y ouvre dans Safari, dont le stockage est séparé de celui de la PWA.
- Le lien mène à la PWA, et seule la PWA confirme. Une visite automatique du lien par une messagerie ne consomme donc jamais le code.
- **5 essais** au plus par code ; au-delà, il faut en demander un nouveau. Un nouveau code annule le précédent.
- La réponse à une demande est **identique**, que l’adresse ait un compte ou non : on ne peut pas savoir qui joue.
- Les demandes sont limitées par adresse réseau, par profil et par adresse électronique (par exemple 3 toutes les 15 minutes).

### L’adresse n’est jamais conservée en clair

- Le serveur ne garde qu’une **empreinte HMAC-SHA256** de l’adresse normalisée (rognée, minuscules), avec une clé secrète dédiée, distincte de celle des jetons de session.
- L’adresse en clair n’existe que le temps de l’envoi. Elle n’apparaît ni en base, ni dans les journaux, ni dans la télémétrie ([ADR 0010](0010-observabilite-et-vie-privee.md)), pas plus que le code ou le lien.
- Conséquence assumée : le jeu ne peut écrire à un joueur qu’au moment où il demande à se connecter. Ni lettre d’information, ni annonce par e-mail.
- L’empreinte reste une donnée pseudonyme : la page des données collectées l’explique avant toute diffusion publique.

### Un joueur, plusieurs appareils

- Chaque appareil garde son identifiant d’installation et son secret. Un joueur peut en avoir plusieurs.
- Se connecter à un compte rattache l’appareil au joueur du compte.
- **Si l’appareil porte une partie invitée avec une progression, le compte l’emporte** : un avertissement explicite précède le remplacement, et la partie invitée est effacée, jamais fusionnée. Additionner deux économies (encre, plis, garanties, sabliers) ouvrirait des abus.
- Se déconnecter détache l’appareil, qui repart d’un profil invité neuf.

### Le pseudo

- Chaque compte a un **pseudo unique** : 3 à 24 caractères, lettres minuscules sans accent, chiffres et traits d’union isolés.
- Par défaut, c’est la partie locale de l’adresse, assainie et tronquée à 16 caractères, suivie de `-` et de 4 chiffres : *camille.martin@exemple.fr* devient `camille-martin-4821`.
- Cette partie contient souvent un vrai nom : le pseudo par défaut reste **privé** tant que le joueur ne l’a ni confirmé ni modifié. Aucun écran ne le montre à un autre joueur avant cela.

### Supprimer, purger

- « Supprimer mon compte » efface le joueur et toute sa progression, sur tous ses appareils, après une confirmation explicite.
- Un **invité sans compte, inactif depuis 6 mois**, est effacé par les purges quotidiennes. La carte de relance le dit. Un compte n’est jamais effacé automatiquement.
- Les demandes de connexion sont effacées au bout d’un jour.

### Envoi des e-mails

- L’envoi passe par **Scaleway Transactional Email**, offre *Essential* (à l’usage), depuis `noreply@etymologique.fr` en production et `noreply@horsprod.etymologique.fr` hors production.
- Le domaine d’envoi est authentifié par SPF, DKIM et DMARC. La zone étant tenue hors de Scaleway, ces enregistrements se posent à la main.
- L’e-mail est en français, sobre, sans image distante ni lien de suivi. Il dit ce qu’il faut faire si l’on n’a rien demandé : rien.

## Options envisagées

### Compte obligatoire au premier lancement

Écarté, comme dans l’[ADR 0007](0007-etat-et-economie-autoritaires.md) : un e-mail en retard ou classé en indésirable empêcherait de jouer, et la première session doit mener à une fusion sans détour.

### Mot de passe

Écarté : un secret de plus à retenir, à réinitialiser par e-mail de toute façon, et à protéger contre la réutilisation et le bourrage d’identifiants.

### Lien magique seul

Écarté : sur iOS, le lien s’ouvre dans Safari et pas dans la PWA installée. La connexion aboutirait dans le mauvais stockage.

### Connexion par Google ou Apple

Écartée pour l’instant : un tiers apprendrait qui joue, et il faudrait deux intégrations pour couvrir tout le monde. À reconsidérer si les e-mails n’arrivent pas.

### Adresse conservée, chiffrée au repos

Écartée : elle permettrait d’écrire aux joueurs, mais ferait de la base une liste d’adresses à protéger, à exporter et à effacer sur demande. Le jeu n’en a pas besoin.

### Fusionner la partie invitée et le compte

Écarté : additionner réserves, encre, sabliers et compteurs de garanties fausserait l’économie autoritaire ([ADR 0007](0007-etat-et-economie-autoritaires.md)).

### Pseudo tiré au hasard

Écarté à la demande du produit : un pseudo tiré de l’adresse est plus reconnaissable pour le joueur. Le risque d’exposer un nom est traité en gardant le pseudo privé jusqu’à confirmation.

## Conséquences

### Positives

- La progression survit à un changement d’appareil, et on peut jouer sur plusieurs appareils.
- Aucune adresse en clair : une fuite de la base ne livre aucun moyen de contacter les joueurs.
- Aucun mot de passe à protéger ni à réinitialiser.
- Une future vue compétitive dispose d’une identité.
- Le premier lancement reste sans friction.

### Négatives

- Un nouveau service, l’envoi d’e-mails, avec sa réputation de domaine à entretenir et un coût au-delà de l’offre gratuite.
- Le jeu ne peut pas écrire à ses joueurs en dehors d’une connexion.
- Perdre la clé HMAC rend tous les comptes introuvables : elle se protège comme la clé des jetons, et sa rotation demanderait une double empreinte.
- Un invité inactif 6 mois perd sa partie.
- Plus d’écrans : connexion, relance, compte dans les Réglages.

## Critères de réévaluation

- Plus de 5 % des demandes de code sans confirmation dans les 15 minutes : vérifier la délivrabilité, envisager un fournisseur d’identité tiers.
- Moins de 10 % des joueurs actifs depuis 30 jours ont un compte : revoir les moments de relance.
- Ouverture d’une vue compétitive : règles de modération des pseudos ([ADR 0075](0075-moderation-des-pseudos-et-obligations-dsa.md)).
