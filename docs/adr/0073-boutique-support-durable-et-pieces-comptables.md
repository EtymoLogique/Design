# ADR 0073 — Boutique : le reçu du prestataire sert de support durable, le jeu ne conserve aucune adresse

- **Statut** : Proposé, à faire valider par le conseil avant la boutique en euros
- **Portée** : après le MVP, avant l’ouverture des achats en euros
- **Remplace partiellement** : [ADR 0018](0018-sabliers-et-boutique.md), section « Remboursement », pour le moment de la renonciation au droit de rétractation (avant le paiement, et non avant le premier usage)
- **Complète** : [ADR 0070](0070-comptes-facultatifs-par-lien-et-code-e-mail.md) (la promesse « aucune adresse conservée » tient pour les achats), [ADR 0071](0071-duree-de-conservation-des-comptes-et-des-plis.md) (durée de conservation des pièces comptables), [ADR 0010](0010-observabilite-et-vie-privee.md) (aucune adresse ni donnée de paiement dans les journaux)

## Contexte

Vendre du contenu numérique en ligne impose, d’après le relevé juridique du projet (à confirmer par le conseil) :

- une **confirmation du contrat sur un support durable** (code de la consommation, art. L221-13) ;
- pour renoncer au droit de rétractation d’un contenu numérique (art. L221-28), un **accord exprès**, une **renonciation expresse** et cette confirmation ;
- la **conservation des pièces comptables pendant 10 ans** (code de commerce, art. L123-22), et une facture sur demande.

L’[ADR 0070](0070-comptes-facultatifs-par-lien-et-code-e-mail.md) pose que le jeu ne garde jamais l’adresse électronique en clair et ne peut écrire à un joueur qu’au moment où il demande à se connecter. Il renvoie la question des reçus à l’ouverture des achats. L’[ADR 0018](0018-sabliers-et-boutique.md) place la renonciation avant le premier usage d’un sablier acheté, soit après la fourniture du contenu. Ce relevé n’est pas un avis juridique.

## Décision

### Le prestataire de paiement envoie le reçu et garde l’adresse

- Le **prestataire de paiement** reçoit l’adresse de facturation de l’acheteur, envoie le **reçu** et la conserve pour ses propres obligations, **comme responsable de traitement distinct**.
- Le jeu **ne reçoit ni ne conserve** cette adresse. La promesse de l’[ADR 0070](0070-comptes-facultatifs-par-lien-et-code-e-mail.md) reste vraie : le jeu n’écrit à personne en dehors d’une connexion.
- La décision est **conditionnée à l’avis du conseil** : le reçu du prestataire doit valoir confirmation du contrat sur un support durable, et porter la mention de la renonciation. Sans cet avis, la boutique reste fermée.

### Pièces comptables conservées par le jeu

Pour chaque achat, le jeu garde un **enregistrement comptable** : identifiant de transaction du prestataire, lot, montant TTC, TVA, date, pays d’émission de la carte, et la **preuve de la renonciation** (voir ci-dessous). Il ne contient **aucune adresse, aucun nom, aucune donnée de carte**.

- Durée : **10 ans** à compter de la clôture de l’exercice, quelle que soit l’inactivité du compte ([ADR 0071](0071-duree-de-conservation-des-comptes-et-des-plis.md) ne s’applique pas à cet enregistrement).
- « Supprimer mon compte » **détache** l’enregistrement du joueur (plus d’identifiant de joueur ni de pseudo) mais ne l’efface pas : c’est une obligation légale qui prime l’effacement.
- Une **facture sur demande** est établie à partir de l’enregistrement et des nom et adresse que l’acheteur saisit alors ; ils servent à la produire et à l’envoyer, et ne sont pas conservés par le jeu. Le conseil dit si l’exemplaire conservé sans ces champs suffit.

### Renonciation au droit de rétractation : avant le paiement

Le contenu numérique est fourni au **crédit des sabliers**. La renonciation doit donc précéder le paiement, pas le premier usage.

- Sur l’écran d’achat, avant la redirection vers le paiement, le joueur coche une case **non précochée** : il demande l’exécution immédiate et renonce à son droit de rétractation pour le lot affiché.
- Le serveur enregistre la case (achat, version du texte, horodatage) **avant** de créer le paiement, et ne crédite qu’un paiement confirmé ([ADR 0018](0018-sabliers-et-boutique.md)).
- Le **remboursement sous 14 jours d’un lot inutilisé**, prévu par l’[ADR 0018](0018-sabliers-et-boutique.md), reste, mais comme **geste commercial** et non comme droit légal. Le conseil dit s’il faut le garder.

### Si le conseil refuse

Dans cet ordre, par un ADR qui remplace celui-ci :

1. **le jeu envoie lui-même la confirmation** au moment de l’achat, à l’adresse saisie alors, qu’il ne garde que le temps de l’envoi (comme le code de connexion) : le jeu écrirait aux acheteurs, une fois, à leur demande ;
2. **le jeu conserve l’adresse des seuls acheteurs**, chiffrée, pour la durée comptable.

## Options envisagées

### Le jeu conserve l’adresse de tous les joueurs

Écartée : elle ferait de la base une liste d’adresses à protéger, à exporter et à effacer, alors que seuls les acheteurs en ont besoin ([ADR 0070](0070-comptes-facultatifs-par-lien-et-code-e-mail.md)).

### Le jeu conserve l’adresse des seuls acheteurs, chiffrée

Reportée en dernier recours : elle ne tient la promesse de l’[ADR 0070](0070-comptes-facultatifs-par-lien-et-code-e-mail.md) que pour les non-acheteurs, et ajoute une donnée à protéger pendant 10 ans.

### Renonciation avant le premier usage (ADR 0018)

Remplacée : le contenu est déjà fourni au crédit, la renonciation arriverait trop tard.

### Aucune renonciation, rétractation de 14 jours sur tout achat

Écartée : un joueur pourrait dépenser les sabliers puis demander le remboursement.

## Conséquences

### Positives

- Le jeu garde sa promesse : aucune adresse en clair, y compris pour les acheteurs.
- Une fuite de la base ne livre aucune adresse d’acheteur.
- La renonciation est prouvée avant le crédit, sans adresse.

### Négatives

- Le jeu dépend du contenu du reçu du prestataire, qu’il ne maîtrise pas : mention de la renonciation, langue, conservation.
- Les pièces comptables survivent à la suppression du compte, ce que la page des données collectées doit expliquer.
- Un second responsable de traitement apparaît : le prestataire.
- Le remboursement de 14 jours devient une libéralité à garder ou non.

### À écrire dans la politique de confidentialité

- **nouveau destinataire** : le prestataire de paiement, responsable de traitement distinct, qui reçoit l’adresse et les données de carte ;
- **nouvelle durée** : 10 ans pour l’enregistrement comptable d’un achat, détaché du compte à sa suppression ;
- **la renonciation** au droit de rétractation et son moment.

## Critères de réévaluation

- Le conseil juge que le reçu du prestataire ne vaut pas confirmation du contrat : passer à la confirmation envoyée par le jeu, puis à la conservation de l’adresse.
- Le prestataire ne peut pas joindre la mention de la renonciation à son reçu.
- Le montant ou la fréquence des achats fait exiger des factures systématiques.
- Changement de prestataire ou de la loi sur les contenus numériques.
