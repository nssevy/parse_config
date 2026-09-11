struct Operation<'a> {
    cle: &'a str,
    valeur: &'a str,
}

fn lire_reglage(ligne_config: &str) -> Result<(String, u32), String> {

    let reglage = match ligne_config.split_once('=') {
        Some((gauche, droite)) => Operation {
            cle: gauche.trim(),
            valeur: droite.trim() },
        None => return Err(format!("ligne mal formée : {}", ligne_config)),
    };

    if reglage.cle.is_empty() {
        return Err("cle vide".to_string());
    }

    let valeur_num = match reglage.valeur.parse::<u32>() {
        Ok(v) => v,
        Err(_) => return Err(format!("valeur invalide pour {} : {}", reglage.cle, reglage.valeur)),
    };

    Ok((reglage.cle.to_string(), valeur_num))
    
}

fn main() {

    let lignes = ["sevy=8080vesd", "port=8080", "timeout = 30", "max_conn", "=42", "retries=beaucoup"];

    for ligne in lignes {
        match lire_reglage(ligne) {
            Ok((cle, valeur)) => println!("{} = {}", cle, valeur),
            Err(e) => eprintln!("erreur : {}", e),
        }
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn check_valeur_est_uniquement_un_chiffre() {
        let chaine: &str = "retries=beaucoup45";
        assert_eq!(Err("valeur invalide pour retries : beaucoup45".to_string()), lire_reglage(chaine)); 
    }

    #[test]
    fn check_contient_une_cle() {
        let chaine: &str = "=42";
        assert_eq!(Err("cle vide".to_string()), lire_reglage(chaine)); 
    }

    #[test]
    fn check_contient_un_egal() {
        let chaine: &str = "max_conn";
        assert_eq!(Err("ligne mal formée : max_conn".to_string()), lire_reglage(chaine));
    }

}

/*
Écris une fonction lire_reglage qui prend une ligne de configuration au format cle=valeur et renvoie un Result<(String, u32), String> : le nom du réglage et sa valeur numérique.

Trois cas d'échec, dans cet ordre :

pas de = dans la ligne → "ligne malformee : <la ligne>"
la clé est vide → "cle vide"
la valeur n'est pas un nombre → "valeur invalide pour <cle> : <la valeur>"

Les espaces autour de la clé et de la valeur doivent être tolérés.

Teste avec : "port=8080", "timeout = 30", "max_conn", "=42", "retries=beaucoup"

Résultat attendu

port = 8080
timeout = 30
erreur : ligne malformee : max_conn
erreur : cle vide
erreur : valeur invalide pour retries : beaucoup
*/
