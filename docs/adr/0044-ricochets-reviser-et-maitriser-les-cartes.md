# ADR 0044 — Ricochets : réviser les cartes trouvées et les maîtriser

- **Statut** : Proposé
- **Complète** : [ADR 0012](0012-briques-rationnees.md) (une activité qui ne consomme rien) ; [ADR 0007](0007-etat-et-economie-autoritaires.md) (une commande de plus sous autorité serveur) ; [ADR 0045](0045-quetes-du-jour-carte-de-lecteur-et-quetes-au-long-cours.md) (la quête de mémoire du jour)

## Contexte

Entre deux plis, le joueur n’a parfois plus rien à faire : sa réserve ne permet aucune découverte, et l’attente peut durer jusqu’au filet du 5ᵉ pli ([ADR 0012](0012-briques-rationnees.md)). Une fois son fascicule complet, il n’a plus de mot à chercher avant le suivant.

Le pilier Transmission veut que le joueur retienne ce qu’il découvre. L’étude de marché compte les étudiants parmi ses cibles, qui veulent réviser les racines grecques et latines en jouant. Le game design nomme aussi le risque inverse : un jeu « perçu comme un cours ».

Il faut donc une activité qui se joue sans énergie ni exemplaire, sur ce que le joueur a déjà trouvé, courte et ludique.

## Décision

### Ce que sont les ricochets

- Les **ricochets** sont de courtes questions sur les cartes déjà trouvées. Ils ne consomment rien : ni exemplaire, ni énergie, ni encre.
- Ils s’ouvrent depuis le codex et depuis la quête du jour qui les demande ([ADR 0045](0045-quetes-du-jour-carte-de-lecteur-et-quetes-au-long-cours.md)). Ils n’ajoutent pas de destination à la navigation.
- Une **séance** compte 5 questions, environ une minute, en mêlant les formats. Elle choisit d’abord les cartes à revoir, puis les découvertes récentes.
- Le joueur fait autant de séances qu’il le souhaite. Seule une séance par jour compte pour les quêtes.

### Six formats

| Format | Exemple | Données employées |
|---|---|---|
| Sens d’une brique | *géo-* veut-il dire « terre », « vie », « peur » ou « étude » ? | gloses des briques |
| Brique d’un sens | Quelle brique veut dire « peur » ? | gloses des briques |
| Mot d’un sens littéral | Quel mot veut dire « description de la terre » ? | sens littéraux |
| Littéral ou actuel | Pour *géographie*, lequel est le sens littéral : « description de la terre » ou « science qui décrit la surface de la Terre » ? | sens littéral et définition actuelle, seulement s’ils diffèrent nettement |
| Découper | Où *géographie* se coupe-t-il : *géo · graphie*, *géog · raphie*, *gé · ographie* ou *géogra · phie* ? Au verdict, les couleurs du préfixe et du suffixe apparaissent. | segments de la composition |
| Paires | Reliez quatre briques à leurs quatre sens. | gloses des briques |

- Réponses et leurres viennent des cartes trouvées et des briques connues du joueur : jamais d’une carte inconnue, qui serait dévoilée, jamais d’un mot inventé. Avec moins de quatre cartes disponibles, une question propose deux ou trois choix.
- Pas de chronomètre. Un toucher sur un choix répond, sans bouton « Valider ». Une brique s’affiche toujours comme une brique, jamais en texte brut, et sans sa glose quand la glose est la réponse. Une mauvaise réponse s’affiche en Gris encre, comme un échec à la table, jamais en rouge.
- Une mauvaise réponse ne coûte rien : la carte revient simplement plus tôt.
- Le codex n’affiche pas de nombre de cartes à revoir : un tas qui grossit se vit comme un devoir. Le bouton « Faire des ricochets » suffit.

### La maîtrise

