//! Régénère la liste des ADR dans docs/adr.html à partir des fichiers docs/adr/*.md.
//!
//! Usage : cargo run -q --manifest-path outils/Cargo.toml --bin build_adr
//!
//! Le contenu est inséré entre les marqueurs <!-- ADR:START --> et <!-- ADR:END -->.
//! Le convertisseur ne gère que le sous-ensemble Markdown utilisé par les ADR :
//! titres, paragraphes, listes (imbriquées), tableaux, citations, gras, italique, code et liens.

use fancy_regex::{Captures, Regex};
use std::fs;
use std::path::Path;
use std::sync::LazyLock;

/// Renvois vers les pages qui illustrent chaque décision.
const XREFS: &[(&str, &[(&str, &str)])] = &[
    (
        "0001",
        &[
            ("index.html", "Le dossier, pensé mobile d’abord"),
            ("design-system.html#ds-1-7", "Grille et points de rupture"),
            ("interfaces.html", "L’inventaire des interfaces"),
            ("maquettes.html", "Chaque écran sur mobile et ordinateur"),
        ],
    ),
    (
        "0002",
        &[
            ("codex.html#filiation", "La filiation dans le codex"),
            ("codex.html#carte-etymologique", "Une carte et ses formes"),
        ],
    ),
    (
        "0003",
        &[
            ("jeu.html#table", "Des recettes ordonnées sur la table"),
            ("codex.html#carte-geologie", "Une transformation expliquée"),
        ],
    ),
    (
        "0004",
        &[
            ("plis.html#chances", "Garantie et filet d’utilité"),
            ("jeu.html#table", "La réserve de départ"),
        ],
    ),
    ("0005", &[("codex.html", "Les cartes éditoriales du codex")]),
    (
        "0006",
        &[
            ("codex.html#carte-grec", "Une carte de langue"),
            ("typographie.html", "Chaque écriture garde sa forme"),
        ],
    ),
    ("0007", &[("plis.html", "Tirages et énergie de la démo")]),
    (
        "0008",
        &[
            ("plis.html", "L’atelier des plis"),
            ("mouvement.html", "Le rituel d’ouverture"),
        ],
    ),
    (
        "0009",
        &[("plis.html#chances", "Poids et chances affichés")],
    ),
    (
        "0010",
        &[(
            "design-system.html#accessibilite",
            "Accessibilité et respect du joueur",
        )],
    ),
    (
        "0011",
        &[
            ("plis.html#chances", "Chances par rareté et par brique"),
            ("identite.html#formes", "Les formes de rareté"),
        ],
    ),
    (
        "0012",
        &[
            ("jeu.html#table", "Compteurs, « −1 » et briques épuisées"),
            ("plis.html", "Exemplaires, plafond et filet"),
            ("design-system.html#ds-3-2", "Réserve et rationnement"),
        ],
    ),
    (
        "0016",
        &[
            ("plis.html#pkChooser", "Choisir un fascicule"),
            ("plis.html#jaquettes", "Une jaquette par fascicule"),
        ],
    ),
    (
        "0015",
        &[
            ("codex.html#codex", "Complétude par fascicule"),
            ("plis.html#chances", "Légendaires inconnues regroupées"),
        ],
    ),
    (
        "0014",
        &[
            ("modele.html#sources", "Sources et fascicules"),
            ("plis.html#fascicules", "Les fascicules côté joueur"),
        ],
    ),
    (
        "0017",
        &[("modele.html#sources", "Sources et licence du catalogue")],
    ),
    (
        "0018",
        &[("plis.html#sablier", "Le sablier dans l’atelier des plis")],
    ),
    (
        "0019",
        &[("plis.html#encre", "L’encre gagnée à chaque doublon")],
    ),
    (
        "0020",
        &[
            ("plis.html#energie", "Deux plis en attente"),
            ("plis.html#sablier", "Une goutte, une heure"),
        ],
    ),
    (
        "0021",
        &[
            ("logo.html#piste-1", "Le sceau retenu et ses déclinaisons"),
            ("plis.html#plis", "Le logo monochrome sur la face du pli"),
            ("identite.html#logo", "Le logo dans la charte"),
        ],
    ),
    (
        "0024",
        &[
            (
                "interfaces.html#i-04",
                "Hors connexion et resynchronisation",
            ),
            ("interfaces.html#i-25", "Action refusée par le serveur"),
            ("modele.html", "Le modèle éditorial, compilé en artefact"),
        ],
    ),
    (
        "0023",
        &[
            ("codex.html#codex", "Les textures dans le codex"),
            ("identite.html#textures", "La règle des textures"),
            (
                "cartes-visuels.html#declinaison",
                "Les dix pistes et la piste retenue",
            ),
        ],
    ),
    (
        "0022",
        &[
            ("plis.html#fascicules", "Les fascicules côté joueur"),
            ("modele.html#sources", "Le choix d’un fascicule"),
        ],
    ),
    (
        "0025",
        &[
            ("plis.html#jaquettes", "Les jaquettes, servies en statique"),
            ("codex.html#codex", "Les silhouettes des cartes découvertes"),
        ],
    ),
    (
        "0026",
        &[
            (
                "modele.html#sources",
                "Sources et fascicules, éditées dans le dépôt privé",
            ),
            ("modele.html#schema", "Le schéma, public"),
        ],
    ),
    (
        "0027",
        &[
            ("modele.html#stockage", "Où vivent les données"),
            ("modele.html#joueur", "Les données du joueur"),
        ],
    ),
    ("0029", &[("modele.html#stockage", "Où vivent les données")]),
    (
        "0032",
        &[
            ("jeu.html#table", "Deux cases sur la table"),
            (
                "maquettes.html#m-09",
                "La table de fusion, mobile et ordinateur",
            ),
            (
                "maquettes.html#m-12",
                "L’indice «\u{a0}nature des deux briques\u{a0}»",
            ),
        ],
    ),
    (
        "0013",
        &[
            ("modele.html", "Le modèle de données"),
            ("modele.html#decisions", "Quatre cas, quatre règles"),
            ("codex.html#filiation", "Formes et filiation dans le codex"),
        ],
    ),
    (
        "0035",
        &[
            ("plis.html#chances", "Les chances du pli"),
            ("maquettes.html#m-20", "L’atelier du pli"),
        ],
    ),
    (
        "0038",
        &[
            ("interfaces.html#i-12", "Les indices"),
            ("interfaces.html#i-18", "Objectifs et jalons"),
            ("maquettes.html#m-12", "La feuille des indices"),
        ],
    ),
    (
        "0039",
        &[
            ("plis.html#sablier", "Le sablier dans l’atelier des plis"),
            ("interfaces.html#i-22", "Échanges d’encre"),
        ],
    ),
    (
        "0036",
        &[
            ("plis.html#chances", "Les chances du pli"),
            ("interfaces.html#i-20", "L’atelier du pli"),
        ],
    ),
    (
        "0042",
        &[
            ("interfaces.html#i-18", "Objectifs et jalons"),
            ("interfaces.html#i-01", "Navigation principale"),
        ],
    ),
    (
        "0043",
        &[
            ("plis.html#chances", "Les chances du pli"),
            ("codex.html#codex", "Complétude et légendaires trouvées"),
        ],
    ),
    (
        "0044",
        &[
            ("interfaces.html#i-26", "Les ricochets"),
            ("maquettes.html#m-26", "Maquette des ricochets"),
            ("identite.html#formes", "Le signe de maîtrise"),
        ],
    ),
    (
        "0052",
        &[
            ("interfaces.html#i-26", "Les ricochets"),
            ("maquettes.html#m-26", "Maquette de la découpe tactile"),
        ],
    ),
    (
        "0045",
        &[
            ("interfaces.html#i-27", "Quêtes du jour et carte de lecteur"),
            ("interfaces.html#i-28", "Quêtes au long cours"),
        ],
    ),
    (
        "0046",
        &[(
            "interfaces.html#i-29",
            "Cadeau de bienvenue et quêtes initiales",
        )],
    ),
    (
        "0047",
        &[("plis.html#fascicules", "Les fascicules côté joueur")],
    ),
    (
        "0048",
        &[("plis.html#fascicules", "Un fascicule tous les 30 jours")],
    ),
    (
        "0049",
        &[
            ("interfaces.html#i-19", "Choix du fascicule"),
            ("interfaces.html#i-23", "Nouveau fascicule"),
        ],
    ),
    ("0050", &[("interfaces.html#i-30", "Votre cabinet")]),
];

