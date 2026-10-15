# Conformité RGPD : registre des traitements, violation de données, mineurs

Documents internes exigés par le RGPD (écart L11 de l’[audit juridique](https://github.com/EtymoLogique/App/blob/develop/docs/audits/juridique.md), EtymoLogique/Design#37). Ils décrivent l’état prévu à la **bêta ouverte** (janvier 2027). Ils ne contiennent aucune information interne sensible : ils peuvent rester dans ce dépôt.

Ce n’est pas un avis juridique. Chaque ligne est à faire confirmer par le conseil (2 500 € en M3, plan financier). Tenez ce document à jour à chaque ADR qui ajoute, change ou retire un traitement.

- **Responsable du traitement** : l’éditeur du service, à désigner avant la bêta (EtymoLogique/Design#35). Tant qu’il n’est pas choisi, les cellules « contact » restent à remplir.
- **Sous-traitant** : Scaleway SAS (hébergement, envoi d’e-mails), en France, sans transfert hors de l’Union. Son accord de sous-traitance (art. 28) fait partie de ses conditions générales : l’archiver et vérifier qu’il couvre l’envoi d’e-mails.
- **Vocabulaire** : les données des joueurs sont **pseudonymisées**, jamais « anonymes ». Seuls les agrégats à seuil de 50 le sont ([ADR 0050](adr/0050-cohortes-de-retention-et-statistiques-du-joueur.md)).

## 1. Registre des traitements (art. 30)

Un traitement par ligne. Les durées « à fixer » sont des décisions ouvertes, à trancher avant la bêta.

| Traitement | Finalité | Base légale | Données | Durée | Destinataires |
|---|---|---|---|---|---|
| Partie invitée | Fournir le jeu sans compte | Exécution du service demandé (art. 6.1.b) | Identifiant d’installation, empreinte du secret, progression, dates d’activité | 6 mois sans activité, puis effacement par la purge quotidienne | Scaleway (hébergement) |
| Compte | Récupérer la progression, jouer sur plusieurs appareils, acheter | Exécution du contrat (6.1.b) | Empreinte HMAC de l’adresse, pseudo (privé tant que non confirmé), appareils rattachés | À fixer : effacement après une inactivité longue et annoncée (3 ans proposés) | Scaleway |
| Envoi du code de connexion | Authentifier le joueur | Exécution du contrat (6.1.b) | Adresse en clair le temps de l’envoi ; défi en empreintes | Défi : 1 jour. E-mail chez Scaleway : durée à obtenir | Scaleway (envoi d’e-mails) |
| Sécurité et limitation de débit | Empêcher les abus | Intérêt légitime (6.1.f) | Adresse réseau, en mémoire seulement | Vie de l’instance. Journaux d’accès Scaleway : durée à obtenir | Scaleway |
| Historique des commandes, quêtes et ricochets du jour | Idempotence et support | Exécution du contrat (6.1.b) | Identifiant, contenu de la commande | 30 jours | Scaleway |
| Historique des plis | Diagnostiquer l’équilibrage | Intérêt légitime (6.1.f) | Brique tirée, règle, date | À fixer | Scaleway |
| Télémétrie (J7) | Mesurer l’usage et la rétention | Intérêt légitime (6.1.f), régime à confirmer (audit § 4) | Identifiant pseudonyme renouvelé chaque mois civil, événements | 90 jours de données brutes | Scaleway |
| Statistiques de rétention | Piloter la bêta | Intérêt légitime (6.1.f) | Agrégats sans identifiant, seuil de 50 | Anonymes, durée libre | Aucun |
| Achats de gouttes (boutique, M7) | Vendre et rembourser | Exécution du contrat (6.1.b), obligation légale (6.1.c) | Identifiant de l’achat chez le prestataire, montant, date, consentements ; **aucune adresse ni donnée de carte** | À fixer avec l’[ADR 0073](adr/0073-support-durable-et-pieces-comptables-de-la-boutique.md) ; pièces comptables 10 ans (C. com., art. L123-22) | Prestataire de paiement (responsable distinct), Scaleway |
| Pseudos visibles (plus tard) | Vue compétitive | À définir | Pseudo confirmé | À définir | Autres joueurs. Revue dédiée avant ouverture ([ADR 0075](adr/0075-moderation-des-pseudos-et-obligations-dsa.md)) |

Stockage sur l’appareil : identifiant et secret d’installation, cache de la partie, réglages, cache du service worker. Tous sont strictement nécessaires au service demandé (art. 82 de la loi Informatique et Libertés) : pas de bandeau tant que la télémétrie n’écrit rien sur l’appareil.

Droits des personnes : accès, portabilité, effacement, opposition. Une demande arrive par l’adresse de contact ; avant de répondre, vérifier que le demandeur contrôle l’adresse (code de connexion). L’empreinte HMAC retrouve le compte sans conserver l’adresse. Délai de réponse : un mois.

## 2. Procédure de violation de données (art. 33 et 34)

**Définition.** Une violation est toute destruction, perte, altération, divulgation ou accès non autorisé à des données des joueurs, par accident ou par malveillance. Deux incidents comptent **toujours**, même sans fuite constatée :

- la perte ou la fuite de `ETYMO_CLE_COURRIEL` (clé de l’empreinte HMAC des adresses) : elle permet de retrouver un compte à partir d’une adresse ;
- la perte ou la fuite de `ETYMO_CLE_SIGNATURE_SESSION` : elle permet de forger des sessions.

Une fuite de la base livrerait peu (aucune adresse, aucun mot de passe, aucune donnée de carte), mais reste une violation.

**Étapes**

1. **Détecter et contenir** (immédiat). Qui constate l’incident prévient la personne d’astreinte. Couper l’accès en cause : révoquer la clé ou le jeton, isoler l’instance, changer les secrets dans Secret Manager.
2. **Consigner** (dans l’heure). Ouvrir une ligne dans le registre des incidents (§ 3) : date et heure de la découverte, nature, données et nombre de joueurs touchés, cause connue.
3. **Évaluer le risque** (sous 24 h). Les données exposées permettent-elles de remonter à une personne, de se faire passer pour elle ou de lui nuire ? Une clé HMAC perdue sans la base ne l’autorise pas ; avec la base, si. Conclure : pas de risque, risque, risque élevé.
4. **Notifier la CNIL** (sous 72 h après la découverte) dès qu’il y a un risque pour les personnes, par le téléservice de notification de la CNIL. Si tout n’est pas connu, notifier en plusieurs fois. Pas de risque : ne pas notifier, mais justifier par écrit dans le registre.
5. **Informer les joueurs** (sans délai) si le risque est **élevé** : message dans l’application (le jeu ne peut pas écrire aux joueurs hors connexion, [ADR 0070](adr/0070-comptes-facultatifs-et-connexion-par-courriel.md)) et sur la vitrine, en langage clair : ce qui s’est passé, ce qui est touché, ce qu’il faut faire.
6. **Corriger et clore.** Rotation des clés touchées, correctif, test de non-régression. Consigner les mesures et la date de clôture.
7. **Prévenir les sous-traitants et partenaires** concernés (Scaleway, prestataire de paiement) si l’incident vient d’eux ou les touche.

**Contacts** (à compléter avant la bêta)

| Rôle | Contact |
|---|---|
| Astreinte technique | À désigner |
| Responsable du traitement | À désigner (EtymoLogique/Design#35) |
| CNIL | Téléservice de notification des violations de données personnelles |
| Hébergeur | Support Scaleway |
| Conseil juridique | À désigner |

## 3. Registre des incidents

Un incident par ligne, même s’il n’a pas été notifié.

| Date | Nature | Données touchées | Risque | CNIL | Joueurs | Mesures | Clôture |
|---|---|---|---|---|---|---|---|
| — | Aucun incident à ce jour | — | — | — | — | — | — |

## 4. Analyse d’impact (AIPD) : décision

**Décision : pas d’AIPD à la bêta.** Le traitement ne cumule aucun critère retenu par la CNIL : pas de profilage ni de décision automatisée, pas de données sensibles, pas de surveillance systématique, pas de croisement de jeux de données, pas de collecte à grande échelle de personnes vulnérables. Les données sont minimisées et pseudonymisées, la télémétrie est agrégée avec un seuil de 50.

**À reconsidérer** (nouvelle décision écrite avant chacun) :

- l’offre aux écoles de l’étude de marché (données d’élèves, mineurs) ;
- l’ouverture des pseudos à d’autres joueurs ([ADR 0075](adr/0075-moderation-des-pseudos-et-obligations-dsa.md)) ;
- tout traitement qui profile un joueur ou conserve une adresse.

## 5. Mineurs

- Le jeu attirera des collégiens et des lycéens. La base légale est l’exécution du service, pas le consentement : l’âge de 15 ans de l’article 45 de la loi Informatique et Libertés ne s’applique pas directement.
- **L’information doit être compréhensible par un mineur** (art. 12) : phrases courtes, vocabulaire simple, pas de jargon juridique, dans la politique de confidentialité et à l’écran de création du compte.
- La boutique reste cadrée par les règles de l’[ADR 0071](adr/0071-boutique-de-gouttes-en-euros.md) : achats désactivés par défaut, plafond mensuel, rétractation.
- **Aucune offre aux écoles sans revue dédiée** : contrat avec l’établissement, rôle de responsable ou de sous-traitant, données d’élèves, AIPD à refaire.
- Ne pas s’engager sur un contrôle d’âge qu’une PWA ne peut pas tenir.
