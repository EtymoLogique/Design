# Boutique de gouttes : dossier pour le conseil et projet de CGV

Prépare la revue juridique de M6 (4 000 €, [plan financier](plan-financier.md)) : EtymoLogique/Design#42. Ce document n’est pas un avis juridique. Il rassemble ce qu’il faut remettre au conseil, un **projet** de conditions générales de vente à relire, et le tableau où reporter ses réponses. Rien ne s’ouvre à la vente avant ces réponses.

## 1. Pièces à remettre

- [ADR 0076](adr/0076-boutique-de-gouttes-en-euros.md) (gouttes seules en euros), [ADR 0077](adr/0077-prestataire-et-parcours-de-paiement.md) (Stancer), [ADR 0073](adr/0073-boutique-support-durable-et-pieces-comptables.md) (support durable, pièces comptables), [ADR 0018](adr/0018-sabliers-et-boutique.md) (garde-fous), [ADR 0070](adr/0070-comptes-facultatifs-par-lien-et-code-e-mail.md) (comptes), [ADR 0071](adr/0071-duree-de-conservation-des-comptes-et-des-plis.md) (conservation).
- Les captures de la feuille d’achat, du retour de paiement, de Réglages › Achats en euros et du consentement : [maquettes m-40 à m-43](maquettes.html#m-40).
- [conformite-rgpd.md](conformite-rgpd.md) (registre, violation, mineurs) et l’[audit juridique](https://github.com/EtymoLogique/App/blob/develop/docs/audits/juridique.md).
- Le [plan financier](plan-financier.md) (ventes prévues, TVA).
- Le texte de la case de consentement et du reçu du prestataire (à obtenir de Stancer, Design#43).

## 2. Questions au conseil

Chaque ligne se coche une fois la réponse reportée dans l’ADR concerné.

| # | Question | Source | ADR |
|---|---|---|---|
| Q1 | Le reçu de Stancer vaut-il confirmation du contrat sur un support durable (C. consom., art. L221-13) pour l’achat de gouttes ? Quelles mentions doit-il porter ? | audit L16 | 0073, 0076, 0077 |
| Q2 | Pour un sablier ou un indice payé en gouttes achetées, un historique des achats dans le jeu, consultable et exportable, suffit-il comme support durable ? | audit L16 | 0076 |
| Q3 | La case « exécution immédiate, perte de la rétractation » à la première dépense de gouttes achetées, valable jusqu’à retrait, est-elle suffisante (L221-28) ? | principe CPC 5 | 0076 |
| Q4 | Le sablier est-il un consommable ou une seconde monnaie (principe CPC 2) ? À défaut, vendre le temps directement en gouttes. | principe CPC 2 | 0076 |
| Q5 | L’ordre de dépense (gouttes gagnées d’abord, puis achetées, de la plus ancienne) et le calcul du remboursement au prorata, arrondi au centime inférieur, sont-ils conformes ? | principe CPC 5 | 0076 |
| Q6 | Un préavis de 14 jours pour une hausse du prix en gouttes suffit-il (principe CPC 6) ? | principe CPC 6 | 0076 |
| Q7 | Mineurs : achats désactivés par défaut, plafond de 20 €, 3D Secure systématique ; un mineur peut-il faire seul ces « actes courants » (C. civ., art. 1148) ? | principe CPC 7 | 0076 |
| Q8 | La fonction de rétractation en ligne (directive (UE) 2023/2673, depuis le 19 juin 2026) s’applique-t-elle, et sous quelle forme ? Confirmer la transposition au Journal officiel. | audit L17 | 0076 |
| Q9 | Belgique et Pays-Bas : l’encre achète du temps, donc un accès plus rapide à des plis tirés au hasard. Quels territoires fermer ? | ADR 0018 | 0018, 0076 |
| Q10 | Les objets du jeu, non cessibles et sans valeur, sont-ils bien hors des jeux d’argent (CSI, art. L320-1) et du régime JONUM ? Confirmer qu’aucun échange entre joueurs ne s’ouvre sans nouvelle revue. | audit | 0076 |
| Q11 | Franchise en base de la SAS, TVA française à 20 % sous 10 000 € de ventes B2C intra-UE, passage au guichet unique (OSS) au-delà ; seuils suisse et québécois. | audit L17 | 0077, plan financier |
| Q12 | E-reporting des ventes B2C (vers septembre 2027 pour les petites entreprises) : l’export du prestataire suffit-il ? | audit L17 | 0077 |
| Q13 | Le prestataire est responsable de traitement distinct pour l’adresse et la carte : quelle mention dans la politique de confidentialité et les CGV ? | audit | 0073, 0077 |
| Q14 | Conservation : pièces comptables 10 ans sans adresse ; l’enregistrement détaché d’un compte supprimé suffit-il ? | ADR 0073 | 0073 |
| Q15 | Garantie légale de conformité du contenu numérique (L224-25-1 et suivants) appliquée à des gouttes : quelle formulation ? | audit L17 | CGV |
| Q16 | EAA (accessibilité) : exemption probable des microentreprises, à confirmer. | audit L17 | CGV |

## 3. Médiateur de la consommation

À désigner avant l’ouverture (C. consom., art. L612-1 et L616-1) et à citer dans les CGV avec ses coordonnées. **Ne pas citer la plateforme européenne de règlement en ligne des litiges**, fermée le 20 juillet 2025.

- [ ] Choisir et signer avec un médiateur référencé. Décision à prendre par l’éditeur (Design#35).
- [ ] Reporter nom, adresse postale et site dans les CGV et les mentions légales.

## 4. Projet de CGV

Brouillon à faire relire par le conseil ; les crochets sont à remplir. À publier avec les CGU (App#66), sur une page liée depuis la feuille d’achat, la boutique et les Réglages. Texte en français, vouvoiement.

> **Conditions générales de vente · gouttes d’encre**
>
> **1. Qui vend.** [Dénomination de l’éditeur], [forme], au capital de [capital], [adresse du siège], SIREN [numéro], TVA [numéro ou mention de franchise]. Directeur de la publication : [nom]. Contact : [adresse de contact].
>
> **2. Ce que vous achetez.** Des gouttes d’encre, la ressource du jeu. Les gouttes ne se convertissent jamais en argent, ne se cèdent pas et ne s’échangent pas entre joueurs. Elles achètent du temps (des sabliers) et de l’aide (des indices) : jamais un pli, une brique, une chance ni un tirage. Tout ce qu’elles achètent s’obtient aussi en jouant.
>
> **3. Prix.** 60 gouttes valent 1 € toutes taxes comprises, sans remise de quantité. Vous choisissez la quantité par pas de 60 gouttes, à partir de 120 gouttes (2 €). Votre plafond d’achat est de 20 € par mois civil au plus ; vous pouvez l’abaisser tout de suite et le relever après 7 jours. Le prix d’un sablier ou d’un indice en gouttes s’affiche toujours avec son équivalent en euros. Une hausse de ce prix en gouttes est annoncée dans le jeu au moins 14 jours à l’avance ; elle ne retire jamais ce que vous avez déjà obtenu.
>
> **4. Commander.** Les achats en euros demandent un compte. Ils sont désactivés tant que vous ne les activez pas dans Réglages › Achats en euros. Vous cliquez sur « Commander avec obligation de paiement » ; le paiement par carte se fait sur la page sécurisée de [prestataire], avec une authentification forte de votre banque. Vos gouttes sont créditées quand le paiement est confirmé.
>
> **5. Reçu.** [Prestataire] vous envoie un reçu à l’adresse que vous lui donnez. Le jeu ne garde pas votre adresse électronique. Gardez ce reçu : il confirme votre commande. Votre historique d’achats reste consultable et exportable dans les Réglages.
>
> **6. Droit de rétractation sur les gouttes.** Vous pouvez vous rétracter pendant 14 jours après un achat, sans donner de motif. Vous récupérez le montant payé, au prorata des gouttes de cet achat que vous n’avez pas utilisées, arrondi au centime inférieur ; ces gouttes quittent votre solde. Quand vous dépensez de l’encre, vos gouttes gagnées partent d’abord, puis vos gouttes achetées, de la plus ancienne à la plus récente. Pour vous rétracter, utilisez « Me rétracter » dans Réglages › Achats en euros : il n’y a rien à écrire.
>
> **7. Sabliers et indices payés en gouttes achetées.** Ils vous sont fournis tout de suite. La première fois que vous dépensez des gouttes achetées, nous vous demandons d’accepter l’exécution immédiate et de reconnaître que vous perdez votre droit de rétractation sur ce que vous obtenez ainsi. Vous pouvez retirer cet accord dans les Réglages. Vous ne perdez aucun droit en dépensant des gouttes gagnées.
>
> **8. Garantie.** Les contenus numériques bénéficient de la garantie légale de conformité (articles L224-25-1 et suivants du code de la consommation) [formulation à valider].
>
> **9. Impayé ou contestation.** Si le paiement d’un achat est contesté auprès de la banque, les gouttes inutilisées de cet achat sont retirées de votre solde. Vos gouttes déjà dépensées ne sont pas reprises. Votre compte n’est pas suspendu sans que vous puissiez vous expliquer.
>
> **10. Territoires.** La boutique n’est pas ouverte partout. Si elle est fermée dans le pays de votre carte, l’achat est remboursé en entier et aucune goutte n’est créditée.
>
> **11. Mineurs.** Les achats sont désactivés par défaut. Un titulaire de compte mineur ne peut acheter qu’avec l’accord de son représentant légal [formulation à valider].
>
> **12. Données.** Voir la politique de confidentialité. Le prestataire de paiement traite vos données de paiement pour son propre compte.
>
> **13. Litiges.** Droit français. Vous pouvez écrire à [adresse de contact]. Vous pouvez aussi saisir gratuitement le médiateur de la consommation : [nom, adresse postale, site].

## 5. Réponses du conseil

À remplir, une ligne par question. Reporter ensuite chaque réponse dans l’ADR concerné **avant** son acceptation.

| # | Date | Réponse | Reportée dans |
|---|---|---|---|
| Q1 à Q16 | — | En attente de la revue de M6 | — |
