# ADR 0071 — Durée de conservation des comptes inactifs et de l’historique des plis

- **Statut** : Proposé
- **Portée** : données du joueur, purges quotidiennes, textes légaux et Réglages de la PWA
- **Remplace partiellement** : [ADR 0070](0070-comptes-facultatifs-par-lien-et-code-e-mail.md), section « Supprimer, purger », pour la phrase « Un compte n’est jamais effacé automatiquement »
- **Complète** : [ADR 0010](0010-observabilite-et-vie-privee.md) (« applique des durées de conservation documentées »), [ADR 0024](0024-architecture-logicielle-et-hebergement.md) (durées de conservation), [ADR 0050](0050-cohortes-de-retention-et-statistiques-du-joueur.md) (agrégats d’équilibrage)

## Contexte

Le RGPD demande une durée de conservation limitée et justifiée pour chaque donnée (art. 5.1.e). Deux données n’en ont pas :

- **les comptes** : l’[ADR 0070](0070-comptes-facultatifs-par-lien-et-code-e-mail.md) dit « Un compte n’est jamais effacé automatiquement » ;
- **l’historique des plis** (`ouverture_pli`) : jamais purgé.

Le jeu ne garde pas l’adresse ([ADR 0070](0070-comptes-facultatifs-par-lien-et-code-e-mail.md)) : il ne peut donc pas prévenir un joueur avant d’effacer son compte. L’avertissement doit passer par le jeu lui-même et par les textes légaux.

Les autres données ont déjà leur durée : commandes 30 jours, quêtes du jour et réponses de ricochets 30 jours, demandes de connexion 1 jour, parties invitées inactives 6 mois, télémétrie brute 90 jours ([ADR 0024](0024-architecture-logicielle-et-hebergement.md)). Le reste est la progression, qui vit autant que le profil.

## Décision

### Un compte inactif 3 ans est effacé

- Un **compte** (le joueur et toute sa progression, sur tous ses appareils) est effacé par les purges quotidiennes quand **aucun de ses appareils n’a joué depuis 3 ans**.
- Est une activité toute session ouverte avec le serveur par un appareil du joueur : une synchronisation, une commande, une connexion. Elle repousse l’effacement de 3 ans à compter de ce jour.
- Justification : la progression sert à reprendre la partie. Trois ans sans jouer laissent trois ans de fascicules sans aucune visite. Passé ce délai, la reprise est improbable, et une base de joueurs partis n’a plus de finalité. Un invité sans compte garde son délai de **6 mois**, plus court, faute d’adresse pour le retrouver.
- Aucun achat en euros n’existe, et il n’y en aura pas.
- Les 3 ans sont **à confirmer avec le conseil** avant la bêta ouverte ([ADR 0048](0048-beta-ouverte-et-ouverture-a-quatre-fascicules.md)). Le chiffre vit en un seul endroit du code des purges (`MOIS_COMPTE_INACTIF`).

### Le joueur est prévenu sans e-mail

Aucune adresse n’est gardée : l’annonce se fait là où le joueur passe et là où le droit le demande.

- **Réglages**, entrée « Mon compte » : une ligne « Un compte inactif depuis 3 ans est effacé. »
- **Carte de relance de l’invité** ([ADR 0070](0070-comptes-facultatifs-par-lien-et-code-e-mail.md)) : elle garde « une partie invitée inactive 6 mois est effacée ». On n’y ajoute rien : elle s’adresse à un invité.
- **Conditions générales et politique de confidentialité** : la durée, le critère d’activité et la manière de s’y soustraire (rouvrir le jeu).
- **Au retour** : un joueur qui revient avant 3 ans ne perd rien. Après, il repart d’un profil invité neuf ; l’adresse n’étant pas gardée, rien ne le relie à l’ancien compte.

Le texte à montrer :

> **Conservation de votre compte.** Vous n’avez rien à faire pour le garder : il suffit de jouer de temps en temps. Si aucun de vos appareils n’a joué pendant 3 ans, votre compte et votre progression sont effacés. Nous ne vous écrivons pas avant : nous ne gardons pas votre adresse électronique. Vous pouvez aussi effacer votre compte à tout moment, dans les Réglages.

