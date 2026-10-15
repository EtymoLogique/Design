# ADR 0077 — Paiement par Stancer, page hébergée et crédit vérifié par le serveur

- **Statut** : Proposé
- **Portée** : après le MVP, avec la boutique de l’[ADR 0076](0076-boutique-de-gouttes-en-euros.md)
- **Complète** : [ADR 0024](0024-architecture-logicielle-et-hebergement.md) (un service externe de plus, sans script tiers dans la PWA, et une tâche planifiée de plus), [ADR 0018](0018-sabliers-et-boutique.md) (reçu validé par le serveur, crédit idempotent, données de paiement chez le prestataire), [ADR 0070](0070-comptes-facultatifs-par-lien-et-code-e-mail.md) (reçu sans adresse conservée par le jeu), [ADR 0010](0010-observabilite-et-vie-privee.md) (aucune donnée de paiement dans les journaux ni la télémétrie)

## Contexte

L’[ADR 0076](0076-boutique-de-gouttes-en-euros.md) vend des gouttes en euros, par achats de 2 € et plus. Il faut un prestataire de paiement et un parcours qui tiennent les contraintes déjà posées :

- **aucun script tiers** dans la PWA, CSP stricte ([ADR 0024](0024-architecture-logicielle-et-hebergement.md)) : pas de formulaire de carte intégré par un script du prestataire ;
- **autorité du serveur** : le serveur valide chaque paiement avant de créditer, et le crédit est idempotent ([ADR 0018](0018-sabliers-et-boutique.md)) ;
- **aucune adresse conservée** par le jeu ([ADR 0070](0070-comptes-facultatifs-par-lien-et-code-e-mail.md)), alors que la vente à distance exige une confirmation sur un support durable ;
- **petits montants** : sur 2 €, le frais fixe par paiement pèse plus que le pourcentage ;
- **préférence pour un acteur français**, comme pour l’hébergement ;
- **PWA installée sur iOS** : un lien ou une redirection peut s’ouvrir dans Safari, dont le stockage est séparé de celui de la PWA (le même piège que celui de l’ADR 0070).

Les candidats ont été comparés le 14 octobre 2026 sur leurs pages officielles. Pour un achat de 2 € : Stancer prend environ 0,16 €, Stripe 0,28 €, Mollie 0,27 à 0,29 €, PayPlug 0,28 € plus 10 € par mois, Paddle (marchand de référence) environ 0,56 €.

## Décision

### Prestataire : Stancer

