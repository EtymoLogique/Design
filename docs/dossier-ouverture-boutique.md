# ÉtymoLogique — Dossier d’ouverture de la boutique

Ce dossier réunit ce qu’il faut décider, rédiger et faire valider avant la vente de sabliers en euros (M7, avril 2027). Il répond à l’écart L17 de l’audit juridique du projet ([EtymoLogique/App#63](https://github.com/EtymoLogique/App/blob/develop/docs/audits/juridique.md)) et à l’[issue Design#33](https://github.com/EtymoLogique/Design/issues/33). Il s’appuie sur l’[ADR 0018](adr/0018-sabliers-et-boutique.md) (boutique), l’[ADR 0039](adr/0039-plafonner-l-achat-des-sabliers-pas-leur-usage.md) (sabliers) et l’[ADR 0073](adr/0073-boutique-support-durable-et-pieces-comptables.md) (support durable, pièces comptables), et sur le [plan financier](plan-financier.md) (lots à 1,99 € et 4,99 €, TVA à 20 %).

> **Ce dossier n’est pas un avis juridique.** Chaque règle citée vient d’un relevé et reste **à confirmer par le conseil**. Tant que les cases de la [liste de lancement](#liste-de-lancement) ne sont pas toutes cochées, la boutique reste fermée (« Bientôt ouvert », ADR 0039).

## Liste de lancement

| # | Point | Qui décide | État |
|---|---|---|---|
| 1 | [CGV](#1-conditions-générales-de-vente) relues par le conseil | Conseil | Brouillon rempli ci-dessous, à relire par le conseil |
| 2 | [Médiateur de la consommation](#2-médiateur-de-la-consommation) désigné et cité dans les CGV | Porteur du projet | Retenu : CM2C ([ADR 0074](adr/0074-ouverture-de-la-boutique-mediateur-tva-territoires-plafond.md)), à confirmer à l’adhésion |
| 3 | [Régime de TVA](#3-tva) décidé | Porteur du projet, avec l’expert-comptable | Retenu : option A, TVA française puis OSS (ADR 0074) |
| 4 | [Revue par territoire](#4-revue-par-territoire) faite, territoires fermés listés | Conseil | Rempli : France et Union ouvertes, Belgique, Pays-Bas et hors Union fermés (ADR 0074) |
| 5 | [Accessibilité](#5-accessibilité-european-accessibility-act) : exemption confirmée ou conformité visée | Conseil | Microentreprise retenue, à confirmer |
| 6 | [Plafond mensuel par défaut](#6-plafond-mensuel-par-défaut-pour-les-mineurs) fixé | Porteur du projet | Retenu : 10 € TTC par mois (ADR 0074) |
| 7 | Aucun lien vers la plateforme européenne de règlement en ligne des litiges (ODR) | Relecture | Voir [ci-dessous](#2-médiateur-de-la-consommation) |

Dépendances : la décision sur le support durable (ADR 0073, [Design#32](https://github.com/EtymoLogique/Design/issues/32)) et la récupération de progression de l’[ADR 0007](adr/0007-etat-et-economie-autoritaires.md), sans lesquelles la boutique ne s’ouvre pas ([ADR 0018](adr/0018-sabliers-et-boutique.md)).

## 1. Conditions générales de vente

### Contenu obligatoire

| Exigence | Texte de référence | Où la satisfaire |
|---|---|---|
| Informations précontractuelles : caractéristiques essentielles, prix TTC, identité du vendeur, modalités de paiement et de fourniture, garanties, fonctionnalités et interopérabilité du contenu numérique | C. consom., art. L221-5 et L111-1 | CGV, et récapitulatif sur l’écran d’achat |
| Bouton de commande portant la mention « commande avec obligation de paiement » ou formule non équivoque | C. consom., art. L221-14 | Écran d’achat, dernier bouton avant le paiement |
| Confirmation du contrat sur support durable | C. consom., art. L221-13 | Reçu du prestataire de paiement ([ADR 0073](adr/0073-boutique-support-durable-et-pieces-comptables.md)) |
| Rétractation de 14 jours, et renonciation expresse avant le paiement pour un contenu numérique fourni tout de suite | C. consom., art. L221-18, L221-28 | Case non précochée avant la redirection vers le paiement (ADR 0073) |
| Garantie légale de conformité du contenu numérique | C. consom., art. L224-25-1 et suivants | CGV |
| Médiateur de la consommation | C. consom., art. L612-1, L616-1 | CGV, [§ 2](#2-médiateur-de-la-consommation) |

### Brouillon

Les passages entre crochets sont à compléter. Vouvoiement, comme le reste du jeu.

**Article 1 — Vendeur.** ÉtymoLogique est édité par [dénomination, forme juridique, capital, siège, SIREN, numéro de TVA intracommunautaire, adresse électronique de contact, directeur de la publication].

**Article 2 — Ce que vous achetez.** Un **sablier** est un contenu numérique du jeu : il fait avancer d’une heure la recharge de l’énergie. Il ne donne aucun tirage, ne change ni les chances, ni la garantie de nouveauté, ni le filet d’utilité. Tout ce que la boutique vend s’obtient aussi en jouant. Les lots, leurs prix TTC et le prix par sablier sont affichés sur l’écran d’achat.

**Article 3 — Prix.** Les prix sont en euros, toutes taxes comprises. Le prix affiché au moment de la commande est celui que vous payez : [mention sur la TVA applicable au pays de l’acheteur, selon la décision du § 3].

**Article 4 — Commander.** Vous choisissez un lot, vous vérifiez le récapitulatif, vous cochez la case de renonciation (article 6), puis vous appuyez sur « Commande avec obligation de paiement ». Le paiement est assuré par [prestataire], qui conserve vos données de carte et votre adresse comme responsable de traitement distinct. ÉtymoLogique ne reçoit ni votre adresse ni vos données de carte.

**Article 5 — Livraison.** Les sabliers sont crédités sur votre réserve dès la confirmation du paiement. Un reçu vous est envoyé par [prestataire].

**Article 6 — Rétractation.** Vous avez en principe 14 jours pour vous rétracter d’un achat à distance. Un contenu numérique crédité tout de suite fait exception : avant de payer, vous demandez expressément l’exécution immédiate et vous renoncez à votre droit de rétractation. Sans cette demande, la commande n’est pas possible. [À confirmer par le conseil : par geste commercial, un lot dont aucun sablier n’a été utilisé reste remboursable pendant 14 jours (ADR 0018, ADR 0073).]

**Article 7 — Garantie légale de conformité.** Le contenu numérique est conforme au contrat. En cas de défaut de conformité, vous avez droit à sa mise en conformité, puis, à défaut, à une réduction du prix ou à la résolution du contrat, dans les conditions des articles L224-25-1 et suivants du code de la consommation. [Reprendre ici le texte intégral exigé : durée, charge de la preuve, recours.]

**Article 8 — Plafond de dépense.** Un plafond mensuel de [montant, § 6] est appliqué par défaut. Le titulaire du moyen de paiement peut le modifier.

**Article 9 — Territoires.** La boutique peut être fermée dans certains pays. Le reste du jeu reste accessible. [Liste du § 4.]

**Article 10 — Réclamations et médiation.** Écrivez d’abord à [adresse]. Sans réponse satisfaisante sous [délai], vous pouvez recourir gratuitement au médiateur de la consommation : [nom, adresse postale, site du médiateur choisi au § 2].

**Article 11 — Données personnelles.** Voir la politique de confidentialité ([ADR 0010](adr/0010-observabilite-et-vie-privee.md), [ADR 0073](adr/0073-boutique-support-durable-et-pieces-comptables.md)) : le prestataire de paiement est un destinataire, les pièces comptables sont conservées 10 ans sans adresse ni nom.

**Article 12 — Droit applicable.** Droit français, sans priver un consommateur de la protection de la loi de son pays de résidence.

## 2. Médiateur de la consommation

Tout professionnel qui vend à des consommateurs doit leur garantir un recours effectif à un médiateur de la consommation (C. consom., art. L612-1), et lui communiquer ses coordonnées (art. L616-1).

- **Choix.** Le médiateur doit figurer sur la liste que la Commission d’évaluation et de contrôle de la médiation de la consommation (CECMC) publie. Le porteur du projet le choisit ; ce dossier ne le fait pas à sa place.
- **Critères.** Le médiateur accepte les litiges de contenu numérique vendu en ligne, traite en français, ne facture rien au consommateur, et fixe un coût d’adhésion ou de saisine compatible avec un panier moyen de 3,50 € TTC ([plan financier](plan-financier.md)).
- **Où l’afficher.** Dans les CGV (article 10), sur la page de la boutique, et sur la page des mentions légales.
- **Pas de lien vers la plateforme ODR.** La plateforme européenne de règlement en ligne des litiges a fermé le 20 juillet 2025 (règlement (UE) 2024/3228). Aucune page, aucun texte ni aucun modèle d’e-mail ne la cite. Contrôle avant l’ouverture : chercher le mot « ODR » dans les CGV, les pages et les e-mails de l’application.

## 3. TVA

Le plan financier compte 20 % sur tous les lots. La règle change avec la provenance des acheteurs :

- **Jusqu’à 10 000 € HT par an** de ventes à des particuliers d’autres États membres (seuil commun à l’ensemble de l’Union), la TVA française à 20 % s’applique partout.
- **Au-delà**, c’est la TVA du pays de l’acheteur, déclarée par le **guichet unique OSS** (régime de l’Union).
- Les ventes aux particuliers français restent à 20 %, quel que soit le seuil.

### Options

| | A. Le jeu vend, TVA française puis OSS | B. Un revendeur officiel vend (« merchant of record ») |
|---|---|---|
| Qui collecte et déclare la TVA | ÉtymoLogique | Le revendeur |
| Revenu net | Prix HT, moins frais de paiement (1,5 % + 0,25 %, plan financier) | Prix HT, moins une commission plus forte |
| Charge | Suivre le seuil de 10 000 €, s’inscrire à l’OSS, déclarer chaque trimestre | Aucune déclaration de TVA à l’étranger |
| Vendeur auprès du consommateur | ÉtymoLogique (CGV ci-dessus) | Le revendeur : CGV et médiateur à réécrire |
| Reçu et support durable | Prestataire de paiement (ADR 0073) | Revendeur, à vérifier avec l’ADR 0073 |

### Proposition

**Option A.** À 1,99 € et 4,99 €, le panier est bas et le volume incertain : le seuil de 10 000 € ne sera probablement pas dépassé au premier exercice, et la TVA française à 20 % s’applique alors partout. L’option B est le repli si l’inscription à l’OSS pèse plus que la commission. À valider avec l’expert-comptable ; la décision, une fois prise, change le plan financier (TVA par pays) et fait l’objet d’un ADR.

Mise en œuvre : le serveur enregistre le pays d’émission de la carte et la TVA de chaque achat ([ADR 0073](adr/0073-boutique-support-durable-et-pieces-comptables.md)) ; le cumul annuel des ventes hors France sert de compteur du seuil et alerte à 80 %.

## 4. Revue par territoire

L’[ADR 0018](adr/0018-sabliers-et-boutique.md) prévoit une revue juridique par territoire avant l’ouverture, car un sablier accélère l’accès à un tirage aléatoire (coffres à butin, [ADR 0011](adr/0011-plis-raretes-et-doublons.md)), et la boutique doit pouvoir se bloquer par territoire sans toucher au reste du jeu.

| Territoire | Raison de la revue | Décision du conseil | Boutique |
|---|---|---|---|
| France | Marché principal ; règles de la consommation de ce dossier | À remplir | À remplir |
| Belgique | Règles belges sur les coffres à butin | À remplir | À remplir |
| Pays-Bas | Règles néerlandaises sur les coffres à butin | À remplir | À remplir |
| Autres États membres de l’Union | Droit de la consommation harmonisé ; TVA du [§ 3](#3-tva) | À remplir | À remplir |
| Hors Union | Non ouvert au lancement, sauf avis contraire | À remplir | Fermée par défaut |

Exigences techniques (dans l’application, hors de ce dépôt) :

- le pays se déduit côté serveur, jamais d’une donnée que le joueur saisit seul ;
- un territoire fermé voit la boutique verrouillée (« Bientôt ouvert » ou « Indisponible dans votre pays »), sans effet sur l’échange d’encre ni sur le reste du jeu ;
- la liste des territoires fermés est un paramètre versionné ([ADR 0009](adr/0009-contenu-et-equilibrage.md)) et figure dans les CGV (article 9).

## 5. Accessibilité (European Accessibility Act)

La directive (UE) 2019/882 s’applique au commerce en ligne depuis le 28 juin 2025. Les **microentreprises** (moins de 10 personnes et au plus 2 M€ de chiffre d’affaires ou de bilan) qui fournissent des services en sont probablement exemptées. **À confirmer par le conseil**, en fonction de la structure juridique réelle (structure allégée ou complète du plan financier).

Quelle que soit la réponse, le projet vise déjà les critères de l’identité visuelle : contraste 4,5:1, cibles de 44 px, couleur jamais seul signal, `prefers-reduced-motion`. Si l’exemption ne tient pas : déclaration d’accessibilité et parcours d’achat au niveau WCAG 2.1 AA, à vérifier sur l’écran d’achat et la page de la boutique avant l’ouverture.

## 6. Plafond mensuel par défaut pour les mineurs

L’[ADR 0018](adr/0018-sabliers-et-boutique.md) prévoit, sur le web, un plafond de dépense mensuel par défaut, modifiable par le titulaire du moyen de paiement. Sur les magasins d’applications, le contrôle parental de la plateforme s’applique.

**Proposition : 10 € TTC par mois et par compte**, soit environ deux lots à 4,99 €. Il couvre largement l’usage normal (panier moyen de 3,50 €) et borne la dépense d’un mineur. Aucun ciblage ni offre personnalisée. Le plafond s’affiche sur l’écran d’achat avec le montant déjà dépensé ce mois-ci. À fixer par le porteur du projet, puis à inscrire dans un ADR ou un paramètre d’équilibrage ([ADR 0009](adr/0009-contenu-et-equilibrage.md)).

## Ordre de travail

1. Choisir le médiateur ([§ 2](#2-médiateur-de-la-consommation)) et décider la TVA ([§ 3](#3-tva)) : ils alimentent les CGV.
2. Envoyer au conseil les CGV, l’[ADR 0073](adr/0073-boutique-support-durable-et-pieces-comptables.md), le tableau du [§ 4](#4-revue-par-territoire) et la question du [§ 5](#5-accessibilité-european-accessibility-act).
3. Intégrer ses retours ; passer l’ADR 0018 (« Proposé ») et l’ADR 0073 à « Accepté » ; fixer les prix dans un ADR.
4. Ouvrir la boutique territoire par territoire, en commençant par la France.
