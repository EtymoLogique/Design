# ADR 0068 — Déblocage progressif des fascicules

- **Statut** : Proposé
- **Portée** : données du joueur, serveur de jeu, PWA ; la publication du catalogue ne change pas
- **Remplace partiellement** : [ADR 0030](0030-reserve-de-depart-par-fascicule.md), pour le moment où les graines sont accordées : au déblocage du fascicule pour ce joueur, et non plus à sa parution ; [ADR 0048](0048-beta-ouverte-et-ouverture-a-quatre-fascicules.md), pour l’estimation de ce qu’un nouveau joueur trouve le premier mois, qui supposait tout le catalogue ouvert
- **Complète** : [ADR 0038](0038-indices-a-prix-croissant-et-format-des-jalons.md) et [ADR 0042](0042-page-quetes-et-recompenses-a-recuperer.md) (le deuxième jalon d’un fascicule ouvre le suivant ; la récupération de la récompense reste à part) ; [ADR 0016](0016-plis-et-jaquettes-par-fascicule.md) (un pli ne s’ouvre que pour un fascicule débloqué) ; [ADR 0045](0045-quetes-du-jour-carte-de-lecteur-et-quetes-au-long-cours.md) et [ADR 0046](0046-demarrage-genereux-cadeau-de-bienvenue-et-quetes-initiales.md) (quêtes du jour, pli de lecteur et pli offert limités aux fascicules débloqués) ; [ADR 0015](0015-codex-fascicules-et-legendaires.md) (codex d’un fascicule verrouillé) ; [ADR 0054](0054-fascicules-de-lancement-plus-gros-et-plus-rapproches.md) (les quatre premiers fascicules paraissent ensemble, un joueur les ouvre un par un) ; [ADR 0049](0049-date-du-prochain-fascicule-publique.md) (la date du prochain fascicule reste affichée)

## Contexte

Un fascicule paru est aujourd’hui ouvert à tous, tout de suite : ses graines arrivent à la première synchronisation ([ADR 0030](0030-reserve-de-depart-par-fascicule.md)), son pli s’ouvre, ses silhouettes remplissent le codex et ses jalons la page Quêtes.

Les fascicules de lancement changent l’échelle ([ADR 0054](0054-fascicules-de-lancement-plus-gros-et-plus-rapproches.md)) : les quatre premiers paraissent ensemble, de 50 à 106 mots chacun, puis un par semaine. Un nouveau joueur reçoit d’un coup des centaines de silhouettes, quatre plis à choisir et douze jalons. La charge mentale est trop forte pour un premier contact, et le joueur ne sait pas par où commencer.

Chaque fascicule déclare déjà trois jalons, à environ un quart, la moitié et la totalité de ses mots ([ADR 0038](0038-indices-a-prix-croissant-et-format-des-jalons.md)). Ils rythment la progression dans un fascicule ; ils peuvent aussi rythmer le passage au suivant.

## Décision

### La règle

- Un nouveau profil n’a qu’un fascicule débloqué : le fascicule paru de **plus petit numéro**.
- Atteindre le **deuxième jalon** d’un fascicule débloqué débloque, pour ce joueur seulement, le **fascicule paru suivant**, celui de numéro immédiatement supérieur parmi les parus.
- Le déblocage a lieu quand le jalon est **atteint**, dans la transaction de la découverte qui l’atteint, ou à la lecture de l’état. La récompense du jalon se récupère toujours à part ([ADR 0042](0042-page-quetes-et-recompenses-a-recuperer.md)).
- Si le fascicule suivant n’est **pas encore paru**, le droit est acquis : il se débloque pour ce joueur dès sa parution, à sa première synchronisation.
- Un fascicule débloqué ne se **reverrouille jamais**, même si un fascicule de numéro intermédiaire paraît après coup.
- Les profils créés avant cette décision gardent débloqués tous les fascicules dont ils ont déjà reçu les graines.
- La publication ne change pas : l’index, les manifestes, la tâche de parution et la date du prochain fascicule ([ADR 0029](0029-parution-par-promotion-de-prefixe.md), [ADR 0049](0049-date-du-prochain-fascicule-publique.md)) restent les mêmes pour tous. Le déblocage est une **donnée du joueur**, sous autorité du serveur ([ADR 0007](0007-etat-et-economie-autoritaires.md)), qui ne contient que des identifiants opaques et une date.
- Chaque fascicule déclare donc au moins deux jalons ; le validateur du catalogue l’exige.

### Ce qui attend le déblocage

| Élément | Fascicule paru et verrouillé |
|---|---|
| Graines | Accordées dans la transaction qui débloque le fascicule, plus à sa parution ([ADR 0030](0030-reserve-de-depart-par-fascicule.md)). |
| Pli du fascicule, pli offert, pli de lecteur | Ne s’ouvrent pas ; le serveur refuse la commande. |
| Jalons | Ne sont pas marqués. Les mots déjà trouvés comptent dès le déblocage, qui peut atteindre aussitôt un ou plusieurs jalons, et en débloquer un autre. |
| Quêtes du jour | La quête de table ne vise que des mots des fascicules débloqués. |
| Codex | La jaquette porte un cadenas. On y voit les mots déjà trouvés, aucune silhouette à trouver ; le compteur dit « 2 trouvées », sans total. |

