//! Vérifie mécaniquement la conformité des styles à l'identité visuelle d'ÉtymoLogique.
//!
//! Usage :
//!   cargo run -q --manifest-path outils/Cargo.toml --bin check_identity -- [chemins...] [--base REF] [--strict]
//!
//! Sans chemin, analyse docs/. Avec --base REF, ne signale que les lignes ajoutées
//! ou modifiées depuis REF (git diff). --strict fait échouer aussi sur les avertissements.
//!
//! La palette de référence est lue dans les propriétés personnalisées de docs/assets/etymo.css.
//! Une ligne contenant le marqueur « identite: ok » est ignorée (exception assumée, à justifier),
//! ainsi que les contre-exemples volontaires (class="rule-demo bad").
//!
//! Règles vérifiées :
//!   hors-palette     (erreur) couleur absente des jetons d'etymo.css
//!   blanc-pur        (erreur) blanc pur : le fond est Papier, les surfaces Crème
//!   degrade-mixte    (erreur) dégradé mêlant deux couleurs de ressource
//!   texte-sur-vif    (erreur) texte clair sur une couleur vive (le texte y est toujours en Encre)
//!   bande-laterale   (erreur) bordure de 3 px ou plus sur un seul côté
//!   encre-variable   (avert.) var(--ink) sur couleur vive : s'inverse en nocturne, préférer var(--fixed-ink)
//!   ombre-coloree    (avert.) ombre ou halo d'une couleur de ressource ou d'action

use fancy_regex::Regex;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::sync::LazyLock;
use std::{env, fs};

type Rgb = (u32, u32, u32);

const EXTENSIONS: [&str; 3] = ["css", "html", "js"];
// « rule-demo bad » : contre-exemples volontaires de la page identité.
const SKIP_MARKERS: [&str; 2] = ["identite: ok", "class=\"rule-demo bad\""];
const VIVID_VARS: [&str; 8] = [
    "--c-prefix",
    "--c-suffix",
    "--c-lang",
    "--c-family",
    "--c-pli",
    "--c-action",
    "--c-action-hover",
    "--c-action-active",
];
const LIGHT_VARS: [&str; 3] = ["--cream", "--paper", "--paper-2"];
const VIVID_HEX: [&str; 8] = [
    "cdf25a", "4fd1c5", "8f7bf0", "f59ac1", "f4b740", "ff6b4a", "ff8667", "e24e2d",
];
const LIGHT_HEX: [&str; 4] = ["fffbf3", "f3eee3", "e8dfcc", "ffffff"];

fn re(motif: &str) -> Regex {
    Regex::new(motif).expect("expression régulière valide")
}

static HEX_RE: LazyLock<Regex> =
    LazyLock::new(|| re(r"(?<![\w&])#([0-9a-fA-F]{8}|[0-9a-fA-F]{6}|[0-9a-fA-F]{3,4})\b"));
