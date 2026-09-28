# ADR 0039 — Sabliers échangés à part, à 10 gouttes et sans plafond, dans une boutique

- **Statut** : Accepté
- **Remplace partiellement** : [ADR 0020](0020-deux-plis-en-attente-et-sabliers.md) et [ADR 0018](0018-sabliers-et-boutique.md), pour le prix du sablier (1 goutte), son achat implicite au moment de l'usage, le plafond de 12 sabliers par 24 h glissantes, sur l'usage comme sur l'échange, le plafond de 36 sabliers détenus et l'annonce du temps perdu avant de confirmer ; [ADR 0038](0038-indices-a-prix-croissant-et-format-des-jalons.md), pour les sabliers de jalon perdus au-delà de 36 détenus. Le reste de ces ADR reste en vigueur

## Contexte

L'ADR 0018 plafonne l'usage : 12 sabliers utilisés par 24 h glissantes, gratuits comme achetés. L'ADR 0020 reprend ce plafond pour les sabliers du MVP.

Depuis l'[ADR 0038](0038-indices-a-prix-croissant-et-format-des-jalons.md), les jalons donnent des sabliers. Avec un plafond sur l'usage, un joueur qui en reçoit plusieurs ne peut pas s'en servir quand il en a besoin : sa récompense attend le lendemain. Le plafond ne doit pas retenir ce que le joueur a déjà gagné. Pour la même raison, le plafond de 36 sabliers détenus fait perdre des récompenses de jalon, ce que l'ADR 0038 notait comme une limite provisoire.

Par ailleurs, la feuille des sabliers proposait le plus grand nombre de sabliers *sans perte* et avertissait du temps perdu au-delà. À 11 h 59 du prochain pli, elle proposait 11 sabliers et expliquait qu'un 12ᵉ gaspillerait 59 minutes : le joueur devait choisir entre attendre une minute et lire un avertissement pour une perte négligeable.

## Décision

### Prix et échange

- Un sablier s'**échange contre 10 gouttes d'encre**, au lieu d'une. L'échange est un geste distinct, qui ajoute des sabliers à ceux détenus.
- **Seuls les sabliers détenus s'utilisent.** Utiliser un sablier ne prélève jamais d'encre.
- À ce prix, les 12 gouttes gagnées en 3 jours de jeu valent un sablier : l'encre sert d'abord aux indices, et les sabliers viennent surtout des jalons.

### Plafonds

| Paramètre | Valeur |
|---|---|
| Sabliers échangés contre de l'encre | sans limite |
| Sabliers détenus utilisés par 24 h | sans limite |
| Sabliers détenus | sans limite |

- **Aucun échange entre ressources du jeu n'est plafonné.** L'encre ne s'obtient qu'en ouvrant des plis : elle borne à elle seule le rythme, et un plafond d'échange ne ferait que retenir des gouttes déjà gagnées.
- Seul l'**achat en euros** sera plafonné, quand la boutique payante de l'ADR 0018 sera acceptée.
- Une commande d'échange porte au plus 1 000 sabliers : c'est une borne de la requête, pas une règle de jeu.
- Un sablier ne s'utilise toujours que si l'énergie est sous 2.
- Les sabliers d'un jalon s'ajoutent tous à ceux détenus : aucun n'est perdu.
- Une commande utilise au plus 24 sabliers, ceux qui remplissent une énergie vide.
- Le prix du sablier est un paramètre d'équilibrage ([ADR 0009](0009-contenu-et-equilibrage.md)).

### Boutique

- La **boutique** est un écran à part, ouvert depuis l'en-tête des plis et la carte de l'encre. Elle a deux sections :
  - **Échanges** : les ressources du jeu entre elles, aujourd'hui l'encre contre des sabliers, en lots de 1, 3 et 12 que la feuille permet d'ajuster ;
  - **Achats** : les lots en euros de l'ADR 0018, affichés « Bientôt ouvert » et verrouillés tant que cet ADR n'est pas accepté.
- **Raccourcis** : le joueur n'a pas à passer par la boutique quand une ressource manque. Sous les plis en attente, tant que l'énergie n'est pas pleine, un bouton propose « Remplir avec des sabliers » s'il en détient, sinon « Échanger contre des gouttes », qui ouvre directement la feuille d'échange.

### Feuille des sabliers

- La feuille propose d'emblée le **nombre qui remplit l'énergie** : le temps restant avant 2 / 2, arrondi à l'heure supérieure, dans la limite des sabliers détenus et achetables.
- Ce nombre est aussi le **maximum** : le joueur ne peut pas dépenser un sablier qui ne servirait à rien.
- Aucun avertissement de temps perdu : seul le dernier sablier peut n'avancer qu'une partie de son heure, et la feuille affiche le temps réellement gagné.

## Options envisagées

### Garder 1 goutte le sablier, acheté au moment de l'usage

Écartée : à ce prix, l'encre achète le temps presque gratuitement, et l'achat implicite mélange deux gestes dans une même feuille.

### Garder le plafond sur l'usage

Écartée : il retient les sabliers gagnés par les jalons, et le joueur ne comprend pas pourquoi une récompense déjà reçue est bloquée.

### Garder l'avertissement de temps perdu

Écartée : l'avertissement porte sur moins d'une heure, et proposer un sablier de moins oblige le joueur à revenir pour quelques minutes.

### Plafonner l'échange à 12 sabliers par 24 h

Écartée : l'encre ne s'achète pas et se gagne lentement (2 gouttes par pli, plus les doublons). Le plafond ne bornait donc que des gouttes déjà gagnées, obligeait à expliquer un second compteur et renvoyait le joueur au lendemain. Le plafond reste nécessaire pour l'achat en euros, où il borne la dépense (ADR 0018).

### Échanger seulement depuis l'écran des plis

Écartée : l'échange se mêlait aux chances et à l'atelier du pli, et ne laissait aucune place pour la boutique en euros à venir.

## Conséquences

### Positives

- Les sabliers gagnés s'utilisent quand le joueur le souhaite, et aucune récompense de jalon n'est perdue.
- Le rythme payé en encre reste borné par l'encre gagnée : 120 gouttes pour un pli de plus.
- Remplir l'énergie se fait en un geste, sans calcul ni avertissement.
- Le sablier a une valeur lisible face aux indices (5 à 80 gouttes).

### Négatives

- Les sabliers gratuits accélèrent davantage le rythme : un joueur qui en a accumulé beaucoup peut remplir son énergie à chaque visite. L'énergie reste plafonnée à 2 plis en attente, ce qui limite le gain par visite.
- Un joueur qui a accumulé de l'encre peut l'échanger d'un coup contre plusieurs recharges : il renonce alors aux indices qu'elle aurait payés.
- Le dernier sablier peut perdre jusqu'à 59 minutes, sans que la feuille le dise.
- La promesse de l'ADR 0020 d'un pli offert tous les 3 jours grâce à l'encre ne tient plus : 12 gouttes font un sablier, soit une heure.

## Critères de réévaluation

- Des jalons qui donnent assez de sabliers pour vider un fascicule en une journée.
- Des joueurs qui accumulent des centaines de sabliers : réintroduire un plafond de détention.
- Des joueurs qui échangent assez d'encre pour ouvrir plus d'un pli de plus par jour : réintroduire un plafond d'échange.
- Des joueurs qui n'échangent jamais d'encre : baisser le prix.
