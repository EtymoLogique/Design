# ADR 0083 — Vue Réseau du codex : la constellation en deux rives

- **Statut** : Proposé
- **Portée** : codex de la PWA (I-14), nouvelle vue « Réseau »
- **Complète** : [ADR 0015](0015-codex-fascicules-et-legendaires.md) (codex et légendaires), [ADR 0034](0034-ecran-dans-l-adresse.md) (vue dans l’adresse)

## Contexte

Le codex montre des cartes isolées : le joueur ne voit pas que *géo-* a donné trois mots, ni que *-logie* en relie cinq. La vue Familles liste les mots d’une brique, pas les liens entre familles. Le propriétaire a retenu une vue en réseau, à condition qu’elle reste lisible, sans chevauchement, et qu’elle ne bouge pas à chaque découverte ([pistes de rétention](../pistes-retention.html#constellation-titre)).

Deux prototypes ont été mesurés sur les fascicules 1 à 6. Un graphe à forces déplaçait 27 briques sur 30 à chaque découverte (48 px en médiane) et devenait un écheveau au-delà de 5 fascicules sur ordinateur, 3 sur mobile. Une disposition en deux rives n’avait aucun chevauchement et ne déplaçait une brique que pour faire place à une nouvelle.

## Décision

### Deux rives

- À gauche, les briques trouvées de la 1ʳᵉ case des recettes ; à droite, celles de la 2ᵉ. Chaque mot trouvé est une courbe qui traverse la « rivière » entre ses deux briques.
- Une **bande par fascicule débloqué**, dans l’ordre de parution ; dans chaque bande, les briques par **ordre alphabétique**, une rangée de **44 px** chacune. La rangée entière est la cible tactile.
- Les courbes partent et arrivent à l’horizontale : elles ne passent jamais sous une brique ni sous un texte.
- La dernière découverte est tracée en Corail ; sous chaque brique, une barre Rosée donne la progression de sa famille.
- La largeur des rives dépend de toutes les briques du réseau, jamais de la seule vue : une brique nouvelle ne déplace rien en largeur, et seulement en hauteur les briques qui la suivent dans l’alphabet.

### Aides

- La vue s’ouvre sur le dernier fascicule débloqué qui a au moins un mot trouvé ; un sélecteur passe d’un fascicule à l’autre ou à « Tous ».
- Dans « Tous », une courbe de plus de 640 px de haut devient deux **antennes** chiffrées (« ↓3 »), sauf pour la famille suivie et la dernière découverte.
- Toucher une brique suit sa famille dans un panneau collé en bas de l’écran : ses mots trouvés, qui ouvrent leur carte, et leurs briques partenaires, qui font suivre leur propre famille.
- Une **liste** en repli dit la même chose en texte ; elle sert aussi aux lecteurs d’écran.
- Une brique qui s’insère apparaît, celles qui changent de rangée glissent en 240 ms ; rien en mouvement réduit.

### Garde-fous

- Seules les briques et les mots trouvés apparaissent : jamais une case pour une brique à trouver ([ADR 0051](0051-indices-a-la-carte.md)). L’ordre alphabétique ne dit rien des recettes.
- Jamais une légendaire ([ADR 0043](0043-legendaires-chance-fixe-hors-garanties-et-secretes.md)), jamais un fascicule verrouillé ; les totaux des familles ne comptent que les fascicules débloqués ([ADR 0069](0069-deblocage-progressif-des-fascicules.md)). Une légendaire trouvée recevra plus tard une bande à part : hors périmètre.
- La disposition se calcule dans la PWA, sur la projection du codex ; aucune donnée ni règle nouvelle côté serveur. La vue entre dans l’adresse (`vue=reseau`, [ADR 0034](0034-ecran-dans-l-adresse.md)).

## Options envisagées

### Un graphe à forces

Plus organique. Écartée : il déplaçait 27 briques sur 30 à chaque découverte et devenait illisible au-delà de quelques fascicules.

### Une matrice préfixes × suffixes

Aucune courbe. Écartée : une case par paire possible montrerait la place des mots à trouver, et la matrice devient immense.

### Un arbre par brique

Lisible pour une famille. Écartée : il ne montre pas les liens entre familles, ce que la vue Familles fait déjà.

## Conséquences

### Positives

- Le joueur voit son vocabulaire comme un réseau qui grandit, sans qu’une découverte ne bouscule ce qu’il connaît.
- Une fonction pure et déterministe : même codex, même dessin, facile à tester.

### Négatives

- Une vue de plus à maintenir, en SVG, avec sa liste en repli.
- Sur mobile, la rivière est étroite : les longues briques réduisent la place des courbes.

## Critères de réévaluation

- La vue est peu ouverte : la retirer ou la fondre dans Familles.
- Au-delà de dix fascicules, la vue « Tous » devient trop haute : n’afficher que les fascicules choisis.