static RGB_RE: LazyLock<Regex> = LazyLock::new(|| {
    re(r"rgba?\(\s*(\d{1,3})\s*,?\s*(\d{1,3})\s*,?\s*(\d{1,3})\s*(?:[,/]\s*([\d.]+%?))?\s*\)")
});
static WHITE_RE: LazyLock<Regex> = LazyLock::new(|| re(r"(?<![-\w])white(?![-\w])"));
static VAR_RE: LazyLock<Regex> = LazyLock::new(|| re(r"var\(\s*(--[\w-]+)"));
static DECL_RE: LazyLock<Regex> = LazyLock::new(|| re(r#"(?<![\w-])([a-z-]+)\s*:\s*([^;{}"]+)"#));
static BLOCK_RE: LazyLock<Regex> = LazyLock::new(|| re(r"\{([^{}]*)\}"));
static FAMILLE_RE: LazyLock<Regex> =
    LazyLock::new(|| re(r"^--c-(prefix|suffix|lang|family|pli|action)"));
static JETON_RE: LazyLock<Regex> = LazyLock::new(|| re(r"(--[\w-]+)\s*:\s*#([0-9a-fA-F]{3,8})\b"));
static EPAISSEUR_RE: LazyLock<Regex> = LazyLock::new(|| re(r"(\d+(?:\.\d+)?)px"));
static BORD_RE: LazyLock<Regex> =
    LazyLock::new(|| re(r"^border-(left|right|inline-start|inline-end)(-width)?$"));
static PARENTHESES_RE: LazyLock<Regex> = LazyLock::new(|| re(r"\([^)]*\)"));
static LONGUEUR_RE: LazyLock<Regex> =
    LazyLock::new(|| re(r"(?<![\w.#(-])(-?[\d.]+)(?:px)?(?![\w%])"));
static INK_RE: LazyLock<Regex> = LazyLock::new(|| re(r"var\(\s*--ink\s*\)"));
static HUNK_RE: LazyLock<Regex> = LazyLock::new(|| re(r"\+(\d+)(?:,(\d+))?"));

/// Toutes les captures d'un motif : (début de la correspondance, groupes).
fn captures<'t>(motif: &Regex, texte: &'t str) -> Vec<(usize, Vec<&'t str>)> {
    motif
        .captures_iter(texte)
        .map(|c| {
            let c = c.expect("recherche");
            let groupes = (1..c.len())
                .map(|i| c.get(i).map_or("", |g| g.as_str()))
                .collect();
            (c.get(0).unwrap().start(), groupes)
        })
        .collect()
}

fn hex_to_rgb(valeur: &str) -> Rgb {
    let valeur: String = if matches!(valeur.len(), 3 | 4) {
        valeur.chars().take(3).flat_map(|c| [c, c]).collect()
    } else {
        valeur.to_string()
    };
    let canal = |i: usize| {
        u32::from_str_radix(&valeur[i..(i + 2).min(valeur.len())], 16).expect("hexadécimal")
    };
    (canal(0), canal(2), canal(4))
}

fn famille(nom: &str) -> Option<String> {
    FAMILLE_RE
        .captures(nom)
        .expect("recherche")
        .map(|c| c[1].to_string())
}

struct Palette {
    autorisees: HashSet<Rgb>,
    familles: HashMap<Rgb, String>,
}

/// Retourne l'ensemble des couleurs autorisées et la famille de chaque couleur de ressource.
fn load_palette(racine: &Path) -> Palette {
    let css = fs::read_to_string(racine.join("docs/assets/etymo.css"))
        .expect("lecture de docs/assets/etymo.css");
    let mut palette = Palette {
        autorisees: HashSet::from([(0, 0, 0)]),
        familles: HashMap::new(),
    };
    for (_, g) in captures(&JETON_RE, &css) {
        let rgb = hex_to_rgb(g[1]);
        palette.autorisees.insert(rgb);
        if let Some(f) = famille(g[0]) {
            palette.familles.insert(rgb, f);
        }
    }
    palette
}

fn colors_in(texte: &str) -> Vec<(usize, String, Rgb)> {
    let mut couleurs = Vec::new();
    for m in HEX_RE.captures_iter(texte) {
        let m = m.expect("recherche");
        let tout = m.get(0).unwrap();
        couleurs.push((tout.start(), tout.as_str().to_string(), hex_to_rgb(&m[1])));
    }
    for m in RGB_RE.captures_iter(texte) {
        let m = m.expect("recherche");
        let canal = |i: usize| m[i].parse::<u32>().expect("entier");
        let rgb = (canal(1), canal(2), canal(3));
        if rgb.0 <= 255 && rgb.1 <= 255 && rgb.2 <= 255 {
            let tout = m.get(0).unwrap();
            couleurs.push((tout.start(), tout.as_str().to_string(), rgb));
        }
    }
    for m in WHITE_RE.find_iter(texte) {
        couleurs.push((
            m.expect("recherche").start(),
            "white".to_string(),
            (255, 255, 255),
        ));
    }
    couleurs
}

fn vars_in(valeur: &str) -> Vec<&str> {
    captures(&VAR_RE, valeur)
        .into_iter()
        .map(|(_, g)| g[0])
        .collect()
}

fn families_in(valeur: &str, palette: &Palette) -> BTreeSet<String> {
    let mut trouvees: BTreeSet<String> = vars_in(valeur).into_iter().filter_map(famille).collect();
    trouvees.extend(
        colors_in(valeur)
            .into_iter()
            .filter_map(|(_, _, rgb)| palette.familles.get(&rgb).cloned()),
    );
    trouvees
}

fn contient(valeur: &str, variables: &[&str], hexas: &[&str]) -> bool {
    vars_in(valeur).iter().any(|v| variables.contains(v))
        || colors_in(valeur)
            .iter()
            .any(|(_, _, rgb)| hexas.iter().any(|h| hex_to_rgb(h) == *rgb))
}

/// Découpe une valeur d'ombre en couches, en ignorant les virgules entre parenthèses.
fn shadow_layers(valeur: &str) -> Vec<String> {
    let (mut couches, mut profondeur, mut courante) = (Vec::new(), 0, String::new());
    for c in valeur.chars() {
        profondeur += (c == '(') as i32 - (c == ')') as i32;
        if c == ',' && profondeur == 0 {
            couches.push(std::mem::take(&mut courante));
        } else {
            courante.push(c);
        }
    }
    couches.push(courante);
    couches
}

/// Un anneau (décalages et flou nuls, étalement seul) n'est pas une ombre : focus, pulsation, projecteur.
fn is_ring(couche: &str) -> bool {
    let sans_parentheses = PARENTHESES_RE.replace_all(couche, "");
    let longueurs: Vec<_> = captures(&LONGUEUR_RE, &sans_parentheses)
        .into_iter()
        .map(|(_, g)| g[0].to_string())
        .collect();
    longueurs.len() == 4
        && longueurs[..3]
            .iter()
            .all(|v| v.parse::<f64>().is_ok_and(|x| x == 0.0))
}

type Constat = (usize, &'static str, &'static str, String);

fn check_text(texte: &str, palette: &Palette) -> Vec<Constat> {
    let mut constats = Vec::new();
    for (offset, brut, rgb) in colors_in(texte) {
        if rgb == (255, 255, 255) {
            constats.push((
                offset,
                "erreur",
                "blanc-pur",
                format!("{brut} : jamais de blanc pur (Papier #F3EEE3 ou Crème #FFFBF3)"),
            ));
        } else if !palette.autorisees.contains(&rgb) {
            constats.push((
                offset,
                "erreur",
                "hors-palette",
                format!("{brut} n'est pas un jeton d'etymo.css"),
            ));
        }
    }

    let joindre = |s: BTreeSet<String>| s.into_iter().collect::<Vec<_>>().join(", ");
    for (offset, g) in captures(&DECL_RE, texte) {
        let (prop, valeur) = (g[0], g[1]);
        if valeur.contains("gradient") {
            let mut ressources = families_in(valeur, palette);
            ressources.remove("action");
            if ressources.len() >= 2 {
                constats.push((
                    offset,
                    "erreur",
                    "degrade-mixte",
                    format!("dégradé entre ressources ({})", joindre(ressources)),
                ));
            }
        }
        // Bande d'accent : un seul côté de 3 px ou plus. Un filet séparateur (1 à 2 px) reste permis.
        let epaisseur = EPAISSEUR_RE
            .captures(valeur)
            .expect("recherche")
            .map(|c| c[1].parse::<f64>().expect("nombre"));
        if BORD_RE.is_match(prop).expect("recherche") && epaisseur.is_some_and(|e| e >= 3.0) {
            constats.push((
                offset,
                "erreur",
                "bande-laterale",
                format!(
                    "{prop} : jamais de bande sur un seul côté, contour égal sur les quatre côtés"
                ),
            ));
        }
        if prop == "box-shadow" || prop == "text-shadow" || valeur.contains("drop-shadow") {
            let teintes: BTreeSet<String> = shadow_layers(valeur)
                .iter()
                .filter(|c| !is_ring(c))
                .flat_map(|c| families_in(c, palette))
                .collect();
            if !teintes.is_empty() {
                constats.push((
                    offset,
                    "avertissement",
                    "ombre-coloree",
                    format!(
                        "ombre {} : les ombres ne sont jamais colorées (sauf vitrine Corail, halos du rituel à justifier)",
                        joindre(teintes)
                    ),
                ));
            }
        }
    }

    for (debut, g) in captures(&BLOCK_RE, texte) {
        let corps = g[0];
        let decls: HashMap<&str, &str> = captures(&DECL_RE, corps)
            .into_iter()
            .map(|(_, d)| (d[0], d[1]))
            .collect();
        let fond = ["background-color", "background"]
            .iter()
            .find_map(|p| decls.get(p).filter(|v| !v.is_empty()))
            .copied()
            .unwrap_or("");
        let couleur = decls.get("color").copied().unwrap_or("");
        if fond.is_empty()
            || couleur.is_empty()
            || fond.contains("gradient")
            || !contient(fond, &VIVID_VARS, &VIVID_HEX)
        {
            continue;
        }
        let offset = debut + corps.find("color").unwrap();
        if contient(couleur, &LIGHT_VARS, &LIGHT_HEX) {
            constats.push((
                offset,
                "erreur",
                "texte-sur-vif",
                "texte clair sur couleur vive : le texte y est toujours en Encre".into(),
            ));
        } else if INK_RE.is_match(couleur).expect("recherche") {
            constats.push((
                offset,
                "avertissement",
                "encre-variable",
                "var(--ink) sur couleur vive s'inverse en nocturne : utiliser var(--fixed-ink)"
                    .into(),
            ));
        }
    }
    constats
}

fn resoudre(chemin: &Path) -> PathBuf {
    fs::canonicalize(chemin).unwrap_or_else(|_| std::path::absolute(chemin).expect("chemin absolu"))
}

fn git(racine: &Path, args: &[&str], chemins: &[String]) -> Result<String, String> {
    let sortie = Command::new("git")
        .args(args)
        .arg("--")
        .args(chemins)
        .current_dir(racine)
        .output()
        .map_err(|e| e.to_string())?;
    if !sortie.status.success() {
        return Err(String::from_utf8_lossy(&sortie.stderr).into_owned());
    }
    Ok(String::from_utf8_lossy(&sortie.stdout).into_owned())
}

/// Lignes ajoutées ou modifiées depuis `base`, par fichier ; `None` : fichier nouveau, tout est vérifié.
fn changed_lines(
    racine: &Path,
    base: &str,
    chemins: &[String],
) -> Result<HashMap<PathBuf, Option<HashSet<usize>>>, String> {
    let diff = git(racine, &["diff", "-U0", "--no-color", base], chemins)?;
    let (mut resultat, mut courant): (HashMap<PathBuf, Option<HashSet<usize>>>, Option<PathBuf>) =
        (HashMap::new(), None);
    for ligne in diff.split('\n') {
        if let Some(nom) = ligne.strip_prefix("+++ ") {
            courant = if ligne.ends_with("/dev/null") {
                None
            } else {
                Some(resoudre(&racine.join(nom.get(2..).unwrap_or(""))))
            };
        } else if ligne.starts_with("@@")
            && let Some(fichier) = &courant
        {
            let c = HUNK_RE
                .captures(ligne)
                .expect("recherche")
                .expect("en-tête de bloc");
            let debut: usize = c[1].parse().unwrap();
            let nombre: usize = c.get(2).map_or(1, |n| n.as_str().parse().unwrap());
            if let Some(lignes) = resultat
                .entry(fichier.clone())
                .or_insert_with(|| Some(HashSet::new()))
            {
                lignes.extend(debut..debut + nombre);
            }
        }
    }
    for nom in git(
        racine,
        &["ls-files", "--others", "--exclude-standard"],
        chemins,
    )?
    .split_whitespace()
    {
        resultat.insert(resoudre(&racine.join(nom)), None);
    }
    Ok(resultat)
}

fn fichiers_de(dossier: &Path, fichiers: &mut Vec<PathBuf>) {
    for entree in fs::read_dir(dossier).into_iter().flatten().flatten() {
        let chemin = entree.path();
        if chemin.is_dir() {
            fichiers_de(&chemin, fichiers);
        } else if chemin
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| EXTENSIONS.contains(&e))
        {
            fichiers.push(chemin);
        }
    }
}

