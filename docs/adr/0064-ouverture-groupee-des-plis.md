# ADR 0064 — Ouvrir tous les plis prêts d’un coup, en canon

- **Statut** : Proposé
- **Complète** : [ADR 0011](0011-plis-raretes-et-doublons.md) (le tirage d’un pli reste inchangé) et [ADR 0020](0020-deux-plis-en-attente-et-sabliers.md) (les plis en attente)

## Contexte

Quand plusieurs plis attendent le joueur (la capacité d’énergie peut monter à dix), il doit les ouvrir un à un : chaque ouverture dure 1,5 à 2 secondes et finit par un écran à passer. Dix plis, c’est plus de vingt secondes de rituel identique, au moment où le joueur veut surtout retrouver ses briques.

Le rituel d’un pli (charge, écriture, explosion, révélation) est ce que le jeu a de plus marquant. Le raccourcir ou le supprimer pour un lot appauvrirait ce que le joueur aime.

## Décision

### Un bouton, à partir de deux plis prêts

- Sur l’atelier d’un fascicule, dès que **deux plis ou plus** sont prêts, un bouton secondaire « Ouvrir les N plis » apparaît à côté de « Ouvrir le pli ». « Ouvrir le pli » reste l’unique action principale, en Corail.
- Le bouton ouvre **tous les plis prêts du fascicule choisi**, jusqu’à la capacité d’énergie. Il est désactivé hors connexion et pendant une ouverture.
- Chaque pli est un pli ordinaire : même tirage, mêmes garanties comptées pli après pli, même encre. Le serveur reste seul maître de chaque tirage. L’application envoie **une commande d’ouverture par pli, l’une après l’autre**, chacune avec sa propre clé d’idempotence et la version d’état renvoyée par la précédente.
- Si une commande échoue en cours de route (plus de charge, état périmé, réseau), la série s’arrête. Les plis déjà ouverts s’affichent, une phrase dit pourquoi les autres ne l’ont pas été, et l’état est resynchronisé. L’échec n’est jamais présenté comme un succès.

### Un rituel en canon

- Les N plis s’affichent en grille, secoués, dès l’envoi. **Chaque pli joue le rituel entier d’un pli seul**, avec ses brouillons barrés et sa plume, et les départs sont décalés de **0,11 s**.
- La brique prend **la place de son pli** : nouvelle brique, « +1 · ×3 » ou « pleine », avec sa forme de rareté.
- Une rare ou une légendaire annonce son tirage dès l’écriture : plume et rayons violets, écriture plus longue de 0,6 s, anneau violet à la révélation, accord pour la légendaire. Une annonce « Rare ! » ou « Légendaire ! » conclut le lot, d’après sa meilleure rareté.
- La plume du rituel groupé va un peu plus vite que celle d’un pli seul et saute le brouillon de translittération pour une brique ordinaire ; à six plis ou plus, un pli ordinaire n’écrit qu’un brouillon.
- À dix plis, le rituel dure environ **4 secondes** (5 avec une rare et une légendaire).
- Toucher la scène, Échap ou Espace passe l’animation, comme pour un pli seul. Si le système demande de réduire les animations, le résultat s’affiche directement.

### Revue de l’animation

Une relecture indépendante de l’animation (11 octobre 2026) a conduit à quelques retouches, sans changer le principe du canon :

- **Retenu** : une rare ou une légendaire garde un contour violet une fois le lot révélé, et les rayons de fin restent violets, pour que le moment fort ne s’efface pas avec l’anneau. Un pli ordinaire de petite taille (six plis ou plus) n’écrit qu’un brouillon, la plume y étant illisible. L’annonce lue aux lecteurs d’écran commence par la meilleure rareté, chaque brique dit sa rareté, et le flash est coupé quand les animations sont réduites. L’écart entre les deux grands plis est élargi.
- **Gardé tel quel** : le décalage fixe de 0,11 s entre deux plis, et l’ordre des départs, qui suit celui des tirages. Allonger le décalage avec le nombre de plis ou jouer la rare en dernier ferait monter l’attente, mais allongerait l’ouverture et ferait passer la mise en scène avant l’ordre réel des tirages.
- **Non traité** : l’équilibrage des rangées (4-4-2, 3-3-3-1) et le coût de rendu de l’écriture lettre à lettre, qui n’a pas été mesuré.

### Résultat

- Chaque brique est un bouton « Essayer sur la table ». « Continuer » revient à l’atelier.
- Une seule phrase résume le lot, annoncée une fois aux lecteurs d’écran : « 5 plis ouverts : 2 nouvelles briques, +14 gouttes d’encre. »
- L’encre, les nouveautés, la garantie et l’historique restent figés pendant le rituel et arrivent d’un bloc à la fin, comme pour un pli seul.

## Options envisagées

### Une route serveur qui ouvre N plis en une transaction

Tout ou rien, une seule clé d’idempotence, un seul aller-retour. Écartée pour l’instant : elle demande un contrat d’API nouveau et ne change rien pour le joueur. À reprendre si les échecs en cours de série se montrent fréquents.

### Rafale, éventail, plis retournés, crescendo, bilan groupé

Cinq autres mises en scène ont été comparées en maquette. La rafale et l’éventail raccourcissent le rituel sans le garder entier ; les plis retournés demandent N gestes ; le crescendo révèle les briques dans un ordre différent du tirage ; le bilan groupé ne montre pas les briques atterrir. Le canon garde le rituel de chaque pli, sans ordre inventé.

## Conséquences

### Positives

- Dix plis s’ouvrent en environ 4 secondes au lieu de plus de 20, sans retirer le rituel.
- Aucun changement de règle de jeu, d’équilibrage ni de contrat d’API.
- Le joueur reconnaît le rituel qu’il connaît.

### Négatives

- À dix plis, chaque pli est petit : la plume est décorative, seul le résultat se lit.
- Une série interrompue laisse un lot ouvert en partie.
- Beaucoup d’éléments animés à l’écran : l’animation se limite aux propriétés `transform` et `opacity`.
- La durée de 1,5 à 2 secondes du [rituel d’ouverture](../game-design.md) ne vaut plus que pour un pli seul.

## Critères de réévaluation

- Plus de 2 % d’ouvertures groupées interrompues en cours de série : passer à une route serveur en une transaction.
- Une ouverture groupée passée dans plus de la moitié des cas : raccourcir encore le canon.
- Un taux de plis ouverts un à un qui baisse sans que les ouvertures groupées augmentent : le bouton n’est pas trouvé.