Ne changent pas : le cadeau de bienvenue ([ADR 0046](0046-demarrage-genereux-cadeau-de-bienvenue-et-quetes-initiales.md)), les quêtes au long cours, qui comptent toutes les découvertes, et les ricochets, qui portent déjà sur les seules cartes trouvées.

### Un mot croisé reste découvrable

Des briques partagées peuvent former un mot d’un fascicule encore verrouillé. La fusion l’accepte : le mot entre au codex et compte pour les jalons de son fascicule dès le déblocage. Le serveur ne fait jamais passer un vrai mot pour un échec.

### Montrer et annoncer

- Un fascicule paru et verrouillé reste **visible** dans le choix du pli, le codex et les quêtes : jaquette atténuée, cadenas, et sa condition, « Atteignez le 2ᵉ jalon de *Nom* ». La couleur n’est jamais le seul signal.
- Les fascicules débloqués passent devant les verrouillés, dans le choix du pli comme dans le sélecteur du codex et des quêtes. La liste des jalons de tous les fascicules ne montre que les débloqués ; un fascicule verrouillé ne montre sa condition que filtré.
- Sur le fascicule précédent, le deuxième jalon annonce « Ouvre le fascicule suivant ».
- La découverte qui débloque un fascicule le dit sur la table : « Nouveau fascicule ouvert : *Nom* », avec un lien vers son pli.
- Une **pastille Corail** « nouveau » marque la jaquette du fascicule débloqué jusqu’à sa première ouverture : le Corail garde son sens, quelque chose de nouveau attend le joueur.

### Premier mois d’un nouveau joueur

L’estimation de l’[ADR 0048](0048-beta-ouverte-et-ouverture-a-quatre-fascicules.md), 55 à 60 mots le premier mois, supposait tout le catalogue ouvert. Elle ne tient plus : le nouveau joueur tire d’abord dans un seul pli et ouvre le deuxième fascicule à 25 mots sur 50. Le critère de taille critique (ne pas épuiser le catalogue en un mois) est d’autant plus facile à tenir ; seul le garde-fou de l’[ADR 0012](0012-briques-rationnees.md), jamais plus de deux jours sans découverte possible, reste à surveiller par la mesure de la rétention ([ADR 0050](0050-cohortes-de-retention-et-statistiques-du-joueur.md)).

## Options envisagées

### Tout ouvrir à la parution

Écartée : c’est la situation actuelle, trop lourde avec quatre fascicules de lancement ouverts ensemble.

### Débloquer au dernier jalon

Écartée : finir un fascicule de 50 à 106 mots demande des semaines ; le joueur resterait trop longtemps enfermé, et les mots difficiles d’un fascicule bloqueraient tout le reste.

### Débloquer à la récupération de la récompense

Écartée : un joueur qui ne va jamais dans les quêtes resterait bloqué sans comprendre pourquoi. Atteindre le jalon suffit ; la récupération reste un geste de récompense.

### Cacher les fascicules verrouillés

Écartée : le joueur ignorerait qu’une suite existe. Une jaquette verrouillée avec sa condition donne un but, sans charge mentale.

### Refuser un mot croisé d’un fascicule verrouillé

Écartée : un refus présenté comme un échec mentirait sur un vrai mot, et un refus explicite confirmerait qu’un mot existe sans rien apporter.

### Un champ du catalogue pour désigner le jalon qui débloque

Écartée pour l’instant : les onze fascicules ont trois jalons, et le deuxième convient partout. Un champ demanderait schéma, validateur et contrat pour aucun cas réel.

## Conséquences

### Positives

- Un nouveau joueur commence par un seul fascicule : un pli, une poignée de graines, trois jalons.
- Chaque fascicule ouvert est une récompense et une surprise, annoncées sur la table.
- La publication, l’index et l’infrastructure ne changent pas.
- Les joueurs existants ne perdent rien.

### Négatives

- Une table de plus dans les données du joueur, et un calcul de déblocage à chaque synchronisation et à chaque découverte.
- Un joueur assidu peut vouloir explorer un fascicule paru qu’il n’a pas encore ouvert : il doit d’abord avancer dans le précédent.
- Le cadeau de bienvenue peut contenir des briques d’un fascicule verrouillé ; elles n’ouvrent que des mots croisés.
- Les maquettes du choix du pli, du codex et des quêtes ne montrent pas encore l’état verrouillé.

## Critères de réévaluation

- Des joueurs actifs restés plus de deux jours sans découverte possible dans leur premier fascicule : abaisser le seuil, ou ouvrir deux fascicules au départ.
- Des joueurs qui abandonnent devant un fascicule verrouillé : rapprocher le jalon qui débloque.
- Un fascicule dont le deuxième jalon ne convient pas : ajouter au catalogue un champ qui désigne le jalon qui débloque.