fn main() -> ExitCode {
    let racine = resoudre(&Path::new(env!("CARGO_MANIFEST_DIR")).join(".."));
    let (mut chemins, mut base, mut strict) = (Vec::new(), None, false);
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                println!(
                    "{}",
                    include_str!("check_identity.rs")
                        .lines()
                        .take_while(|l| l.starts_with("//!"))
                        .map(|l| l.trim_start_matches("//!").strip_prefix(' ').unwrap_or(""))
                        .collect::<Vec<_>>()
                        .join("\n")
                );
                return ExitCode::SUCCESS;
            }
            "--strict" => strict = true,
            "--base" => match args.next() {
                Some(r) => base = Some(r),
                None => {
                    eprintln!("check_identity : l'option --base attend une révision");
                    return ExitCode::from(2);
                }
            },
            _ if arg.starts_with("--base=") => base = Some(arg["--base=".len()..].to_string()),
            _ if arg.starts_with('-') && arg != "-" => {
                eprintln!("check_identity : option inconnue {arg}");
                return ExitCode::from(2);
            }
            _ => chemins.push(arg),
        }
    }
    if chemins.is_empty() {
        chemins.push(racine.join("docs").to_string_lossy().into_owned());
    }

    let palette = load_palette(&racine);
    let mut fichiers = Vec::new();
    for c in &chemins {
        let p = resoudre(Path::new(c));
        if p.is_dir() {
            let mut trouves = Vec::new();
            fichiers_de(&p, &mut trouves);
            trouves.sort();
            fichiers.extend(trouves);
        } else {
            fichiers.push(p);
        }
    }

    let portee = match base
        .as_deref()
        .map(|b| changed_lines(&racine, b, &chemins))
        .transpose()
    {
        Ok(p) => p,
        Err(e) => {
            eprintln!("check_identity : git a échoué : {e}");
            return ExitCode::FAILURE;
        }
    };
    let (mut erreurs, mut avertissements) = (0, 0);
    for chemin in &fichiers {
        let lignes_vues = match &portee {
            Some(p) => match p.get(chemin) {
                Some(l) => l.as_ref(),
                None => continue,
            },
            None => None,
        };
        let texte = match fs::read_to_string(chemin) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("check_identity : {} : {e}", chemin.display());
                return ExitCode::FAILURE;
            }
        };
        let lignes: Vec<&str> = texte.split('\n').collect();
        let mut debuts = vec![0];
        for l in &lignes {
            debuts.push(debuts.last().unwrap() + l.len() + 1);
        }
        let mut vus = HashSet::new();
        let mut constats = check_text(&texte, &palette);
        constats.sort();
        for (offset, severite, regle, message) in constats {
            let numero = debuts.iter().position(|&d| d > offset).unwrap();
            if SKIP_MARKERS.iter().any(|m| lignes[numero - 1].contains(m))
                || vus.contains(&(numero, regle, message.clone()))
            {
                continue;
            }
            if lignes_vues.is_some_and(|l| !l.contains(&numero)) {
                continue;
            }
            vus.insert((numero, regle, message.clone()));
            if severite == "erreur" {
                erreurs += 1
            } else {
                avertissements += 1
            }
            let affiche = chemin.strip_prefix(&racine).unwrap_or(chemin);
            println!(
                "{}:{numero}: [{severite}] {regle} — {message}",
                affiche.display()
            );
        }
    }

    println!("\n{erreurs} erreur(s), {avertissements} avertissement(s)");
    if erreurs > 0 || (strict && avertissements > 0) {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
