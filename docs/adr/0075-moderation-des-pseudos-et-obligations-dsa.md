# ADR 0075 — Modération des pseudos et obligations du règlement sur les services numériques (DSA)

- **Statut** : Proposé, à faire valider par le conseil et à accepter avant tout affichage d’un pseudo à un autre joueur
- **Portée** : vue compétitive (après le MVP), CGU, mentions légales, Réglages de la PWA
- **Complète** : [ADR 0070](0070-comptes-facultatifs-par-lien-et-code-e-mail.md) (section « Le pseudo » et critère « règles de modération des pseudos »), [ADR 0010](0010-observabilite-et-vie-privee.md) (aucun contenu libre dans les journaux ni la télémétrie), [ADR 0071](0071-duree-de-conservation-des-comptes-et-des-plis.md) (durées de conservation)

## Contexte

Dès qu’un pseudo est visible d’un autre joueur, le jeu héberge un contenu fourni par ses utilisateurs. Le règlement sur les services numériques (DSA) s’applique alors, même à une très petite structure (écart L18 de l’audit juridique) :

- **points de contact** pour les autorités (art. 11) et pour les utilisateurs (art. 12) ;
- **conditions d’utilisation** qui décrivent la modération, y compris l’examen humain et le traitement des réclamations (art. 14) ;
- **mécanisme de signalement** (art. 16) ;
- **motivation de chaque décision** qui restreint un contenu (art. 17).

Le jeu a deux contraintes propres. Le serveur ne garde jamais l’adresse électronique d’un joueur ([ADR 0070](0070-comptes-facultatifs-par-lien-et-code-e-mail.md)) : il ne peut donc écrire ni au signalant ni au joueur visé. Et le pseudo par défaut vient de la partie locale de l’adresse, qui contient souvent un vrai nom : il reste privé tant que le joueur ne l’a pas confirmé.

Aujourd’hui, aucun écran ne montre un pseudo à un autre joueur. Cet ADR doit donc être accepté avant la vue compétitive, pas avant. Ce relevé n’est pas un avis juridique : le conseil confirme chaque règle.

## Décision

### Hypothèse de départ

- Le jeu est un service d’hébergement. Si la vue compétitive diffuse les pseudos au public, il est aussi une plateforme en ligne, mais **une microentreprise ou une petite entreprise est exemptée des obligations propres aux plateformes** (art. 19 : réclamations, règlement extrajudiciaire, signaleurs de confiance, rapport sur les utilisateurs actifs) et du rapport de transparence (art. 15). Le jeu est établi en France : pas de représentant légal à désigner (art. 13). Le conseil confirme ces trois points.
- Les obligations qui restent s’appliquent telles quelles : points de contact, conditions d’utilisation, signalement, motivation.
- Le jeu applique **volontairement** une réclamation simple (voir plus bas) : elle ne coûte presque rien et répond à l’article 14.

### Quand un pseudo devient visible

- Un pseudo n’est montré à un autre joueur que s’il est **confirmé** par le joueur, ou modifié par lui ([ADR 0070](0070-comptes-facultatifs-par-lien-et-code-e-mail.md)), et qu’il respecte les règles ci-dessous.
- L’écran de confirmation dit où le pseudo sera visible (« Les autres joueurs verront ce pseudo dans… ») et renvoie aux règles. Le joueur peut en choisir un autre à tout moment.
- Le pseudo par défaut n’est jamais montré, quel que soit le moment.

### Règles des pseudos

Le format de l’[ADR 0070](0070-comptes-facultatifs-par-lien-et-code-e-mail.md) reste (3 à 24 caractères, minuscules sans accent, chiffres, traits d’union isolés). Il s’y ajoute :

- **Aucun propos haineux, injurieux, discriminatoire, sexuel, violent ou qui fait l’apologie d’un crime.**
- **Aucun nom de personne identifiable** autre que le sien, et aucune coordonnée : adresse électronique, numéro de téléphone (pas de suite de 6 chiffres ou plus), nom d’utilisateur d’un autre service.
- **Aucune usurpation** : pas de nom qui laisse croire que le pseudo est celui d’ÉtymoLogique, de son équipe ou d’un modérateur. Les noms réservés (par exemple `etymologique`, `admin`, `support`, `moderateur`) sont refusés.
- **Aucune publicité** ni appel à acheter, à rejoindre ou à visiter.

Le jeu tient pour **non retenue** toute orthographe qui contourne ces règles (chiffres à la place de lettres, traits d’union entre les lettres).

### Le filtre à la saisie