fn re(motif: &str) -> Regex {
    Regex::new(motif).expect("expression régulière valide")
}

static CODE: LazyLock<Regex> = LazyLock::new(|| re(r"`([^`]+)`"));
static GRAS: LazyLock<Regex> = LazyLock::new(|| re(r"\*\*(.+?)\*\*"));
static ITALIQUE: LazyLock<Regex> =
    LazyLock::new(|| re(r"(?<![\w*])\*(?!\s)(.+?)(?<!\s)\*(?![\w*])"));
static LIEN: LazyLock<Regex> = LazyLock::new(|| re(r"\[([^\]]+)\]\(([^)]+)\)"));
static LIEN_ADR: LazyLock<Regex> = LazyLock::new(|| re(r"^(\d{4})-[\w-]+\.md\n?$"));
static NUMERO: LazyLock<Regex> = LazyLock::new(|| re(r"^\d+\. "));
static PUCE: LazyLock<Regex> = LazyLock::new(|| re(r"^(?:[-*]|\d+\.) "));
static ELEMENT: LazyLock<Regex> = LazyLock::new(|| re(r"^\s*(?:[-*]|\d+\.) "));
static DEBUT_BLOC: LazyLock<Regex> = LazyLock::new(|| re(r"^(#|\||>|\s*(?:[-*]|\d+\.) )"));
static TITRE: LazyLock<Regex> = LazyLock::new(|| re(r"^#\s*ADR \d{4}\s*—\s*"));
static STATUT: LazyLock<Regex> = LazyLock::new(|| re(r"(?m)^- \*\*Statut\*\* : ([^\n]+)"));

