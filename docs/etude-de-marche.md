# ÉtymoLogique — Étude de marché

> Étude du 27 septembre 2026. Les chiffres de marché viennent de sources publiques citées en fin de document ; les estimations propres au projet sont signalées comme des **hypothèses**. Elle alimente le [plan financier](plan-financier.md).

## Résumé

- **Le marché existe et grandit** : en France, le jeu vidéo pèse 5,8 Md€ en 2025, dont 1,79 Md€ pour le mobile (+11 % en un an). 40,2 millions de Français jouent, et 59 % d’entre eux sur mobile. Les jeux de réflexion sont parmi ceux qui fidélisent le mieux.
- **Les jeux de mots quotidiens ont prouvé l’appétit francophone** : SUTOM, Cémantix (plus de 1 600 énigmes publiées depuis 2022) et Pédantix (plus de 15 000 joueurs par jour dès 2023) ont trouvé leur public sans budget marketing. Aux États-Unis, les jeux du *New York Times* ont été joués 11,2 milliards de fois en 2025.
- **Aucun concurrent direct** ne combine les trois promesses d’ÉtymoLogique : une étymologie **sourcée**, une mécanique de **fusion** (façon *Little Alchemy*) et une **collection** qui se renouvelle chaque mois (façon jeu de cartes à collectionner). C’est un espace libre, mais étroit : le public « curieux de la langue » est une niche.
- **Le modèle économique choisi est éthique mais peu rémunérateur** : sans publicité, sans monnaie premium, avec des sabliers plafonnés à 12 par jour ([ADR 0018](adr/0018-sabliers-et-boutique.md)), le revenu par joueur actif sera nettement inférieur aux références du jeu mobile. À lui seul, il ne financera pas une équipe : il faut soit une audience de plusieurs centaines de milliers de joueurs actifs, soit des revenus complémentaires, à décider par ADR (voir [§ 9](#9-recommandations)).

## 1. Le produit étudié

ÉtymoLogique est une PWA ([ADR 0001](adr/0001-pwa-responsive.md)) de logique et de collection : le joueur fusionne deux briques (préfixe, suffixe, mot) pour retrouver un mot attesté, découvre son histoire et l’ajoute à son codex ([game-design](game-design.md)). Ce qui le distingue :

| Trait | Conséquence pour le marché |
|---|---|
| Contenu éditorial sourcé dans Wikidata et relu | Crédibilité auprès des enseignants, des médias culturels et des institutions de la langue française. |
| Un fascicule de 20 à 30 mots par mois ([ADR 0022](adr/0022-fascicules-de-20-a-30-mots.md)) | Un rendez-vous mensuel qui soutient la rétention et donne un sujet de communication chaque mois. |
| Plis et exemplaires rationnés ([ADR 0012](adr/0012-briques-rationnees.md)) | Une boucle de retour quotidienne, comparable aux jeux de collection, sans paiement obligatoire. |
| Aucun paiement dans le MVP, puis seulement des sabliers plafonnés ([ADR 0008](adr/0008-energie-et-plis.md), [ADR 0018](adr/0018-sabliers-et-boutique.md)) | Argument de confiance fort (familles, enseignants) ; revenu par joueur faible. |
| PWA, hébergement en France ([ADR 0024](adr/0024-architecture-logicielle-et-hebergement.md)) | Pas de commission des magasins d’applications, mais pas non plus de visibilité dans leurs classements. |
| Code AGPL, contenu CC BY-SA ([ADR 0017](adr/0017-licences.md)) | Image de bien commun ; le contenu ne peut pas être vendu comme exclusivité. |

## 2. Taille du marché

### 2.1 Marché de référence

| Périmètre | Valeur | Source |
|---|---|---|
| Jeu vidéo en France, 2025 | 5,8 Md€ (+2,9 %) | SELL, *L’Essentiel du jeu vidéo*, avril 2026 |
| dont écosystème mobile | 1,792 Md€ (+11 %), 30 % du marché | SELL, avril 2026 |
| Joueurs en France, 2025 | 40,2 millions ; 59 % jouent sur mobile, parité femmes-hommes quasi parfaite | SELL et Médiamétrie, *Les Français et le jeu vidéo*, 2025 |
| Jeux de réflexion (puzzle) sur mobile, monde | de 7,7 à 8,8 Md$ de dépenses entre 2024 et 2025 | AppMagic, rapport de monétisation 2025 |
| Locuteurs de français dans le monde | 396 millions, dont 170 millions d’apprenants | OIF, *La langue française dans le monde*, 2026 |

### 2.2 Du marché total au marché atteignable

| Niveau | Définition | Estimation | Méthode |
|---|---|---|---|
| **Marché total** | Joueurs mobiles en France | environ 23,7 millions | 40,2 M × 59 % (SELL) |
| **Marché adressable** | Joueurs mobiles de jeux de réflexion, de lettres ou de culture générale | environ 7 à 9,5 millions | **hypothèse** : 30 à 40 % des joueurs mobiles |
| **Marché servi** | Parmi eux, les curieux de la langue et de l’histoire des mots, en France, Belgique, Suisse et Québec | environ 0,9 à 1,8 million | **hypothèse** : 10 à 15 % d’affinité, plus 30 % pour la francophonie du Nord |
| **Part visée à 18 mois** | Joueurs actifs mensuels (MAU) en mars 2028 | 6 000 à 10 000 | [plan financier](plan-financier.md), trois scénarios ; soit 0,5 à 1 % du marché servi |

Les 396 millions de francophones ne sont pas un marché à court terme : la majorité vit en Afrique, où la PWA légère est un atout, mais où le paiement par carte l’est beaucoup moins. C’est une piste de croissance pour l’audience, pas pour le revenu.

## 3. Tendances

1. **Le jeu de mots quotidien est devenu un rituel.** *Wordle*, racheté par le *New York Times* en 2022, a entraîné un catalogue qui dépasse le million d’abonnés (2024) ; en français, SUTOM, Cémantix et Pédantix sont joués chaque jour, gratuitement, sur le web. ÉtymoLogique partage ce rituel (deux plis par jour) mais y ajoute une progression durable.
2. **La gamification de la langue paie.** Duolingo compte 52,7 millions d’utilisateurs quotidiens et 12,2 millions d’abonnés payants fin 2025. Le public accepte de payer pour apprendre en jouant, surtout par abonnement.
3. **La collection à tirages est très rentable, et de plus en plus encadrée.** *Pokémon TCG Pocket* a rapporté environ 1,3 Md$ la première année, avec une mécanique proche de nos plis et sabliers (un sablier avance d’une heure la recharge des boosters). Mais la Belgique interdit les coffres à butin payants depuis 2018, et les autorités européennes de protection des consommateurs ont publié en 2025 des principes sur les monnaies virtuelles des jeux. Le choix d’ÉtymoLogique (pas de monnaie premium, prix en euros, chances affichées, plafonds) anticipe ce mouvement.
4. **Le web revient comme canal de distribution.** Les jeux quotidiens à succès sont nés dans le navigateur. Les règles européennes sur les marchés numériques facilitent le paiement hors magasins d’applications ; une PWA encaisse par carte (Stripe : 1,5 % + 0,25 € par paiement européen) au lieu de céder 15 à 30 % à Apple ou Google.

## 4. Concurrence

| Jeu | Modèle | Ce qu’il fait bien | Ce qui manque au regard d’ÉtymoLogique |
|---|---|---|---|
| *Little Alchemy*, *Infinite Craft* | Gratuit, web et mobile, publicité ou achat unique | Plaisir de la fusion, bouche-à-oreille | Combinaisons arbitraires ou générées, aucun fait vérifié, pas de rendez-vous. |
| SUTOM, Cémantix, Pédantix | Gratuit, web, sans compte | Rituel quotidien, partage des résultats | Une énigme par jour, pas de collection ni d’apprentissage durable. |
| Jeux du *New York Times* | Abonnement, quelques jeux gratuits | Marque, qualité éditoriale, catalogue | En anglais ; pas d’étymologie. |
| Duolingo | Gratuit avec abonnement | Rétention, gamification, notoriété | Apprentissage d’une langue étrangère, pas la culture de sa propre langue. |
| Projet Voltaire (Woonoz) | Abonnements particuliers et licences écoles et entreprises ; 16,1 M€ de CA en 2024 | Preuve qu’on vend la maîtrise du français, surtout en B2B | Orthographe et grammaire, pas de jeu de découverte. |
| *Wordscapes*, *Words of Wonders*, *Scrabble GO* | Gratuit avec publicité et achats | Volume massif, acquisition payante rodée | Pas de contenu culturel ; modèle publicitaire que nous écartons. |
| *Pokémon TCG Pocket* | Gratuit, monnaie premium, abonnement | Rituel d’ouverture, collection, rareté | Licence mondiale, monnaie premium et dépenses non plafonnées. |
| Applications de dictionnaires (Le Robert, Larousse) | Gratuit, publicité, premium | Autorité, audience de référence | Consultation, pas de jeu. Partenaires plus que concurrents. |

**Positionnement.** Sur deux axes, *rigueur éditoriale* et *profondeur de progression*, les jeux de mots quotidiens sont rigoureux mais sans progression, les jeux de fusion et de collection progressent mais sans rigueur. ÉtymoLogique occupe seul le quadrant « rigoureux et progressif ». Le risque est que ce quadrant soit vide parce qu’il est petit : c’est ce que le lancement doit mesurer.

## 5. Cibles

| Cible | Profil | Motivation | Rôle économique |
|---|---|---|---|
| **Curieux cultivés** (cœur) | 30-65 ans, lecteurs, joueurs de SUTOM ou Cémantix, auditeurs de radio culturelle | Comprendre d’où viennent les mots, compléter le codex | Premiers payeurs de sabliers, ambassadeurs. |
| **Étudiants** | Lettres, langues anciennes, classes préparatoires, concours | Réviser les racines grecques et latines en jouant | Audience et partage ; peu de dépense. |
| **Enseignants** | Français, lettres classiques, documentalistes | Un support sourcé et sans publicité pour la classe | Prescripteurs ; futurs clients d’une offre établissement. |
| **Apprenants avancés de français** | Niveau B2 et plus, dans le monde | Enrichir leur vocabulaire par les familles de mots | Audience internationale ; revenu faible à court terme. |
| **Familles** | Parents et adolescents de 13 ans et plus | Un jeu intelligent, sans achats cachés | Confiance ; contrôle parental exigé par l’[ADR 0018](adr/0018-sabliers-et-boutique.md). |

## 6. Forces, faiblesses, opportunités, menaces

| Forces | Faiblesses |
|---|---|
| Concept inédit, contenu sourcé, identité visuelle soignée | Niche : le goût de l’étymologie n’est pas universel |
| Rendez-vous mensuel (fascicules) et quotidien (plis) | Aucune notoriété, pas de visibilité dans les magasins d’applications |
| Coût d’infrastructure presque nul au repos (serverless) | Revenu par joueur volontairement bas (plafonds, pas de publicité) |
| Éthique affichée : pas de paiement au lancement, chances publiques | Charge éditoriale continue : environ un mot sourcé par jour |

| Opportunités | Menaces |
|---|---|
| Semaine de la langue française et de la Francophonie (chaque mois de mars), « Dis-moi dix mots » | Un grand acteur (éditeur de dictionnaires, *New York Times*) lance un jeu voisin |
| Aides publiques : CNC, Bpifrance, politique de la langue française | Clones libres rendus possibles par l’AGPL et la CC BY-SA (le nom reste protégé) |
| Partenariats avec médias culturels, bibliothèques, établissements | Règles sur les coffres à butin qui s’étendraient aux accélérateurs de tirage |
| Autres langues de jeu ([ADR 0006](adr/0006-architecture-multilingue.md)) | Lassitude si un fascicule se complète trop vite ou trop lentement |

## 7. Références économiques du secteur

| Indicateur | Référence | Source | Ce qu’on retient pour ÉtymoLogique |
|---|---|---|---|
| Rétention des jeux de réflexion | J1 31,85 %, J7 12,18 %, J30 5,35 % | GameAnalytics 2025 | Rétention mensuelle : 30 % le 2ᵉ mois, 10 % au 8ᵉ, soutenue par le fascicule mensuel. |
| Part de joueurs payeurs | 1,5 à 3,5 % des actifs | Rapports de monétisation 2025-2026 | 0,8 à 2,5 % : pas de relance commerciale, boutique discrète. |
| Dépense mensuelle par payeur, jeux de réflexion grand public | 8 à 15 $ | GameAnalytics 2025 | 3,50 à 6 € TTC : plafond de 12 sabliers par jour, aucune offre limitée. |
| Coût d’installation d’un jeu de réflexion | 3 $ sur iOS, 2 $ sur Android | Business of Apps, 2025 | Sans magasin, on raisonne en coût par joueur venu d’une publicité : 1,80 à 3,50 €. |
| Clic publicitaire sur Meta en France | 0,40 à 0,95 € | Junto, 2025 | Avec 25 à 35 % des clics qui commencent une partie. |
| Paiement par carte européenne | 1,5 % + 0,25 € | Stripe, 2026 | Pas de lot sous 1,99 € : la part fixe mangerait la marge. |
| Tarif journalier d’un développeur indépendant | React 500 à 600 €, Rust 550 à 750 € | Malt et baromètres 2026 | 550 € (front) et 650 € (back) dans le plan. |

### Grille de prix proposée pour les sabliers

L’[ADR 0018](adr/0018-sabliers-et-boutique.md) laisse les prix à une décision ultérieure. Proposition, à valider par ADR :

| Lot | Prix TTC | Prix par sablier | Équivaut à |
|---|---|---|---|
| 12 sabliers | 1,99 € | 0,17 € | un pli avancé de 12 h |
| 36 sabliers | 4,99 € | 0,14 € | le plafond de détention |

- Aucun lot sous 1,99 € : à 0,99 €, les frais de paiement et la TVA laisseraient environ 0,56 € sur 0,99 €.
- Dépense maximale théorique : 12 sabliers par 24 h, soit 360 par mois, environ 50 € ; le plafond mensuel par défaut proposé sur le web est de **20 €**, modifiable par le titulaire du moyen de paiement.
- Revenu net d’un panier moyen de 3,50 € TTC : environ 2,61 € après TVA (20 %) et frais de paiement, soit 75 %.

## 8. Canaux d’acquisition

| Canal | Pourquoi il convient | Coût | Priorité |
|---|---|---|---|
| Relations presse culturelle (radio, presse écrite, podcasts sur la langue) | Le sujet « d’où viennent les mots » est éditorialement attractif | Attaché·e de presse ponctuel·le | Haute, au lancement et chaque mois de mars |
| Créateurs de contenu sur la langue française (YouTube, TikTok, Instagram, newsletters) | Audience exactement ciblée ; partenariats rémunérés, signalés comme tels | 300 à 1 500 € par partenariat | Haute |
| Temps forts institutionnels : Semaine de la langue française, rentrée scolaire | Visibilité gratuite, crédibilité | Faible | Haute |
| Publicité Meta et TikTok vers la PWA | Ciblage par centres d’intérêt (lecture, langues, jeux de mots) | 1,80 à 3,50 € par joueur | Moyenne, à piloter au coût par joueur actif à 30 jours |
| Contenu éditorial propre (newsletter « l’histoire d’un mot ») | Référencement et fidélisation ; à écrire sur des mots **hors** catalogue, pour ne rien révéler du codex ([ADR 0015](adr/0015-codex-fascicules-et-legendaires.md)) | Temps interne | Moyenne |
| Enseignants et bibliothèques | Prescription durable, image | Faible | Moyenne, après le lancement |
| Magasins d’applications (PWA empaquetée) | Visibilité et recherche | Frais de compte, commission de 15 % | Basse : à reconsidérer après 12 mois |

Le partage des découvertes ne doit jamais révéler une carte : un partage montre la progression (« 18 / 24 cartes du fascicule 3 »), pas le mot trouvé.

## 9. Recommandations

1. **Lancer gratuit, mesurer, puis ouvrir la boutique.** Le MVP reste sans paiement ([ADR 0008](adr/0008-energie-et-plis.md)). Les trois premiers mois servent à mesurer la rétention mensuelle, le rythme de complétion d’un fascicule et le partage. La boutique n’ouvre qu’au 4ᵉ mois d’exploitation, après l’acceptation de l’ADR 0018, la décision sur les comptes ([ADR 0007](adr/0007-etat-et-economie-autoritaires.md)) et la revue juridique par territoire.
2. **Ne pas compter sur les sabliers seuls.** Avec les hypothèses centrales, un joueur actif rapporte environ 0,05 € HT par mois. Couvrir 17 000 à 27 000 € de coûts mensuels demande 340 000 à 530 000 joueurs actifs. Trois revenus complémentaires méritent chacun un ADR, dans le respect des piliers (rien de ce qui est vendu ne s’obtient pas aussi en jouant, aucune brique ni chance vendue) :
   - un **achat de soutien** ou des **cosmétiques** (jaquettes alternatives, thèmes), déjà évoqués et reportés par l’ADR 0018 ;
   - une **offre établissement** pour les écoles, bibliothèques et médiathèques (tableau de bord enseignant, accès sans compte individuel), sur le modèle B2B de Projet Voltaire ;
   - des **financements publics et participatifs** : aides du CNC, Bpifrance, appels à projets de la politique de la langue française, campagne de financement participatif avant le lancement.
3. **Tenir une structure légère.** La variante « structure allégée » du plan financier divise presque par deux le besoin de financement. Le serverless ([ADR 0024](adr/0024-architecture-logicielle-et-hebergement.md)) garde l’infrastructure autour de 100 € par mois jusqu’à 10 000 joueurs actifs.
4. **Piloter au coût par joueur actif, pas par joueur acquis.** Un joueur venu d’une publicité ne vaut que s’il revient au fascicule suivant : couper toute campagne dont le coût par joueur encore actif au 2ᵉ mois dépasse 10 €.
5. **Faire de mars le temps fort annuel.** La Semaine de la langue française et de la Francophonie tombe en M6 (mars 2027) et M18 (mars 2028) : y réserver les fascicules les plus marquants et le plus gros effort de presse.

## Sources

- SELL, [Bilan du marché français 2025](https://www.sell.fr/sites/default/files/essentiel-jeu-video/ejv_avril_2026_final_.pdf) (avril 2026) et [synthèse AFJV](https://afjv.com/news/11956_chiffres-marche-francais-jeux-video-sell.htm).
- SELL et Médiamétrie, [Les Français et le jeu vidéo 2025](https://afjv.com/news/11790_etude-mediametrie-sell-francais-jeux-video-2025.htm) ; [synthèse du Blog du Modérateur](https://www.blogdumoderateur.com/jeu-video-france-profils-joueurs-usages-2025/).
- OIF, [La langue française dans le monde 2026](https://www.francophonie.org/lancement-du-rapport-la-langue-francaise-dans-le-monde-2026-8426).
- GameAnalytics, [2025 Mobile Gaming Benchmarks](https://www.gameanalytics.com/reports/2025-mobile-gaming-benchmarks) et [2026 Mobile & PC Gaming Benchmarks](https://www.gameanalytics.com/reports/2026-mobile-pc-gaming-benchmarks).
- AppMagic, [Mobile Games Monetization Report 2025](https://gamedevreports.substack.com/p/appmagic-mobile-games-monetization).
- Business of Apps, [Cost per Install Rates 2025](https://www.businessofapps.com/ads/cpi/research/cost-per-install/).
- Junto, [Tarifs Meta Ads 2025](https://junto.fr/blog/tarifs-meta-ads).
- Stripe, [tarifs en France](https://affonso.io/resources/stripe-fee-calculator/france) (calculateur tiers, 2026).
- Malt, [baromètre des développeurs React](https://www.malt.fr/t/barometre-tarifs/tech/developpeur-frontend/developpeur-reactjs) ; [TJM Rust confirmé](https://travail-industrie.com/simulateur-tjm/developpement-it/rust-developer/confirme).
- Wikipédia, [Pédantix](https://fr.wikipedia.org/wiki/P%C3%A9dantix) ; [The New York Times Games](https://en.wikipedia.org/wiki/The_New_York_Times_Games) ; [Fast Company sur Wordle](https://www.fastcompany.com/91539885/wordle-statistics-show-why-new-york-times-is-turning-game-into-nbc-tv-show).
- Duolingo, [résultats financiers](https://investors.duolingo.com/news-releases/news-release-details/duolingo-adds-record-number-daus-surpasses-10-million-paid) et [synthèse 2025](https://alphabourse.substack.com/p/analyse-de-resultats-duolingo-duol-464).
- Woonoz (Projet Voltaire), [comptes publiés](https://www.pappers.fr/entreprise/woonoz-484528799).
- *Pokémon TCG Pocket* : [Udonis](https://www.blog.udonis.co/mobile-marketing/mobile-games/pokemon-tcg-pocket), [NintendoReporters](https://www.nintendoreporters.com/en/news/general/pokemon-tcg-pockets-first-year-13b-estimated-revenue-and-record-player-engagement/).
- CNC, [aides au jeu vidéo](https://www.cnc.fr/professionnels/aides-et-financements/jeu-video) et [crédit d’impôt jeu vidéo](https://www.cnc.fr/professionnels/aides-et-financements/jeu-video/credit-dimpot-jeu-video_121078).

Limites : plusieurs références (rétention, conversion, dépense par payeur) portent sur des jeux mobiles distribués en magasin, souvent anglophones ; elles servent d’ordre de grandeur. Les nombres de joueurs de Cémantix et SUTOM ne sont pas publiés.
