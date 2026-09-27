# ÉtymoLogique — Plan financier sur 18 mois

> Plan du 27 septembre 2026, fondé sur l’[étude de marché](etude-de-marche.md). Montants en euros hors taxes, arrondis à la centaine (à la dizaine sous 1 000 €). Les tableaux sont calculés par `docs/scripts/plan_financier.py` : pour changer une hypothèse, modifiez le script puis relancez-le, ne retouchez pas les tableaux à la main.

## Résumé

| | Phases 1 et 2 : M1 à M6 | Phase 3 : M7 à M18 |
|---|---|---|
| Période | octobre 2026 à mars 2027 | avril 2027 à mars 2028 |
| Contenu | 3 mois de développement, mise en production en janvier 2027, 3 mois d’exploitation sans paiement | MCO, un fascicule par mois, boutique de sabliers ouverte en avril 2027 |
| Coûts, structure complète | 224 600 € | 281 700 € |
| Coûts, structure allégée | 144 000 € | 167 700 € |
| Chiffre d’affaires (scénario central) | 0 € | 3 000 € |
| Joueurs actifs en fin de période (central) | 6 270 | 7 690 |

**Ce que dit le plan.** Le jeu peut se construire et vivre 18 mois avec un budget de 310 000 € (structure allégée) à 500 000 € (structure complète). Mais la boutique de sabliers, telle que l’[ADR 0018](adr/0018-sabliers-et-boutique.md) la conçoit (plafonnée, sans relance, sans monnaie premium), ne couvre que 2 à 5 % des coûts mensuels à 18 mois. Même le scénario favorable demande environ 240 000 joueurs actifs pour atteindre l’équilibre. Le plan montre donc un **besoin de financement**, pas une rentabilité, et appelle des décisions sur des revenus complémentaires ([§ 6](#6-leviers-pour-atteindre-léquilibre)).

## 1. Calendrier

| Mois | Phase | Jalons |
|---|---|---|
| M1 oct. 2026 | Développement | Création de la société, dépôt de la marque, API Rust et PWA React en chantier, schéma du catalogue, liste d’attente ouverte |
| M2 nov. 2026 | Développement | Table de fusion, plis, codex de bout en bout ; fascicule 1 rédigé |
| M3 déc. 2026 | Développement | Recette, audit d’accessibilité et de sécurité, CGU et politique de confidentialité, fascicule 2 prêt, dossier de presse |
| **M4 janv. 2027** | **Mise en production** | Fascicule 1 ; lancement presse et créateurs de contenu |
| M5 févr. 2027 | Exploitation | Fascicule 2 ; correctifs ; décision sur les comptes et la récupération de progression ([ADR 0007](adr/0007-etat-et-economie-autoritaires.md)) |
| M6 mars 2027 | Exploitation | Fascicule 3 ; Semaine de la langue française ; ADR 0018 accepté, revue juridique de la boutique, intégration du paiement |
| **M7 avr. 2027** | **MCO et boutique** | Fascicule 4 ; ouverture de la boutique de sabliers, désactivable par territoire |
| M8 à M11 | MCO | Fascicules 5 à 8 ; rythme d’été réduit en marketing |
| M12 sept. 2027 | MCO | Fascicule 9 ; campagne de rentrée (enseignants, étudiants), second audit |
| M13 à M17 | MCO | Fascicules 10 à 14 ; au-delà de dix, chaque fascicule demande une jaquette nouvelle ([ADR 0016](adr/0016-plis-et-jaquettes-par-fascicule.md)) ; campagne de fin d’année en M15 |
| M18 mars 2028 | MCO | Fascicule 15 ; Semaine de la langue française ; bilan et décision sur l’an 2 |

Un fascicule est **produit un mois avant sa parution** (préparé à l’avance, publié à sa date : [ADR 0029](adr/0029-parution-par-promotion-de-prefixe.md)). De M2 à M18, 17 fascicules sont produits : 15 paraissent, 2 restent d’avance à la fin du plan.

**Point d’attention.** Trois mois de développement pour l’API, la PWA et les 25 interfaces de l’[inventaire](interfaces.html) supposent deux développeurs à plein temps et un périmètre tenu. Un mois de retard coûte environ 35 000 € (structure complète) et décale d’autant toutes les dates.

## 2. Hypothèses de coûts

### Équipe

| Poste | Structure complète | Structure allégée |
|---|---|---|
| Développement back (Rust), 650 € par jour | 20 j par mois de M1 à M3, puis 10 j de M4 à M6 | 15 j, puis 6 j |
| Développement front (React, PWA), 550 € par jour | 20 j par mois de M1 à M3, 10 j en M4, 8 j en M5 et M6 | 10 j, puis 4 j (le reste par le fondateur ou la fondatrice) |
| MCO à partir de M7, 600 € par jour | 6 j par mois (correctifs, dépendances, sécurité, supervision) + 4 j d’évolutions | 4 j + 2 j |
| Direction produit et design | 4 500 € chargés par mois | non rémunérée |
| Direction artistique | 2 000 € par mois de M1 à M3 (charte appliquée, jaquettes 1 à 3) | idem |
| Support et communauté | 1 200 € par mois dès le lancement | 600 € |
| Audits accessibilité et sécurité | 3 500 € en M3, 2 000 € en M12 | idem |

### Éditorial : 6 500 € par fascicule (4 500 € en structure allégée)

| Poste | Coût |
|---|---|
| Sourçage et rédaction de 25 mots (environ 11 jours à 350 €) | 3 850 € |
| Relecture savante (2 jours à 450 €) | 900 € |
| Jaquette | 600 € |
| Silhouettes des cartes ([ADR 0023](adr/0023-textures-des-cartes.md)) | 800 € |
| Intégration, validation d’atteignabilité et simulation d’équilibrage | 350 € |

S’y ajoutent 2 000 € en M1 pour le schéma du catalogue et le choix des sources.

### Infrastructure et frais généraux

| Poste | Hypothèse |
|---|---|
| Hébergement Scaleway serverless ([ADR 0024](adr/0024-architecture-logicielle-et-hebergement.md)) | 30 € par mois en développement ; ensuite 50 € + 0,005 € par joueur actif mensuel |
| Outils (dépôt, design, supervision, domaine) | 120 € par mois |
| Expert-comptable, banque, assurance RC pro | 290 € par mois |
| Juridique | 1 400 € en M1 (SAS, marque à l’INPI : [ADR 0017](adr/0017-licences.md)) ; 2 500 € en M3 (CGU, RGPD) ; 4 000 € en M6 (revue de la boutique par territoire, droit de rétractation) |

### Marketing : 76 700 € sur 18 mois

| Période | Achat média | Presse et influence | Contenus et outils | Total |
|---|---|---|---|---|
| M1 à M3 (liste d’attente, dossier de presse, visuels) | 0 € | 1 500 € | 6 000 € | 7 500 € |
| M4 (lancement) | 6 000 € | 4 000 € | 2 000 € | 12 000 € |
| M5 et M6 (dont Semaine de la langue française) | 8 000 € | 4 000 € | 2 000 € | 14 000 € |
| M7 à M18 | 31 500 € | 13 500 € | 10 200 € | 55 200 € |

Pas de publicité dans le jeu : le marketing est une dépense d’acquisition, jamais une source de revenu ([ADR 0018](adr/0018-sabliers-et-boutique.md), options écartées).

## 3. Hypothèses d’audience et de revenu

| Hypothèse | Prudent | Central | Favorable |
|---|---|---|---|
| Coût d’un joueur venu d’une publicité | 3,50 € | 2,50 € | 1,80 € |
| Nouveaux joueurs par bouche-à-oreille, en part des actifs du mois précédent | 7 % | 10 % | 14 % |
| Joueurs actifs qui achètent des sabliers dans le mois | 0,8 % | 1,5 % | 2,5 % |
| Dépense mensuelle par payeur, TTC | 3,50 € | 4,50 € | 6,00 € |

Communes aux trois scénarios :

- **Temps forts** (presse, liste d’attente, événements) : 3 500 joueurs au lancement, 2 000 à chaque Semaine de la langue française, 1 000 à la rentrée, 800 en février et en décembre.
- **Rétention mensuelle d’une cohorte** : 100 % le mois d’arrivée, puis 30, 20, 16, 14, 12, 11 et 10 %, puis 95 % du mois précédent ; le fascicule mensuel est la raison de revenir.
- **Boutique** ouverte en M7, à la moitié de son régime le premier mois ; lots de 12 sabliers à 1,99 € et de 36 à 4,99 €, panier moyen de 3,50 € TTC ; TVA de 20 % ; paiement par carte à 1,5 % + 0,25 €, sans commission de magasin (PWA).
- Aucune subvention, aucun crédit d’impôt ni aucun revenu complémentaire n’est compté : ils sont traités au [§ 5](#5-financement).

## 4. Résultats

<!-- PLAN:START -->

### Synthèse des scénarios

| Indicateur | Prudent | Central | Favorable | Central, structure allégée |
|---|---|---|---|---|
| Joueurs actifs en M6 (mars 2027) | 5 240 | 6 270 | 7 780 | 6 270 |
| Joueurs actifs en M18 (mars 2028) | 6 020 | 7 690 | 10 380 | 7 690 |
| Joueurs acquis sur 18 mois | 26 580 | 34 750 | 47 620 | 34 750 |
| Payeurs en M18 | 48 | 120 | 260 | 120 |
| CA HT M7 à M18 | 930 € | 3 000 € | 9 300 € | 3 000 € |
| CA HT mensuel en M18 | 140 € | 430 € | 1 300 € | 430 € |
| Coûts M1 à M6 | 224 600 € | 224 600 € | 224 600 € | 144 000 € |
| Coûts M7 à M18 | 281 400 € | 281 700 € | 282 500 € | 167 700 € |
| Résultat M7 à M18 | −280 500 € | −278 700 € | −273 100 € | −164 700 € |
| Trésorerie cumulée au plus bas (besoin de financement) | −505 100 € | −503 300 € | −497 700 € | −308 700 € |
| Coûts mensuels en M18 | 26 700 € | 26 700 € | 26 800 € | 17 200 € |
| Couverture des coûts par la boutique en M18 | 1 % | 2 % | 5 % | 3 % |
| MAU nécessaires pour couvrir les coûts de M18 | 1 276 220 | 529 560 | 238 420 | 341 130 |


### Phase 1 : développement, de M1 à M3 (scénario central)

| Mois | M1<br>oct. 2026 | M2<br>nov. 2026 | M3<br>déc. 2026 | Total |
|---|---|---|---|---|
| Développement | 24 000 | 24 000 | 24 000 | 72 000 |
| Produit, design, communauté, audits | 6 500 | 6 500 | 10 000 | 23 000 |
| Éditorial (fascicules) | 2 000 | 6 500 | 6 500 | 15 000 |
| Infrastructure | 30 | 30 | 30 | 90 |
| Frais généraux et juridique | 1 800 | 410 | 2 900 | 5 100 |
| Achat média | 0 | 0 | 0 | 0 |
| Relations presse et influence | 0 | 0 | 1 500 | 1 500 |
| Contenus et outils marketing | 1 500 | 1 500 | 3 000 | 6 000 |
| Frais de paiement | 0 | 0 | 0 | 0 |
| **Total des coûts** | **35 800** | **38 900** | **47 900** | **122 700** |
| Chiffre d’affaires HT (sabliers) | 0 | 0 | 0 | 0 |
| **Résultat du mois** | **−35 800** | **−38 900** | **−47 900** | **−122 700** |
| **Trésorerie cumulée** | **−35 800** | **−74 800** | **−122 700** |  |


### Phase 2 : lancement et trois premiers mois, de M4 à M6 (scénario central)

| Mois | M4<br>janv. 2027 | M5<br>févr. 2027 | M6<br>mars 2027 | Total |
|---|---|---|---|---|
| Développement | 12 000 | 10 900 | 10 900 | 33 800 |
| Produit, design, communauté, audits | 5 700 | 5 700 | 5 700 | 17 100 |
| Éditorial (fascicules) | 6 500 | 6 500 | 6 500 | 19 500 |
| Infrastructure | 80 | 70 | 80 | 230 |
| Frais généraux et juridique | 410 | 410 | 4 400 | 5 200 |
| Achat média | 6 000 | 3 500 | 4 500 | 14 000 |
| Relations presse et influence | 4 000 | 1 500 | 2 500 | 8 000 |
| Contenus et outils marketing | 2 000 | 1 000 | 1 000 | 4 000 |
| Frais de paiement | 0 | 0 | 0 | 0 |
| **Total des coûts** | **36 700** | **29 600** | **35 600** | **101 900** |
| Chiffre d’affaires HT (sabliers) | 0 | 0 | 0 | 0 |
| **Résultat du mois** | **−36 700** | **−29 600** | **−35 600** | **−101 900** |
| **Trésorerie cumulée** | **−159 400** | **−189 000** | **−224 600** |  |


### Audience des six premiers mois (scénario central)

| Mois | Fascicules parus | Nouveaux joueurs | Joueurs actifs du mois (MAU) | Payeurs | CA TTC |
|---|---|---|---|---|---|
| M4 janv. 2027 | 1 | 5 900 | 5 900 | 0 | 0 |
| M5 févr. 2027 | 2 | 2 790 | 4 560 | 0 | 0 |
| M6 mars 2027 | 3 | 4 260 | 6 270 | 0 | 0 |


### Phase 3 : MCO et fascicules, de M7 à M12 (scénario central)

| Mois | M7<br>avr. 2027 | M8<br>mai 2027 | M9<br>juin 2027 | M10<br>juil. 2027 | M11<br>août 2027 | M12<br>sept. 2027 | Total |
|---|---|---|---|---|---|---|---|
| Développement | 6 000 | 6 000 | 6 000 | 6 000 | 6 000 | 6 000 | 36 000 |
| Produit, design, communauté, audits | 5 700 | 5 700 | 5 700 | 5 700 | 5 700 | 7 700 | 36 200 |
| Éditorial (fascicules) | 6 500 | 6 500 | 6 500 | 6 500 | 6 500 | 6 500 | 39 000 |
| Infrastructure | 70 | 70 | 70 | 70 | 70 | 80 | 420 |
| Frais généraux et juridique | 410 | 410 | 410 | 410 | 410 | 410 | 2 500 |
| Achat média | 2 000 | 2 000 | 2 000 | 1 500 | 1 500 | 3 500 | 12 500 |
| Relations presse et influence | 1 000 | 1 000 | 1 000 | 500 | 500 | 1 500 | 5 500 |
| Contenus et outils marketing | 800 | 800 | 800 | 800 | 800 | 1 000 | 5 000 |
| Frais de paiement | 10 | 20 | 20 | 20 | 20 | 30 | 120 |
| **Total des coûts** | **22 500** | **22 500** | **22 500** | **21 500** | **21 500** | **26 700** | **137 200** |
| Chiffre d’affaires HT (sabliers) | 120 | 210 | 200 | 190 | 180 | 280 | 1 200 |
| **Résultat du mois** | **−22 400** | **−22 300** | **−22 300** | **−21 300** | **−21 300** | **−26 400** | **−136 000** |
| **Trésorerie cumulée** | **−247 000** | **−269 200** | **−291 500** | **−312 900** | **−334 200** | **−360 600** |  |


### Phase 3 : MCO et fascicules, de M13 à M18 (scénario central)

| Mois | M13<br>oct. 2027 | M14<br>nov. 2027 | M15<br>déc. 2027 | M16<br>janv. 2028 | M17<br>févr. 2028 | M18<br>mars 2028 | Total |
|---|---|---|---|---|---|---|---|
| Développement | 6 000 | 6 000 | 6 000 | 6 000 | 6 000 | 6 000 | 36 000 |
| Produit, design, communauté, audits | 5 700 | 5 700 | 5 700 | 5 700 | 5 700 | 5 700 | 34 200 |
| Éditorial (fascicules) | 6 500 | 6 500 | 6 500 | 6 500 | 6 500 | 6 500 | 39 000 |
| Infrastructure | 70 | 70 | 80 | 80 | 70 | 90 | 460 |
| Frais généraux et juridique | 410 | 410 | 410 | 410 | 410 | 410 | 2 500 |
| Achat média | 2 500 | 2 500 | 4 000 | 3 000 | 2 500 | 4 500 | 19 000 |
| Relations presse et influence | 1 000 | 1 000 | 1 500 | 1 000 | 1 000 | 2 500 | 8 000 |
| Contenus et outils marketing | 800 | 800 | 1 000 | 800 | 800 | 1 000 | 5 200 |
| Frais de paiement | 30 | 30 | 30 | 30 | 30 | 40 | 190 |
| **Total des coûts** | **23 000** | **23 000** | **25 200** | **23 500** | **23 000** | **26 700** | **144 500** |
| Chiffre d’affaires HT (sabliers) | 240 | 240 | 320 | 290 | 280 | 430 | 1 800 |
| **Résultat du mois** | **−22 800** | **−22 800** | **−24 900** | **−23 200** | **−22 700** | **−26 300** | **−142 700** |
| **Trésorerie cumulée** | **−383 400** | **−406 100** | **−431 000** | **−454 200** | **−477 000** | **−503 300** |  |


### Audience et boutique, de M7 à M18 (scénario central)

| Mois | Fascicules parus | Nouveaux joueurs | Joueurs actifs du mois (MAU) | Payeurs | CA TTC |
|---|---|---|---|---|---|
| M7 avr. 2027 | 4 | 1 430 | 4 210 | 32 | 140 |
| M8 mai 2027 | 5 | 1 220 | 3 770 | 57 | 250 |
| M9 juin 2027 | 6 | 1 180 | 3 610 | 54 | 240 |
| M10 juil. 2027 | 7 | 960 | 3 370 | 50 | 230 |
| M11 août 2027 | 8 | 940 | 3 260 | 49 | 220 |
| M12 sept. 2027 | 9 | 2 730 | 5 040 | 76 | 340 |
| M13 oct. 2027 | 10 | 1 500 | 4 350 | 65 | 290 |
| M14 nov. 2027 | 11 | 1 440 | 4 300 | 64 | 290 |
| M15 déc. 2027 | 12 | 2 830 | 5 730 | 86 | 390 |
| M16 janv. 2028 | 13 | 1 770 | 5 160 | 77 | 350 |
| M17 févr. 2028 | 14 | 1 520 | 4 920 | 74 | 330 |
| M18 mars 2028 | 15 | 4 290 | 7 690 | 120 | 520 |


### Variante : structure allégée, de M1 à M6 (scénario central)

| Mois | M1<br>oct. 2026 | M2<br>nov. 2026 | M3<br>déc. 2026 | M4<br>janv. 2027 | M5<br>févr. 2027 | M6<br>mars 2027 | Total |
|---|---|---|---|---|---|---|---|
| Développement | 15 200 | 15 200 | 15 200 | 6 100 | 6 100 | 6 100 | 64 000 |
| Produit, design, communauté, audits | 2 000 | 2 000 | 5 500 | 600 | 600 | 600 | 11 300 |
| Éditorial (fascicules) | 2 000 | 4 500 | 4 500 | 4 500 | 4 500 | 4 500 | 24 500 |
| Infrastructure | 30 | 30 | 30 | 80 | 70 | 80 | 320 |
| Frais généraux et juridique | 1 800 | 410 | 2 900 | 410 | 410 | 4 400 | 10 400 |
| Achat média | 0 | 0 | 0 | 6 000 | 3 500 | 4 500 | 14 000 |
| Relations presse et influence | 0 | 0 | 1 500 | 4 000 | 1 500 | 2 500 | 9 500 |
| Contenus et outils marketing | 1 500 | 1 500 | 3 000 | 2 000 | 1 000 | 1 000 | 10 000 |
| Frais de paiement | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| **Total des coûts** | **22 600** | **23 700** | **32 700** | **23 700** | **17 700** | **23 700** | **144 000** |
| Chiffre d’affaires HT (sabliers) | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| **Résultat du mois** | **−22 600** | **−23 700** | **−32 700** | **−23 700** | **−17 700** | **−23 700** | **−144 000** |
| **Trésorerie cumulée** | **−22 600** | **−46 300** | **−79 000** | **−102 700** | **−120 300** | **−144 000** |  |


### Variante : structure allégée, de M7 à M18 (scénario central)

| Mois | M7<br>avr. 2027 | M8<br>mai 2027 | M9<br>juin 2027 | M10<br>juil. 2027 | M11<br>août 2027 | M12<br>sept. 2027 | M13<br>oct. 2027 | M14<br>nov. 2027 | M15<br>déc. 2027 | M16<br>janv. 2028 | M17<br>févr. 2028 | M18<br>mars 2028 | Total |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Développement | 3 600 | 3 600 | 3 600 | 3 600 | 3 600 | 3 600 | 3 600 | 3 600 | 3 600 | 3 600 | 3 600 | 3 600 | 43 200 |
| Produit, design, communauté, audits | 600 | 600 | 600 | 600 | 600 | 2 600 | 600 | 600 | 600 | 600 | 600 | 600 | 9 200 |
| Éditorial (fascicules) | 4 500 | 4 500 | 4 500 | 4 500 | 4 500 | 4 500 | 4 500 | 4 500 | 4 500 | 4 500 | 4 500 | 4 500 | 54 000 |
| Infrastructure | 70 | 70 | 70 | 70 | 70 | 80 | 70 | 70 | 80 | 80 | 70 | 90 | 880 |
| Frais généraux et juridique | 410 | 410 | 410 | 410 | 410 | 410 | 410 | 410 | 410 | 410 | 410 | 410 | 4 900 |
| Achat média | 2 000 | 2 000 | 2 000 | 1 500 | 1 500 | 3 500 | 2 500 | 2 500 | 4 000 | 3 000 | 2 500 | 4 500 | 31 500 |
| Relations presse et influence | 1 000 | 1 000 | 1 000 | 500 | 500 | 1 500 | 1 000 | 1 000 | 1 500 | 1 000 | 1 000 | 2 500 | 13 500 |
| Contenus et outils marketing | 800 | 800 | 800 | 800 | 800 | 1 000 | 800 | 800 | 1 000 | 800 | 800 | 1 000 | 10 200 |
| Frais de paiement | 10 | 20 | 20 | 20 | 20 | 30 | 30 | 30 | 30 | 30 | 30 | 40 | 310 |
| **Total des coûts** | **13 000** | **13 000** | **13 000** | **12 000** | **12 000** | **17 200** | **13 500** | **13 500** | **15 700** | **14 000** | **13 500** | **17 200** | **167 700** |
| Chiffre d’affaires HT (sabliers) | 120 | 210 | 200 | 190 | 180 | 280 | 240 | 240 | 320 | 290 | 280 | 430 | 3 000 |
| **Résultat du mois** | **−12 900** | **−12 800** | **−12 800** | **−11 800** | **−11 800** | **−16 900** | **−13 300** | **−13 300** | **−15 400** | **−13 700** | **−13 200** | **−16 800** | **−164 700** |
| **Trésorerie cumulée** | **−156 900** | **−169 700** | **−182 500** | **−194 300** | **−206 100** | **−223 000** | **−236 300** | **−249 600** | **−265 000** | **−278 700** | **−291 900** | **−308 700** |  |


<!-- PLAN:END -->

### Lecture

- **Les six premiers mois coûtent 224 600 €** (144 000 € en structure allégée) et ne rapportent rien, par choix : le MVP est sans paiement ([ADR 0008](adr/0008-energie-et-plis.md)). Le développement représente près de la moitié de ce montant.
- **L’audience plafonne vers 5 000 à 8 000 joueurs actifs** avec ce budget marketing : les pics viennent des temps forts (lancement, mars, rentrée, décembre), et le fascicule mensuel retient une base fidèle. Doubler l’achat média ne double pas l’audience : au coût de 2,50 € par joueur et avec 30 % de rétention le mois suivant, un joueur encore actif au 2ᵉ mois coûte environ 8 €.
- **La boutique rapporte peu** : environ 0,05 € HT par joueur actif et par mois dans le scénario central, contre 0,20 à 0,50 € pour un jeu de réflexion mobile classique, parce qu’elle n’a ni publicité, ni monnaie premium, ni offre limitée, ni relance. C’est le prix des piliers du jeu, et il faut l’assumer.
- **La MCO est le poste stable** : développement, communauté, éditorial et infrastructure coûtent environ 19 000 € par mois en structure complète, 9 000 € en structure allégée, hors marketing.

## 5. Financement

Besoin au plus bas de la trésorerie cumulée : **environ 500 000 €** en structure complète, **environ 310 000 €** en structure allégée, avec une marge de sécurité de 15 % à prévoir. Sources à instruire, **non comptées** dans les tableaux :

| Source | Montant indicatif | Conditions et remarques |
|---|---|---|
| Apport et prêts d’honneur (Initiative France, Réseau Entreprendre) | 15 000 à 50 000 € | Prêt personnel à taux zéro, levier pour un prêt bancaire. |
| Bpifrance, Bourse French Tech | jusqu’à 30 000 € de subvention | Projet innovant, société de moins d’un an. |
| CNC, Fonds d’aide au jeu vidéo | selon la commission | Aide sélective à l’écriture, à la préproduction ou à la production ; dossier à préparer dès M1. |
| Crédit d’impôt jeu vidéo | 30 % des dépenses éligibles | Agrément du CNC, conditions de budget et de contenu culturel ; le dispositif vise en principe les dépenses engagées jusqu’au 31 décembre 2026 : vérifier sa prolongation avant d’en tenir compte. |
| Politique de la langue française (ministère de la Culture), Francophonie | variable | Appels à projets sur la langue française et le numérique. |
| Financement participatif (prévente de soutien, liste d’attente) | 10 000 à 30 000 € | Contreparties sans avantage de jeu : nom au générique, jaquette en affiche. Doit rester compatible avec « aucun paiement dans le MVP ». |

## 6. Leviers pour atteindre l’équilibre

Chaque levier ci-dessous change une décision ou en crée une : il demande un ADR (skill `adr`) avant toute mise en œuvre.

| Levier | Effet estimé | Compatibilité avec les piliers |
|---|---|---|
| Structure allégée (fondateur non rémunéré, un seul prestataire back) | −40 % de coûts, besoin ramené à environ 310 000 € | Aucune difficulté. |
| Achat de soutien unique (par exemple 9,99 €), sans avantage de jeu | 1 à 3 % des joueurs actifs une fois : 500 à 2 000 € par an à l’échelle du plan | Reporté par l’ADR 0018 ; compatible si rien n’est exclusif au jeu. |
| Cosmétiques (jaquettes alternatives, thèmes) | Comparable au soutien | À cadrer : « une couleur, un seul sens » et jaquettes qui ne trahissent rien ([ADR 0016](adr/0016-plis-et-jaquettes-par-fascicule.md)). |
| Offre établissement (écoles, bibliothèques), par exemple 300 € par an | 50 établissements : 15 000 € par an | Demande un tableau de bord, un accès sans compte individuel et un ADR sur les mineurs. |
| Abonnement de mécène (par exemple 2,99 € par mois), sans avantage de jeu | 0,5 % des actifs : environ 750 € HT par mois à 60 000 actifs | Écarté pour l’instant par l’ADR 0018 (attente de contenu) ; à rouvrir si le rythme mensuel tient un an. |
| Audience bien plus large (réussite de type Cémantix, bouche-à-oreille fort) | Équilibre de la structure allégée vers 340 000 joueurs actifs | Dépend du jeu, pas d’un choix de modèle. |

## 7. Écarts avec les sources de vérité

- **La boutique repose sur un ADR proposé.** L’[ADR 0018](adr/0018-sabliers-et-boutique.md) n’est pas accepté : le revenu à partir de M7 suppose qu’il le soit en M6, avec des prix fixés. Les prix de la [grille proposée](etude-de-marche.md#grille-de-prix-proposée-pour-les-sabliers) sont une hypothèse de ce plan.
- **La boutique suppose une décision sur les comptes.** L’ADR 0018 n’ouvre la boutique qu’une fois la récupération de progression décidée ([ADR 0007](adr/0007-etat-et-economie-autoritaires.md)) ; le plan la place en M5.
- **L’achat de soutien, les cosmétiques, l’offre établissement et l’abonnement** ne sont décidés nulle part ; l’abonnement est même écarté pour l’instant.
- **Trois mois de développement** ne sont fixés par aucun document : c’est la contrainte du plan demandé.

## 8. Mettre à jour le plan

```sh
python3 docs/scripts/plan_financier.py   # recalcule les tableaux du § 4
```

Toutes les hypothèses sont regroupées en tête du script (tarifs, jours, coût d’un fascicule, marketing mois par mois, rétention, scénarios). Remplacez les hypothèses par les mesures réelles dès le lancement : rétention mensuelle par cohorte, part de payeurs, panier moyen, coût par joueur actif au 2ᵉ mois.