/// Équivalent de `html.escape` de Python.
fn echapper(texte: &str, guillemets: bool) -> String {
    let texte = texte
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;");
    if guillemets {
        texte.replace('"', "&quot;").replace('\'', "&#x27;")
    } else {
        texte
    }
}

fn correspond(motif: &Regex, texte: &str) -> bool {
    motif.is_match(texte).expect("recherche")
}

fn inline(texte: &str) -> String {
    let texte = echapper(texte, false);
    let texte = CODE.replace_all(&texte, "<code>${1}</code>");
    let texte = GRAS.replace_all(&texte, "<strong>${1}</strong>");
    let texte = ITALIQUE.replace_all(&texte, "<em>${1}</em>");
    LIEN.replace_all(&texte, |c: &Captures| {
        let (libelle, cible) = (&c[1], &c[2]);
        let cible = if let Some(adr) = LIEN_ADR.captures(cible).expect("recherche") {
            format!("#adr-{}", &adr[1])
        } else if cible == "README.md" {
            "#adr-index".to_string()
        } else {
            cible.strip_prefix("../").unwrap_or(cible).to_string()
        };
        format!("<a href=\"{}\">{libelle}</a>", echapper(&cible, true))
    })
    .into_owned()
}

/// Rend une liste Markdown, imbriquée par indentation de deux espaces.
fn render_list(lignes: &[String]) -> String {
    let (mut out, mut pile) = (String::new(), Vec::<&str>::new());
    for brut in lignes {
        let retrait = brut.len() - brut.trim_start_matches(' ').len();
        let nu = brut.trim();
        let ordonnee = correspond(&NUMERO, nu);
        let element = PUCE.replace(nu, "");
        let niveau = retrait / 2;
        while pile.len() > niveau + 1 {
            out += &format!("</li></{}>", pile.pop().unwrap());
        }
        if pile.len() == niveau + 1 {
            out += "</li>";
        }
        while pile.len() < niveau + 1 {
            let balise = if ordonnee { "ol" } else { "ul" };
            pile.push(balise);
            out += &format!("<{balise}>");
        }
        out += &format!("<li>{}", inline(&element));
    }
    while let Some(balise) = pile.pop() {
        out += &format!("</li></{balise}>");
    }
    out
}

fn render_table(lignes: &[&str]) -> String {
    let rangees: Vec<Vec<&str>> = lignes
        .iter()
        .map(|l| {
            l.trim()
                .trim_matches('|')
                .split('|')
                .map(str::trim)
                .collect()
        })
        .collect();
    let cellules = |r: &[&str], balise: &str| {
        r.iter()
            .map(|c| format!("<{balise}>{}</{balise}>", inline(c)))
            .collect::<String>()
    };
    let thead = cellules(&rangees[0], "th");
    let tbody: String = rangees
        .iter()
        .skip(2)
        .map(|r| format!("<tr>{}</tr>", cellules(r, "td")))
        .collect();
    format!(
        "<div class=\"table-scroll\"><table><thead><tr>{thead}</tr></thead><tbody>{tbody}</tbody></table></div>"
    )
}