- Le paiement passe par **Stancer** (Iliad78, groupe Iliad), établissement de paiement agréé par l’ACPR (n° 11758), dont l’infrastructure est chez Scaleway.
- Tarif publié ([conditions tarifaires d’août 2024](https://stancer.com/fr/pricing-conditions.pdf)) : **0,70 % + 0,15 € HT** par paiement en ligne avec une carte CB, Visa ou Mastercard émise dans l’EEE ; 2,50 % + 0,25 € hors EEE ; sans abonnement ni engagement. Impayé : 3 € sur le réseau CB, 25 € sur Visa ou Mastercard. La grille en vigueur est à faire confirmer par écrit avant la signature.
- Moyens acceptés : **carte seule** (CB, Visa, Mastercard). Ni prélèvement, ni Apple Pay, ni Google Pay au lancement : la page hébergée de Stancer ne propose pas ces portefeuilles.
- Intégration par l’**API v2** ([documentation](https://docs.stancer.com/fr/API.html)), en HTTP direct depuis l’API Rust : aucune bibliothèque du prestataire.

### Parcours d’achat

1. La PWA envoie la commande d’achat (quantité de gouttes) avec sa clé d’idempotence, comme toute commande du jeu.
2. Le serveur vérifie le compte, l’activation des achats, le plafond mensuel et le territoire ([ADR 0076](0076-boutique-de-gouttes-en-euros.md)). Il enregistre un **achat en attente**, puis crée chez Stancer une intention de paiement : montant TTC en centimes, `order_id` égal à l’identifiant de l’achat, `unique_id` pour écarter un doublon, `methods_allowed` limité à `card`, `threeds` à `required`, `capture` à `true`, et `return_url` vers la PWA.
3. La PWA **redirige** le joueur vers la page de paiement hébergée par Stancer. Aucun script tiers ne s’exécute dans l’origine du jeu, et la CSP ne change pas.
4. Au retour, la PWA **ne crédite rien** : elle resynchronise l’état. Elle le refait chaque fois qu’elle reprend le focus, tant qu’un achat reste en attente.

### Authentification forte systématique

- Chaque paiement exige le **3D Secure** (`threeds` à `required`, valeur par défaut chez Stancer). Le joueur ne demande aucune exemption de faible montant.
- Ainsi, la fraude avérée reste à la charge de la banque du porteur, pas du jeu. C’est la protection principale contre l’usage de la carte d’un parent par un enfant.

### Crédit : le serveur relit toujours le paiement

- Un **webhook** Stancer, signé en HMAC-SHA256 (en-tête `Stancer-Signature`, horodatage et corps signés, écart toléré d’une minute), signale les changements. Le serveur vérifie la signature, puis **relit l’intention de paiement** par l’API. Il ne crédite qu’un paiement que Stancer lui confirme payé.
- Le crédit est **idempotent** sur l’identifiant de l’achat. Le solde d’encre, l’achat et un mouvement « achat » réussissent ou échouent ensemble, dans une transaction ([ADR 0007](0007-etat-et-economie-autoritaires.md)).
- Stancer ne garantit ni la livraison, ni le délai, ni l’unicité de ses webhooks, et n’en émet aucun pour les remboursements. Une **tâche planifiée de réconciliation** relit donc les achats en attente et les remboursements en cours, au moins toutes les 10 minutes : elle crédite ce qui est payé, capture un paiement resté autorisé, abandonne ce qui a échoué ou expiré.
- Un achat qui reste en attente plus de 24 heures passe en anomalie, visible dans la supervision.

### Remboursement et rétractation

- La rétractation de l’[ADR 0076](0076-boutique-de-gouttes-en-euros.md) retire du solde les gouttes inutilisées de l’achat, puis demande à Stancer un **remboursement partiel ou total** de l’intention de paiement, par l’API. La réconciliation suit le remboursement jusqu’à son terme.
- Un **impayé** signalé par Stancer retire du solde les gouttes inutilisées de l’achat contesté. Les gouttes déjà dépensées ne sont pas reprises, et le compte n’est pas suspendu sans possibilité de contester.

### Territoire

- Avant le paiement, le serveur estime le pays par l’**adresse réseau** de la requête, sans la conserver ([ADR 0024](0024-architecture-logicielle-et-hebergement.md)). Si la boutique y est fermée, l’achat est refusé.
- Après le paiement, Stancer donne le **pays d’émission de la carte**. Si la boutique y est fermée, l’achat est remboursé en entier et aucune goutte n’est créditée.

### Reçu, support durable et données

- Le reçu est envoyé par Stancer, à une adresse électronique que le joueur saisit **sur la page hébergée**. **Le jeu ne la voit ni ne la conserve** : il ne garde que l’identifiant de l’intention de paiement, la quantité, le montant, le pays de la carte et les dates.
- Le **reçu du prestataire** sert de confirmation du contrat sur un support durable. À faire confirmer par la revue juridique (M6), qui dira aussi s’il suffit pour les pièces comptables et les factures demandées.
- Les données de carte ne transitent jamais par le jeu. Les clés `sprod_…` et le secret des webhooks vivent dans Secret Manager, jamais dans le dépôt.
- Hors production, seules les clés de test `stest_…` sont utilisées ; aucun paiement réel n’y est possible.

### Points à vérifier avant la signature

- La grille tarifaire en vigueur, et si les frais de la transaction d’origine sont rendus lors d’un remboursement.
- L’envoi automatique du reçu au payeur, et son contenu. Surtout : **la page hébergée collecte-t-elle l’adresse du payeur ?** Sinon, il faudrait que la feuille d’achat la demande et la transmette à Stancer sans la garder, ce qui serait un second usage passager d’une adresse en clair, que l’[ADR 0070](0070-comptes-facultatifs-par-lien-et-code-e-mail.md) ne prévoit pas : il faudrait alors le décider ici.
- La possibilité de refuser des pays de carte avant le paiement.
- Le calendrier des webhooks asynchrones, en particulier pour les impayés.
- Le comportement de la redirection et du 3D Secure dans une PWA installée sur iOS et sur Android.

## Options envisagées

### Stripe Checkout

Intégration la plus éprouvée, webhooks fiables, Apple Pay et Google Pay sur la page hébergée. Écartée pour le lancement : environ 0,12 € de plus par achat, soit de l’ordre de 130 € par an au volume du plan financier, un litige à 20 €, et un acteur non français. **C’est le repli** si les essais de Stancer échouent sur iOS ou si ses webhooks se révèlent trop peu fiables : le parcours ci-dessus ne change pas.

### Mollie

Européen, tarif CB proche de celui de Stripe, Apple Pay et Google Pay. Écartée : plus cher que Stancer pour un service comparable, et ses frais de remboursement et de litige ne sont pas publiés.

### PayPlug

Français, page hébergée et remboursements. Écartée : l’abonnement de 10 € par mois pèse face à environ 250 € de chiffre d’affaires mensuel prévu.

### Marchand de référence (Paddle et similaires)

Il gère la TVA de chaque pays. Écartée : environ deux fois plus cher par achat, alors que le chiffre d’affaires prévu reste sous le seuil européen de 10 000 € au-delà duquel la TVA du pays de l’acheteur s’impose.

### Formulaire de carte intégré à la PWA

Écartée : il demande un script du prestataire dans l’origine du jeu, contraire à la CSP de l’ADR 0024.

### Page hébergée dans une iframe

Stancer la permet, et elle garderait le joueur dans la PWA installée sur iOS. Non retenue d’emblée : elle demande d’ouvrir `frame-src` à `payment.stancer.com` dans la CSP, et le 3D Secure peut ouvrir une autre fenêtre. À reprendre si la redirection se passe mal sur iOS, par un ADR qui modifie la CSP.

### Crédit sur le seul webhook, ou sur le retour du navigateur

Écartée : le retour du navigateur peut ne jamais arriver ou arriver dans Safari, et le webhook de Stancer n’est pas garanti. Seule la relecture par le serveur, appuyée sur la réconciliation, fait foi.

## Conséquences

### Positives

- Le frais par achat le plus bas parmi les candidats, sans abonnement, chez un acteur français hébergé chez Scaleway.
- Aucun script tiers, aucune donnée de carte et aucune adresse dans le jeu.
- La fraude reste à la charge des banques grâce au 3D Secure systématique.
- Un crédit sûr même si le joueur ferme son navigateur ou si un webhook se perd.

### Négatives

- Une tâche de réconciliation à écrire, tester et superviser, en plus du webhook.
- Pas d’Apple Pay ni de Google Pay : un peu plus de friction sur mobile.
- Le 3D Secure systématique ajoute une étape à chaque achat.
- Les fonds arrivent 7 jours après le paiement ; un versement anticipé coûte 2 %.
- Un acteur jeune, dont le support technique passe par une communauté ; un changement de prestataire reste possible, mais demande une nouvelle intégration.
- Un service externe de plus, avec ses clés à protéger.

## Critères de réévaluation

- Plus de 1 % des achats passent en anomalie (en attente plus de 24 heures), ou les webhooks se perdent souvent : passer à Stripe.
- Plus de 10 % des paiements abandonnés pendant le 3D Secure ou au retour sur iOS : tester l’iframe, ou Stripe avec Apple Pay.
- Les ventes hors de France dépassent 10 000 € par an : TVA du pays de l’acheteur et guichet unique, à confier au prestataire ou à un outil.
- Stancer change sa grille au-delà de 1 % + 0,25 € par paiement.
- Les joueurs demandent Apple Pay ou Google Pay de façon répétée.
