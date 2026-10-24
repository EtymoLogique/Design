# ADR 0081 — Mémoire des essais : les briques déjà essayées passent en fin de réserve

- **Statut** : Proposé
- **Remplace partiellement** : [ADR 0027](0027-catalogue-statique-et-donnees-joueur.md), pour la phrase « les propositions de fusion ne sont jamais stockées » : un échec ou un « presque » se note désormais, sans issue ni date, jusqu’à la parution suivante. Le reste de l’ADR 0027 reste en vigueur.
- **Complète** : [ADR 0010](0010-observabilite-et-vie-privee.md) (ce qui est stocké), [ADR 0033](0033-gestes-de-la-table-de-fusion.md) (réserve et table)

## Contexte

Se tromper ne coûte rien, mais rien ne garde la trace des essais déjà faits. Rosette compte 96 paires possibles pour 47 mots, Routes des mots 450 paires pour 96 mots. Passé une vingtaine de briques, le joueur oublie ses échecs et refait les mêmes essais en boucle. Le besoin a été retenu par le propriétaire sur la page de travail [pistes de rétention](../pistes-retention.html#memoire-titre), avec l’oubli à chaque parution.

Or l’ADR 0027 et le [modèle de données](../modele-donnees.md#7-données-du-joueur) disent que les propositions de fusion ne sont jamais stockées. Une mémoire tenue par le seul appareil ne suivrait ni un compte sur un autre appareil ([ADR 0070](0070-comptes-facultatifs-par-lien-et-code-e-mail.md)) ni une réinstallation.

## Décision

### Ce que voit le joueur

- Dès qu’une brique est posée sur la table et qu’une case reste libre, la réserve range en fin de liste les briques déjà essayées **avec elle, dans cet ordre de cases**, sous l’intertitre « Déjà essayées avec géo- · 3 ».
- Leur ligne de sens dit pourquoi : « aucune recette », « presque : inversez » ou « au codex ».
- Le gris : fond Sable, libellé en Encre secondaire, sans ombre. Une brique grisée **reste jouable** : ce n’est ni l’état désactivé ni l’état épuisé.
- Une paire jamais essayée ne bouge pas : les autres briques gardent l’ordre habituel, sans tri ni compteur. Retirer la brique ou en poser une autre recalcule la liste ; les briques glissent vers leur place en 240 ms, sans animation en mouvement réduit.

### Ce que garde le serveur

- Une table `essai` : le joueur, la brique de gauche, la brique de droite, et le dernier fascicule paru au moment de l’essai. **Ni issue, ni date, ni texte du catalogue.**
- Le serveur l’écrit dans la transaction de la commande `fusion`, pour un échec ou un « presque » seulement, et seulement si le joueur connaît les deux briques. Sans ligne `commande` ni changement de `version_etat` : rejoué, l’essai ne fait rien.
- **3 000 lignes au plus** par joueur ; au-delà, un essai n’est plus noté.
- Un mot déjà au codex n’a besoin d’aucune ligne : le client le déduit de la règle de fusion.
- L’état du joueur porte la liste des paires. La trace suit le compte sur tous ses appareils, entre dans l’export et part avec la partie.

### L’oubli à chaque parution

À la parution d’un fascicule, **toutes** les lignes jugées sous un fascicule plus ancien s’oublient en bloc : elles ne se lisent plus, et la prochaine écriture les efface. De vieilles paires peuvent devenir des mots : 31 recettes de Routes des mots réunissent deux briques déjà parues. Une purge paire par paire désignerait les familles que prolonge le nouveau fascicule.

## Options envisagées

### Une mémoire locale à l’appareil

Aucune donnée de plus côté serveur. Écartée : elle se perd à la réinstallation et ne suit pas un compte sur un autre appareil.

### Garder l’issue et la date de chaque essai

Elles serviraient la télémétrie. Écartée : rien dans l’affichage ne les demande, et l’ADR 0010 limite ce qui est gardé à ce qui sert le joueur.

### La réserve en tableau préfixes × suffixes, ou une annonce « déjà tentée »

Première version de la piste. Écartée par le propriétaire : un tableau devient illisible passé une vingtaine de briques, et une annonce après coup n’évite pas l’essai.

### Oublier paire par paire les seules paires devenues des mots

Moins d’oubli. Écartée : l’oubli ciblé désignerait les familles du nouveau fascicule.

## Conséquences

### Positives

- Le joueur ne refait plus les mêmes essais, sans que rien ne lui révèle une paire qu’il n’a pas essayée.
- La trace est tenue par le serveur, comme le reste de la partie.

### Négatives

- Une table de plus, jusqu’à 3 000 lignes par joueur.
- À chaque parution, les essais déjà faits redeviennent neutres.

## Critères de réévaluation

- La mesure « tentatives avant une découverte » ne baisse pas : revoir la présentation.
- Des joueurs atteignent le plafond : le relever ou oublier les plus anciennes.
