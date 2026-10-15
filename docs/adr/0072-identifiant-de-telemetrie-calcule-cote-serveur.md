# ADR 0072 — L’identifiant de télémétrie est calculé côté serveur, sur intérêt légitime, avec opposition dans les Réglages

- **Statut** : Proposé
- **Portée** : télémétrie produit, base légale, information et opposition du joueur
- **Précise** : [ADR 0050](0050-cohortes-de-retention-et-statistiques-du-joueur.md) (renouvellement au mois civil : où vit l’identifiant)
- **Complète** : [ADR 0010](0010-observabilite-et-vie-privee.md) (« le produit explique la collecte et offre les contrôles requis »), [ADR 0024](0024-architecture-logicielle-et-hebergement.md) (la télémétrie passe par l’API)

## Contexte

L’identifiant pseudonyme de la télémétrie se renouvelle chaque mois civil ([ADR 0050](0050-cohortes-de-retention-et-statistiques-du-joueur.md)). Aucun ADR ne dit **où il vit**, et c’est ce qui fixe le régime juridique (écart L15 de l’audit juridique de l’App, qui complète l’écart E3 de l’audit avant lancement) :

- **écrit sur l’appareil** : c’est un traceur. Il faut soit l’exemption de la CNIL pour la mesure d’audience (finalité limitée, aucun recoupement, aucun tiers, traceur de 13 mois au plus, données de 25 mois au plus, information et opposition), soit un consentement ;
- **calculé côté serveur** : le serveur voit déjà chaque fusion, indice et pli, puisqu’il fait foi ([ADR 0007](0007-etat-et-economie-autoritaires.md)). Rien n’est lu ni écrit sur l’appareil pour la mesure : l’article 82 de la loi Informatique et Libertés ne s’appliquerait sans doute pas. Restent le RGPD (intérêt légitime), l’information et un droit d’opposition.

Les documents existants se contredisent sur la durée : l’[ADR 0024](0024-architecture-logicielle-et-hebergement.md) et la roadmap de l’App (jalon J7) disent « tous les 30 jours », l’ADR 0050 dit « au mois civil ». Le mois civil l’emporte : l’ADR 0050 en donne la raison (un joueur compté deux fois à cheval sur deux mois).

Ce relevé n’est pas un avis juridique : chaque règle ci-dessous est **à confirmer avec le conseil** avant la bêta ouverte ([ADR 0048](0048-beta-ouverte-et-ouverture-a-quatre-fascicules.md)).

## Décision

### Le serveur calcule l’identifiant, l’appareil ne le voit jamais

- L’identifiant pseudonyme d’un événement est une **empreinte à clé** (HMAC-SHA-256) de l’identifiant d’installation et du mois civil de l’événement, avec une clé secrète du serveur. Deux mois, deux identifiants sans lien lisible entre eux.
- La clé vit dans les secrets du serveur, hors de la base de télémétrie. Aucune table ne fait correspondre un identifiant pseudonyme à un joueur : le serveur le **recalcule** quand il en a besoin (par exemple pour l’opposition).
- Les événements naissent sur le serveur, qui fait foi : fusion, indice, pli. Les rares événements vus seulement par l’appareil (étapes du tutoriel, erreur technique) arrivent par l’API, dans une session déjà ouverte ; le serveur y ajoute l’identifiant. **Rien n’est écrit sur l’appareil pour la mesure** : ni identifiant, ni cookie, ni entrée de stockage local, ni file d’événements en attente. Un événement qui ne peut pas partir tout de suite est abandonné.
- Les propriétés restent celles de l’[ADR 0010](0010-observabilite-et-vie-privee.md), plus le mois d’arrivée ([ADR 0050](0050-cohortes-de-retention-et-statistiques-du-joueur.md)). Aucune adresse réseau complète n’est gardée.

### Base légale : l’intérêt légitime de l’éditeur

- **Finalité** : mesurer l’équilibrage, la rétention et les erreurs du jeu, pour décider des fascicules, des plis et des quêtes ([ADR 0050](0050-cohortes-de-retention-et-statistiques-du-joueur.md), tableau des indicateurs). Aucune publicité, aucun profilage, aucun tiers.
- **Base** : intérêt légitime (RGPD, art. 6, 1, f). Pas de consentement : l’événement ne dit rien que le serveur n’a déjà.
- **Garde-fous** qui font l’équilibre : pseudonyme à clé renouvelé chaque mois, propriétés énumérées, télémétrie brute **90 jours** puis agrégats seuls seuillés à 50 joueurs ([ADR 0024](0024-architecture-logicielle-et-hebergement.md), [ADR 0050](0050-cohortes-de-retention-et-statistiques-du-joueur.md)), tables séparées de la progression, accès tracés.
- **Le joueur s’oppose sans condition** (RGPD, art. 21) : voir ci-dessous.
- Les tâches de rétention de l’ADR 0050 (`cree_le`, `derniere_activite`, `mois_activite_precedente`) ne dépendent pas de la télémétrie : elles lisent la table `joueur`, dont la conservation est réglée par l’[ADR 0071](0071-duree-de-conservation-des-comptes-et-des-plis.md). Elles ne produisent que des nombres seuillés ; l’opposition ne les change pas. **À confirmer avec le conseil.**

### Information du joueur

- **Au premier lancement** : une ligne sous l’écran d’accueil de la première session, « Le jeu mesure son usage, sans vous identifier. Détails et refus dans les Réglages. », avec un lien vers « Données collectées ».
- **Réglages, « Données collectées »** : le texte ci-dessous et l’interrupteur d’opposition.
- **Politique de confidentialité** : la finalité, la base légale, la durée (90 jours puis agrégats), l’absence de tiers et de traceur, le droit d’opposition et celui d’introduire une réclamation auprès de la CNIL.