- Une liste de termes interdits, normalisée (minuscules, chiffres lus comme des lettres, traits d’union retirés), est versionnée comme un paramètre ([ADR 0009](0009-contenu-et-equilibrage.md)). Elle sert à **refuser** un pseudo au moment du choix ; elle ne retire jamais un pseudo déjà confirmé.
- Le refus dit la règle enfreinte, sans donner la liste : « Ce pseudo n’est pas disponible : il enfreint la règle sur les noms de personnes. Choisissez-en un autre. »
- Le filtre est une aide : il ne remplace pas le signalement, car une liste ne comprend ni le contexte ni les langues.

### Le signalement

- Toute vue qui montre le pseudo d’un autre joueur porte une entrée **« Signaler ce pseudo »**, atteignable en un geste, à 44 px au moins.
- Le formulaire demande : le **motif** (une règle ci-dessus, « contenu illicite » ou « autre »), une **explication** facultative de 500 caractères au plus, et rappelle que le pseudo signalé est joint au signalement. Il n’est pas nécessaire de se connecter : un invité peut signaler.
- Le jeu **ne demande ni nom ni adresse** au signalant. L’article 16 demande de faciliter ces éléments pour qu’un signalement vaille connaissance effective ; le signalant est ici identifié par son profil, et le conseil confirme que cela suffit. Un signalement envoyé par e-mail au point de contact est traité de la même façon.
- Le jeu **accuse réception** à l’écran dès l’envoi, puis **notifie la décision** au signalant à sa prochaine ouverture (carte refermable), puisqu’aucune adresse n’est gardée.
- Un profil ne peut signaler un même pseudo qu’une fois, et les signalements sont limités par profil et par adresse réseau (par exemple 10 par jour), contre les signalements abusifs.
- Un signalement ne masque rien tout seul : sinon, signaler servirait à faire taire un adversaire.

### L’examen et la décision

- Un humain examine chaque signalement : **sous 48 heures** quand le motif est « contenu illicite », **sous 7 jours** sinon. Aucune décision n’est prise par un moyen automatique seul.
- Trois décisions possibles : **ne rien faire** ; **masquer le pseudo** (les autres joueurs voient « Joueur masqué » et un numéro neutre) ; pour un contenu manifestement illicite, **masquer sans attendre** la fin de l’examen.
- Le joueur dont le pseudo est masqué garde son compte et tout son jeu : la sanction ne touche que **l’affichage public**. Il choisit un autre pseudo, soumis aux mêmes règles. Un joueur dont trois pseudos ont été masqués voit son **affichage public suspendu** 30 jours ; jamais le jeu, la progression ni les achats.
- Un contenu qui laisse soupçonner une infraction pénale grave menaçant la vie ou la sécurité d’une personne est transmis aux autorités compétentes (art. 18). Le point de contact en tient la trace.

### La motivation des décisions

Toute décision qui masque un pseudo ou suspend l’affichage public est **motivée** (art. 17), en français clair, par une **carte à la prochaine ouverture** du joueur, puis conservée dans les Réglages. Faute d’adresse, l’e-mail n’est pas un canal. La motivation dit :

- ce qui est décidé et **depuis quand**, et la durée d’une suspension ;
- les **faits** et la **règle** des CGU enfreinte ;
- si la décision est née d’un **signalement** (sans nommer le signalant) ou d’un contrôle du jeu ;
- que la décision a été prise **par un humain**, ou à l’aide du filtre à la saisie ;
- les **recours** : la réclamation par le point de contact, un règlement amiable, ou les juridictions.

### La réclamation

- Un joueur peut contester une décision depuis la carte ou les Réglages (« Contester cette décision »), pendant **6 mois**. Le message part au point de contact, avec la décision jointe.
- Une **autre personne que l’auteur de la décision**, ou le porteur du projet lui-même à défaut, la réexamine **sous 14 jours**, et répond dans les Réglages. Le résultat peut lever la décision.

### Les points de contact

- **Un seul point de contact électronique**, `contact@etymologique.fr`, sert aux autorités (art. 11) et aux joueurs (art. 12). Il est tenu par un humain, jamais par un robot de réponse seul, et répond **en français et en anglais**.
- Il figure dans les **mentions légales**, dans les **CGU**, dans l’entrée **« Nous contacter »** des Réglages et sur la page de la vue compétitive. Les langues acceptées y sont indiquées.
- Accusé de réception **sous 3 jours ouvrés**. Les demandes des autorités sont traitées en priorité.
- La boîte `contact@etymologique.fr` est créée avant l’ouverture ; son contenu est conservé 12 mois au plus.

### Les conditions d’utilisation