### L’historique des plis est agrégé après 12 mois

- Le détail de `ouverture_pli` (brique tirée, résultat, règle appliquée) sert au **diagnostic de l’équilibrage** : garanties, filets, taux de doublons ([ADR 0008](0008-energie-et-plis.md), [ADR 0016](0016-plis-et-jaquettes-par-fascicule.md)). Ce besoin porte sur quelques semaines, pas sur des années.
- Les lignes de plus de **12 mois** sont **agrégées** par fascicule, rareté, résultat et règle appliquée, puis effacées. L’agrégat ne contient aucun identifiant de joueur et suit le seuil de 50 joueurs de l’[ADR 0050](0050-cohortes-de-retention-et-statistiques-du-joueur.md).
- Le joueur n’a pas besoin du détail : la réserve, le codex et `compteur_pli` tiennent son état. Le détail disparaît aussi avec le joueur, par cascade.

### Chaque table de joueur a sa durée

| Donnée | Durée |
|---|---|
| `joueur`, `appareil`, `compte` | invité : 6 mois sans activité ; compte : 3 ans sans activité ; ou à la demande du joueur |
| progression (`exemplaire`, `decouverte`, `energie`, `encre`, `sablier`, `compteur_pli`, `indice`, `jalon_atteint`, `palier_atteint`, `quete_initiale`, `graine_accordee`, `fascicule_debloque`, `carte_lecteur`, `ricochet`, `reglage`, `migration`) | tant que le joueur existe : elle est la partie |
| `mouvement` | tant que le joueur existe : journal du support et des litiges |
| `ouverture_pli` | détail 12 mois, puis agrégat sans identifiant |
| `commande` | 30 jours |
| `quete_du_jour`, `ricochet_jour` | 30 jours |
| `defi_connexion` | 1 jour |
| télémétrie brute | 90 jours, puis agrégats seuls |

## Options envisagées

### Conserver les comptes sans limite, avec une justification écrite

Écartée : la progression n’a plus de finalité après des années d’absence, et le jeu ne peut pas demander son accord à un joueur qu’il ne sait pas joindre. L’effacement est le moindre risque.

### Une durée plus courte (12 mois)

Écartée : un joueur de collection revient souvent après une longue pause, attiré par un fascicule. 12 mois effaceraient des parties que l’on veut reprendre, sans prévenir.

### Garder l’adresse pour prévenir avant l’effacement

Écartée : ce serait contredire l’[ADR 0070](0070-comptes-facultatifs-par-lien-et-code-e-mail.md) et faire de la base une liste d’adresses à protéger, pour un message par joueur et par vie.

### Purger l’historique des plis sans l’agréger

Écartée : l’équilibrage a besoin de mesures sur la durée, par exemple l’effet d’un nouveau fascicule sur les doublons. L’agrégat les garde sans identifiant.

### Garder le détail des plis aussi longtemps que le compte

Écartée : c’est un historique fin de comportement, sans usage pour le joueur, qui croît à chaque pli.

## Conséquences

### Positives

- Chaque donnée a une durée écrite et justifiée ; la page des données collectées peut la citer.
- La base ne grossit plus sans fin avec des joueurs partis et leurs plis.
- Aucune adresse n’est ajoutée pour prévenir.
- L’issue d’implémentation de l’App (EtymoLogique/App#68) a toutes ses durées.

### Négatives

- Un joueur absent 3 ans perd sa partie, sans avertissement individuel.
- Les purges portent une tâche de plus (agrégation des plis) et un critère d’activité à définir côté serveur.
- Un litige ancien sur un achat futur ne pourra pas s’appuyer sur `mouvement` au-delà de la vie du compte : à revoir à l’ouverture de la boutique.

## Critères de réévaluation

- Avis du conseil sur les 3 ans, ou sur l’agrégation à 12 mois : ajuster les durées.
- Plus de 2 % des comptes effacés qui rouvrent un profil neuf dans les 30 jours : allonger la durée ou prévoir un rappel dans le jeu.
- Ajout d’un historique fin pour le joueur (journal de ses plis) : revoir la durée de `ouverture_pli`.