Le texte à montrer :

> **Mesure de l’usage.** Pour équilibrer le jeu, nous comptons ce qui s’y passe : fusions, indices, plis, erreurs. Chaque mois, un identifiant nouveau, sans lien avec le précédent et sans votre nom, est calculé sur nos serveurs. Rien n’est écrit sur votre appareil pour cela. Les événements sont gardés 90 jours, puis seuls restent des totaux d’au moins 50 joueurs. Personne d’autre que nous ne les reçoit. Vous pouvez refuser à tout moment.

### L’interrupteur d’opposition

- Un interrupteur **« Mesure de l’usage »** dans « Données collectées » ([interfaces](../interfaces.html#i-24)), **actif par défaut**, à effet immédiat, avec la sous-ligne « Aide à équilibrer le jeu. Sans nom, sans tiers. ». Même cible tactile et même forme que les autres interrupteurs ([design système](../design-system.html#ds-2-4)).
- Il est **enregistré côté serveur**, dans `reglage`, pour le profil du joueur : il vaut sur tous ses appareils et ne demande aucun stockage local.
- **Désactivé** : le serveur n’émet plus aucun événement pour ce joueur, qu’il vienne du serveur ou de l’appareil. Il efface aussi les événements bruts des 90 derniers jours : il recalcule les identifiants pseudonymes de ces mois pour ce joueur, puis supprime les lignes. Les agrégats déjà calculés sont anonymes et restent.
- **Réactivé** : la mesure reprend, sans rattacher les événements effacés.
- Le jeu fonctionne de la même façon, interrupteur actif ou non : aucune fonction, aucun cadeau ni contenu n’en dépend.
- La suppression d’un compte ou d’un profil invité ([ADR 0070](0070-comptes-facultatifs-par-lien-et-code-e-mail.md), [ADR 0071](0071-duree-de-conservation-des-comptes-et-des-plis.md)) efface de même les événements bruts restants.

### Alignement des documents

- [ADR 0024](0024-architecture-logicielle-et-hebergement.md) : « change tous les 30 jours » devient « change chaque mois civil, calculé côté serveur » (correction factuelle).
- Roadmap de l’App, jalon J7 : « renouvelé tous les 30 jours » devient « renouvelé chaque mois civil, calculé côté serveur », avec l’interrupteur d’opposition et le texte d’information.
- Audit avant lancement de l’App, E3 : le texte et le comportement sont ici.
- [Game design](../game-design.md), section de l’observabilité : le lieu de l’identifiant et l’opposition.

## Options envisagées

### Un identifiant écrit sur l’appareil, sous l’exemption de mesure d’audience de la CNIL

Écartée : c’est un traceur. L’exemption impose une finalité limitée à la mesure d’audience, des sorties anonymes et une durée de 13 mois au plus ; la télémétrie du jeu (équilibrage, erreurs) dépasse la simple audience. Le serveur n’a pas besoin de l’appareil pour compter.

### Un identifiant sur l’appareil, avec consentement

Écartée : un bandeau à l’ouverture pèse sur une première session que le jeu veut douce ([ADR 0046](0046-demarrage-genereux-cadeau-de-bienvenue-et-quetes-initiales.md)), et les joueurs qui refusent trouent la mesure de rétention, qui sert justement de seuil à la bêta.

### Réutiliser l’identifiant du joueur comme identifiant de télémétrie

Écartée : l’[ADR 0024](0024-architecture-logicielle-et-hebergement.md) le veut distinct, et l’[ADR 0050](0050-cohortes-de-retention-et-statistiques-du-joueur.md) écarte tout identifiant durable.

### Une table de correspondance entre identifiants pseudonymes et joueurs

Écartée : elle rendrait la pseudonymisation réversible pour quiconque lit la base. Le recalcul par la clé suffit à l’opposition.

### Une opposition par défaut (mesure désactivée)

Écartée : la mesure n’aurait de sens que pour une minorité, au risque de fausser les cohortes. L’opposition en un geste est le contrôle que l’intérêt légitime demande.

## Conséquences

### Positives

- Aucun traceur : pas de bandeau, pas de consentement à gérer.
- La télémétrie garde ses cohortes mensuelles et son mois d’arrivée ([ADR 0050](0050-cohortes-de-retention-et-statistiques-du-joueur.md)).
- L’opposition est réelle : elle arrête la collecte et efface le brut.
- Les trois documents (ADR, roadmap, game design) disent la même chose.

### Négatives

- Un secret de plus à garder, à sauvegarder et à faire tourner avec soin : le perdre brouille les recalculs pour l’opposition.
- Un joueur qui s’oppose pèse moins dans les mesures : la base est plus petite que le nombre de joueurs.
- Le serveur voit tout, ce qui est inchangé mais doit maintenant être dit au joueur.
- L’avis du conseil peut changer la base légale, le texte ou la durée.

## Critères de réévaluation

- Avis du conseil sur l’intérêt légitime, la non-application de l’article 82 ou les textes : ajuster la base, la durée ou le texte.
- Plus de 10 % de joueurs qui s’opposent : revoir l’information ou la finalité.
- Ajout d’un événement vu seulement par l’appareil et en grand nombre : revoir l’absence de file locale.
- Tout tiers, session replay ou traceur de plus : revue dédiée de la vie privée ([ADR 0010](0010-observabilite-et-vie-privee.md)).
