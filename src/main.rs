struct Operation<'a> {
    cle: &'a str,
    valeur: &'a str,
}

fn lire_reglage(ligne_config: &str) -> Result<(String, u32), String> {

    let reglage = match ligne_config.split_once("=") {
        Some((gauche, droite)) => Operation {
            cle: gauche.trim(),
            valeur: droite.trim() },
        None => return Err(format!("erreur : ligne malformee : {}", ligne_config)),
    };

    if reglage.cle.is_empty() {
        return Err("erreur : cle vide".to_string());
    }

    if reglage.valeur.chars().all(|value| !value.is_ascii_digit()) {
        return Err(format!("erreur : valeur invalide pour {} : {}",reglage.cle, reglage.valeur ));
    }

    Ok((reglage.cle.to_string(), reglage.valeur.parse::<u32>().unwrap_or(0)))
    
}

fn main() {

    match lire_reglage("port=8080") {
        Ok((cle, valeur)) => println!("{} = {}", cle, valeur),
        Err(_) => {}
    };

    match lire_reglage("time = 30") {
        Ok((cle, valeur)) => println!("{} = {}", cle, valeur),
        Err(_) => {}
    };
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn check_valeur_est_un_chiffre() {
        let chaine: &str = "retries=beaucoup";
        assert_eq!(Err("erreur : valeur invalide pour retries : beaucoup".to_string()), lire_reglage(chaine)); 
    }

    #[test]
    fn check_contient_une_cle() {
        let chaine: &str = "=42";
        assert_eq!(Err("erreur : cle vide".to_string()), lire_reglage(chaine)); 
    }

    #[test]
    fn check_contient_un_egal() {
        let chaine: &str = "max_conn";
        assert_eq!(Err("erreur : ligne malformee : max_conn".to_string()), lire_reglage(chaine));
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
