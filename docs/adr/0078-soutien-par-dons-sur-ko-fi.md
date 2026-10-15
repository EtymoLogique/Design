# ADR 0078 — Soutien par dons sur Ko-fi, sans vente dans le jeu

- **Statut** : Proposé
- **Portée** : PWA (Réglages, navigation, modale), API (un webhook), page des données personnelles
- **Remplace** : l’idée d’une boutique en euros (abandonnée le 15 octobre 2026, ADR 0073 supprimé, ADR 0076 et 0077 jamais acceptés)
- **Complète** : [ADR 0008](0008-energie-et-plis.md) et [ADR 0011](0011-plis-raretes-et-doublons.md) (aucun paiement dans le jeu), [ADR 0070](0070-comptes-facultatifs-par-lien-et-code-e-mail.md) (aucune adresse conservée en clair, empreinte HMAC), [ADR 0010](0010-observabilite-et-vie-privee.md) (aucune adresse dans les journaux), [ADR 0024](0024-architecture-logicielle-et-hebergement.md) (aucun script tiers dans la PWA)

## Contexte

La boutique de gouttes en euros demandait un prestataire de paiement, la TVA, le droit de rétractation, un reçu sur support durable et un compte obligatoire. Le projet y renonce. Il garde un moyen de financer le jeu : des dons libres sur [Ko-fi](https://ko-fi.com/lunik_tiwabbit), page d’un particulier.

Ko-fi n’est pas le marchand de référence : il ne collecte ni TVA ni impôt, et le don arrive directement sur le PayPal ou le Stripe de l’éditeur. Tant que rien n’est vendu, le jeu ne porte aucune des obligations de la vente à distance. Le statut fiscal de l’éditeur (don occasionnel ou activité habituelle) reste à faire confirmer par le SIP ou un comptable ; il ne change pas ce qui est construit ici.

## Décision

### Un don ne donne rien dans le jeu

- Aucune goutte, aucun sablier, aucune brique, aucun pli, aucune chance ni rareté ne s’obtient en donnant. Un abonné joue exactement comme les autres.
- Le seul effet d’un abonnement est de **ne plus voir la modale de rappel**. Le jeu ne le présente jamais comme une contrepartie.
- Seuls les dons libres et les abonnements de la page Ko-fi sont utilisés. Ni boutique Ko-fi, ni commande, ni « membership » à avantages : ils engageraient la TVA.

### Où le soutien se présente

- **Réglages** : une carte « Soutenir ÉtymoLogique » tout en haut, avec le lien. Pour un abonné, le même emplacement remercie. Un bouton « Masquer pendant 30 jours » la cache, ici comme dans la Boutique ; elle revient d’elle-même après 30 jours réels (horodatage local, comme le reste du souvenir de la modale).
- **Boutique** : la même carte sous les échanges, même si rien ne s’y achète : le don n’est pas une offre de la boutique, il le dit (« ne change rien à votre partie »).
- **Navigation** : un lien « Soutenir » discret dans le rail des écrans larges. Sur mobile, la barre garde ses quatre onglets.
- **Modale de rappel** : sur la table au calme, jamais pendant un rituel de pli, une fusion, une fiche ou la relance « Sauvegardez votre progression », et seulement en ligne. Première apparition à partir de 5 découvertes et de 3 jours de jeu, puis au plus tous les 30 jours de jeu. « Plus tard » comme « Offrir un café » la ferment ; le souvenir est local (IndexedDB), pas une donnée du serveur.
- Le lien est un simple `<a href>` vers Ko-fi, ouvert hors de la PWA : aucun script ni cadre tiers, la CSP ne change pas.

### Reconnaître un abonné

- Ko-fi appelle `POST /soutien/kofi` à chaque paiement (webhook, `data=<JSON>` en formulaire). L’API compare le `verification_token` au secret `ETYMO_KOFI_TOKEN` à temps constant ; sans lui, le point d’entrée répond `503`.
- Seuls les paiements d’abonnement (`is_subscription_payment`) comptent. Un don ponctuel ne retire pas la modale.
- L’adresse de l’événement est normalisée comme pour la connexion, puis réduite à son **empreinte HMAC** (la clé de l’ADR 0070). La table `soutien` ne garde que cette empreinte et la date du dernier paiement : ni nom, ni montant, ni message.
- Un abonnement est **actif 45 jours** après son dernier paiement. La ligne est supprimée avec le compte, et les lignes de plus de 90 jours sont purgées à chaque webhook.
- `GET /etat` renvoie `compte.soutien` (vrai ou faux). Un invité sans compte n’est jamais reconnu : il voit la modale.
- Limite connue : la détection ne marche que si l’adresse de l’abonnement Ko-fi est celle du compte.

### Données personnelles

- La page des données personnelles et les CGU le disent : le lien quitte le jeu, Ko-fi et PayPal ou Stripe sont responsables de leur traitement, le jeu ne leur envoie rien, et il ne reçoit l’adresse d’un abonné que le temps de calculer son empreinte.
- Le corps du webhook n’est jamais journalisé. Le contrat accepte les champs inconnus de ce seul point d’entrée (Ko-fi en ajoute), au contraire du reste de l’API.

## Options envisagées

- **Rien : seulement un lien dans les Réglages.** Plus simple, mais aucun rappel : peu de dons.
- **Modale sans reconnaître les abonnés.** Sans donnée de plus, mais un abonné serait relancé. Écartée.
- **Bouton « J’ai déjà soutenu » local.** Sans serveur ni donnée, mais déclaratif. Possible plus tard en complément pour qui s’abonne avec une autre adresse.
- **Contrepartie dans le jeu** (gouttes, cosmétique). Recrée une vente : TVA, rétractation, monnaie de jeu. Écartée.

## Conséquences

- (+) Aucune vente : « gratuit, sans publicité, aucun avantage payant » reste vrai.
- (+) Un seul point d’entrée serveur, une table de deux colonnes, un secret.
- (−) Le secret du webhook se saisit à la main dans Ko-fi puis dans Secret Manager : il n’est pas généré par l’infrastructure.
- (−) Un abonné dont l’adresse Ko-fi diffère de celle du compte voit encore la modale.
- (−) Une empreinte d’adresse reste une donnée pseudonyme : la page des données personnelles l’explique.

## Critères de réévaluation

- Plus d’un tiers des abonnés relancés malgré leur abonnement : ajouter « J’ai déjà soutenu ».
- Dons récurrents réguliers : faire confirmer le statut (micro-entreprise, TVA) avant d’aller plus loin.
- Toute proposition de contrepartie dans le jeu : nouvel ADR, avec la TVA et la monnaie de jeu.