fn render_body(md: &str) -> String {
    let lignes: Vec<&str> = md.split('\n').collect();
    let (mut out, mut i) = (Vec::new(), 0);
    let est_element = |l: &str| correspond(&ELEMENT, l);
    while i < lignes.len() {
        let ligne = lignes[i];
        if ligne.trim().is_empty() {
            i += 1;
        } else if ligne.starts_with('#') {
            let niveau = ligne.len() - ligne.trim_start_matches('#').len();
            out.push(format!(
                "<h{n}>{}</h{n}>",
                inline(ligne[niveau..].trim()),
                n = niveau + 1
            ));
            i += 1;
        } else if ligne.starts_with('|') {
            let debut = i;
            while i < lignes.len() && lignes[i].starts_with('|') {
                i += 1;
            }
            out.push(render_table(&lignes[debut..i]));
        } else if ligne.starts_with('>') {
            let mut bloc = Vec::new();
            while i < lignes.len() && lignes[i].starts_with('>') {
                bloc.push(lignes[i][1..].trim());
                i += 1;
            }
            out.push(format!(
                "<blockquote><p>{}</p></blockquote>",
                inline(&bloc.join(" "))
            ));
        } else if est_element(ligne) {
            let mut bloc: Vec<String> = Vec::new();
            while i < lignes.len()
                && (est_element(lignes[i])
                    || (lignes[i].starts_with("  ") && !lignes[i].trim().is_empty()))
            {
                if est_element(lignes[i]) {
                    bloc.push(lignes[i].to_string());
                } else {
                    let suite = format!(" {}", lignes[i].trim());
                    bloc.last_mut().expect("élément précédent").push_str(&suite);
                }
                i += 1;
            }
            out.push(render_list(&bloc));
        } else {
            let mut bloc = Vec::new();
            while i < lignes.len()
                && !lignes[i].trim().is_empty()
                && !correspond(&DEBUT_BLOC, lignes[i])
            {
                bloc.push(lignes[i].trim());
                i += 1;
            }
            out.push(format!("<p>{}</p>", inline(&bloc.join(" "))));
        }
    }
    out.join("\n")
}

fn main() {
    let docs = Path::new(env!("CARGO_MANIFEST_DIR")).join("../docs");
    let fichier_adr = re(r"^[0-9]{4}-.*\.md$");
    let mut chemins: Vec<_> = fs::read_dir(docs.join("adr"))
        .expect("dossier docs/adr")
        .map(|e| e.expect("entrée de docs/adr").path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| correspond(&fichier_adr, n))
        })
        .collect();
    chemins.sort();

    let (mut articles, mut sommaire) = (Vec::new(), String::new());
    for chemin in &chemins {
        let nom = chemin.file_name().unwrap().to_str().unwrap();
        let md = fs::read_to_string(chemin).expect("lecture de l’ADR");
        let num = &nom[..4];
        let (ligne_titre, reste) = md.split_once('\n').expect("ADR sans corps");
        let titre = TITRE.replace(ligne_titre, "");
        let titre = titre.trim();
        let statut = STATUT
            .captures(reste)
            .expect("recherche")
            .map(|c| c[1].to_string())
            .unwrap_or_default();
        let court = statut
            .split(',')
            .next()
            .unwrap()
            .split('.')
            .next()
            .unwrap()
            .trim();
        let partiel = statut.contains("remplacé") || statut.contains("complété");
        let liens: String = XREFS
            .iter()
            .find(|(n, _)| *n == num)
            .map(|(_, renvois)| {
                renvois
                    .iter()
                    .map(|(h, t)| {
                        format!("<a class=\"xref\" href=\"{h}\">{}</a>", echapper(t, true))
                    })
                    .collect()
            })
            .unwrap_or_default();
        let renvois = if liens.is_empty() {
            String::new()
        } else {
            format!("<p class=\"xrefs\"><span>En situation</span>{liens}</p>")
        };
        articles.push(format!(
            "<article class=\"adr\" id=\"adr-{num}\"><div class=\"adr-head\"><span class=\"adr-num\">ADR {num}</span>\
             <span class=\"adr-status{}\">{}{}</span>\
             <a class=\"xref\" href=\"adr/{nom}\">Source Markdown</a></div>\
             <h2>{}</h2><div class=\"adr-body\">{}</div>{renvois}</article>",
            if partiel { " is-partial" } else { "" },
            echapper(court, true),
            if partiel { " · modifié depuis" } else { "" },
            inline(titre),
            render_body(reste),
        ));
        sommaire += &format!(
            "<a href=\"#adr-{num}\"><span>{num}</span>{}</a>",
            inline(titre)
        );
    }
    let genere = format!(
        "<div class=\"adr-layout\"><nav class=\"adr-toc\" id=\"adr-index\" aria-label=\"Liste des ADR\">{sommaire}</nav>\
         <div class=\"adr-list\">{}</div></div>",
        articles.concat()
    );
    let page_chemin = docs.join("adr.html");
    let page = fs::read_to_string(&page_chemin).expect("lecture de docs/adr.html");
    let (debut, fin) = ("<!-- ADR:START -->", "<!-- ADR:END -->");
    let a = page.find(debut).expect("marqueur ADR:START") + debut.len();
    let b = page.find(fin).expect("marqueur ADR:END");
    fs::write(
        &page_chemin,
        format!("{}\n{genere}\n{}", &page[..a], &page[b..]),
    )
    .expect("écriture de docs/adr.html");
    println!("{} ADR écrits dans docs/adr.html", articles.len());
}