Les CGU ([EtymoLogique/App#66](https://github.com/EtymoLogique/App/issues/66)) reprennent, en français, sous une rubrique « Pseudos et modération » :

> Votre pseudo peut être vu des autres joueurs une fois que vous l’avez confirmé. Il ne doit contenir ni propos haineux, injurieux, discriminatoire, sexuel ou violent, ni nom d’une autre personne, ni coordonnées, ni usurpation d’ÉtymoLogique ou de son équipe, ni publicité.
>
> Vous pouvez signaler le pseudo d’un autre joueur depuis tout écran qui l’affiche. Une personne examine chaque signalement, sous 48 heures s’il s’agit d’un contenu illicite et sous 7 jours sinon. Un pseudo qui enfreint les règles est masqué ; trois pseudos masqués suspendent votre affichage public 30 jours. Votre compte, votre progression et vos achats ne sont jamais touchés.
>
> Nous vous expliquons chaque décision, à votre prochaine ouverture du jeu, et vous pouvez la contester pendant 6 mois depuis les Réglages ou en écrivant à contact@etymologique.fr. Les filtres automatiques refusent un pseudo au moment du choix ; ils ne masquent jamais un pseudo déjà confirmé.

### Les données du signalement

- Un signalement, une décision et une réclamation sont des **données personnelles** : ils figurent sur la page des données collectées, avec leur finalité (la modération) et leur durée.
- On les garde **12 mois** après la décision, la durée d’un recours utile, puis on les efface. Une suspension en cours prolonge la conservation jusqu’à sa fin.
- Ils vivent en base, jamais dans les journaux ni dans la télémétrie ([ADR 0010](0010-observabilite-et-vie-privee.md)) : un pseudo est un contenu libre.
- Le signalant n’est **jamais nommé** au joueur visé, sauf si la décision l’exige.

## Options envisagées

### Masquer dès le premier signalement

Écarté : un joueur pourrait faire disparaître le pseudo d’un rival avec quelques signalements. Seul le contenu manifestement illicite est masqué avant l’examen.

### Refuser la modération humaine, tout filtrer par liste

Écarté : une liste ne comprend ni le contexte, ni les langues, ni les contournements, et une décision seulement automatique est plus difficile à motiver et à contester.

### Pseudos tirés au hasard

Écarté, comme dans l’[ADR 0070](0070-comptes-facultatifs-par-lien-et-code-e-mail.md), à la demande du produit. Les règles ci-dessus traitent le risque.

### Demander nom et adresse au signalant

Écarté : cela ferait entrer dans la base des adresses que l’[ADR 0070](0070-comptes-facultatifs-par-lien-et-code-e-mail.md) refuse de garder. L’article 16 demande de faciliter ces éléments, non de les exiger ; à confirmer par le conseil.

### Notifier par e-mail

Écarté : le serveur ne conserve aucune adresse. La carte à la prochaine ouverture sert de canal. Un joueur qui n’ouvre plus le jeu n’est pas informé : c’est la limite assumée.

### Bannir du jeu

Écarté : le jeu ne contient aucun échange libre, donc aucun danger qui justifierait de retirer à un joueur sa progression ou ses achats. La sanction se limite à l’affichage public.

### Ne pas afficher de pseudo

Reporté, non écarté : sans vue compétitive, aucune obligation ne naît. Si les règles pèsent plus que la vue ne rapporte, il suffit de ne pas l’ouvrir.

## Conséquences

### Positives

- Les règles, le signalement, la motivation et les contacts sont écrits avant tout affichage public.
- Une sanction proportionnée : le joueur perd un affichage, jamais son jeu.
- Aucune adresse conservée pour la modération.
- Les CGU disposent d’un texte prêt à reprendre.

### Négatives

- Un travail humain récurrent (48 heures, 7 jours) pour un porteur de projet seul : à revoir si les signalements montent.
- Un joueur qui n’ouvre plus le jeu ne reçoit pas la motivation avant d’y revenir.
- Une boîte de contact à tenir, en français et en anglais.
- Plusieurs écrans de plus pour la vue compétitive : signalement, carte de décision, contestation, « Nous contacter ».

## Critères de réévaluation

- Plus de 20 signalements par semaine, ou un délai d’examen dépassé deux fois de suite : outiller la modération ou réduire l’exposition des pseudos.
- Le jeu cesse d’être une microentreprise ou une petite entreprise (12 mois de suite) : reprendre les obligations des plateformes, dont le rapport de transparence et le règlement extrajudiciaire.
- Le conseil refuse l’identification du signalant par son profil, la motivation par carte ou la suspension de 30 jours : revoir la section concernée.
- Ouverture de messages, de commentaires ou de tout autre contenu libre entre joueurs : nouvel ADR, car le risque change de nature.