- Chaque carte trouvée a un **niveau**, de 0 à 3. Une bonne réponse à une carte **à revoir** la fait monter d’un niveau et éloigne sa prochaine révision : 1 jour, puis 3, puis 7. Une mauvaise réponse la fait descendre d’un niveau, et elle revient le lendemain.
- Au niveau 3, après trois bonnes réponses sur trois jours différents, la carte est **maîtrisée**, et le reste. Elle revient ensuite tous les 7 jours au plus.
- Une réponse à une carte qui n’est pas à revoir ne change aucun niveau : jouer plus ne fait pas monter plus vite.
- La maîtrise se voit sur la carte, par un signe à dessiner, distinct des formes de rareté, des cercles d’état et des trois traits de la confiance éditoriale ([identité](../identite.html#formes)).
- Chaque fascicule a sa **jauge de maîtrise** (« 18 / 24 maîtrisées »). Les mots légendaires n’y comptent pas ([ADR 0043](0043-legendaires-chance-fixe-hors-garanties-et-secretes.md)).

### Autorité serveur

- Niveaux et prochaines révisions sont des données du joueur. Le serveur vérifie chaque réponse, puisque les ricochets font avancer les quêtes ([ADR 0045](0045-quetes-du-jour-carte-de-lecteur-et-quetes-au-long-cours.md)). La commande `ricochet` est idempotente, comme les autres ([ADR 0007](0007-etat-et-economie-autoritaires.md)).
- Le verdict s’affiche dès la réponse : le client le calcule à partir du catalogue, qui est public. Attendre le serveur ne protégerait donc aucune réponse, et ralentirait chaque question. La commande part en même temps, et le serveur fait foi pour le niveau, la maîtrise et les quêtes. Le récapitulatif de fin de séance, lui, affiche les retours confirmés par le serveur.
- Hors connexion, les ricochets ne se jouent pas, comme toute commande ([ADR 0041](0041-hors-connexion-consultation-seule.md)).

### Le nom

« Ricochets » : chaque bonne réponse renvoie la carte plus loin, comme un galet qui rebondit de plus en plus loin. Le nom se décline : « Faites des ricochets », « Cinq ricochets sans faute », « Dans 7 jours, un ricochet juste et la carte *géologie* sera maîtrisée ».

## Options envisagées

### D’autres formats

Écartés : la langue d’origine d’un mot, le type d’une transformation, la silhouette d’une carte, la recomposition à la table, l’ordre des formes d’un mot d’une langue à l’autre, et la chronologie des mots, qui demanderait une donnée éditoriale de plus (la date de première attestation).

### Récompenser chaque bonne réponse en encre

Écartée : une source d’encre illimitée lèverait la seule borne du rythme des sabliers, l’encre gagnée en ouvrant des plis ([ADR 0039](0039-plafonner-l-achat-des-sabliers-pas-leur-usage.md)). Les ricochets récompensent par les quêtes.

### Un chronomètre

Écarté : il presse le joueur et pénalise ceux qui lisent ou répondent plus lentement.

### Attendre le serveur avant le verdict

Écarté : le catalogue public donne déjà toutes les réponses au client, l’attente n’empêcherait aucune triche et coûterait un aller-retour réseau par question.

### Découper en touchant le mot

Écarté : sur mobile, les lettres sont trop étroites pour une cible de 44 px, et un mot en trois parties a plusieurs coupes justes.

### Littéral ou actuel sur un seul sens

Écarté : une question à deux réponses, sur un sens montré seul, se gagne une fois sur deux au hasard, et devient ambiguë quand le sens littéral et la définition actuelle se ressemblent.

### D’autres noms

Écartés : « Révision », « Exercices » et « Quiz », trop scolaires ; « Épreuves », vocabulaire de l’examen ; « Gammes », « Petits papiers » et « Mots en tête », moins parlants ; « Atelier », déjà pris par l’atelier des plis ; « Défis », qui se confondrait avec le défi du jour.

## Conséquences

### Positives

- Le joueur a toujours quelque chose à faire, sans énergie ni exemplaire.
- Ce qu’il découvre s’ancre : le pilier Transmission devient une activité.
- Aucun coût éditorial : tout vient des cartes publiées.
- La maîtrise ajoute une seconde complétude à chaque fascicule, utile une fois ses mots trouvés.

### Négatives

- Le risque d’un jeu perçu comme un cours : les séances doivent rester courtes, facultatives et jamais notées.
- De nouvelles données du joueur (niveau, prochaine révision) et une commande de plus à vérifier.
- Un signe de maîtrise à dessiner, sans confusion possible avec les autres signes des cartes.

## Critères de réévaluation

- Peu de joueurs font une séance hors des quêtes : les formats ne plaisent pas.
- Les tests montrent que les ricochets sont vécus comme un devoir.
- Les cartes deviennent maîtrisées trop vite ou jamais : revoir les intervalles.
