use regex::Regex;
use std::path::Path;

pub struct NameCleaner {
    re_season_ep: Regex,
    re_year: Regex,
    re_garbage: Regex,
}

impl NameCleaner {
    pub fn new() -> Self {
        Self {
            // Captura S01E01, 1x01, o Cap.110 / Capitulo 110
            re_season_ep: Regex::new(r"(?i)[sS](\d+)[eE](\d+)|(\d+)x(\d+)|(?:Cap|Capitulo|Episodio|Ep)[\.\s]?(\d+)").unwrap(),
            // Captura años (1900-2099)
            re_year: Regex::new(r"(19|20)\d{2}").unwrap(),
            // Basura técnica: resoluciones, codecs, grupos y fuentes (añadido HDTV)
            re_garbage: Regex::new(r"(?i)(HDTV|1080p|720p|4k|2160p|x264|x265|h264|h265|webrip|web-dl|bluray|bdrip|dvdrip|xvid|rip|english|spanish|castellano|subs|multi|dual|ddp5\.1|atmos|ac3|dts|-[a-zA-Z0-9]+$)").unwrap(),
        }
    }

    pub fn clean(&self, filename: &str) -> String {
        let path = Path::new(filename);
        let stem = path.file_stem().unwrap_or_default().to_string_lossy();
        let extension = path.extension().unwrap_or_default().to_string_lossy();

        // 1. Detección de Temporada/Episodio (Ancla)
        let mut season_info = String::new();
        let mut title_end_index = stem.len();

        if let Some(caps) = self.re_season_ep.captures(&stem) {
            title_end_index = caps.get(0).unwrap().start();

            if let (Some(s), Some(e)) = (caps.get(1).or(caps.get(3)), caps.get(2).or(caps.get(4))) {
                season_info = format!(
                    "S{:0>2}E{:0>2}",
                    s.as_str().parse::<u32>().unwrap_or(1),
                    e.as_str().parse::<u32>().unwrap_or(0)
                );
            } else if let Some(e_match) = caps.get(5) {
                let val = e_match.as_str();
                season_info = if val.len() >= 3 {
                    let split_pos = val.len() - 2;
                    format!(
                        "S{:0>2}E{:0>2}",
                        &val[0..split_pos].parse::<u32>().unwrap_or(1),
                        &val[split_pos..].parse::<u32>().unwrap_or(0)
                    )
                } else {
                    format!("S01E{:0>2}", val.parse::<u32>().unwrap_or(0))
                };
            }
        }

        // 2. Extraer el Año (solo en la zona del título)
        let mut year_info = String::new();
        if let Some(last_year) = self.re_year.find_iter(&stem[..title_end_index]).last() {
            year_info = format!("({})", last_year.as_str());
        }

        // 3. Limpieza del Título
        let mut name_part = stem[..title_end_index].to_string();

        // --- MAGIA: Eliminar etiquetas entre corchetes o paréntesis ---
        // Esto quita [SubsCastellano], [Grupo], (2022), etc.
        let re_brackets = Regex::new(r"\[.*?\]|\(.*?\)").unwrap();
        name_part = re_brackets.replace_all(&name_part, " ").into_owned();

        // Limpiar basura técnica restante
        name_part = self.re_garbage.replace_all(&name_part, " ").into_owned();

        // Si quitamos el año del título pero lo queremos al final, nos aseguramos de borrarlo aquí
        if !year_info.is_empty() {
            name_part = name_part.replace(&year_info.replace("(", "").replace(")", ""), " ");
        }

        // Convertir todo lo no-alfanumérico en espacios
        let name_cleaned: String = name_part
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { ' ' })
            .collect();

        // Capitalización forzada (Todo En Mayúsculas Cada Palabra)
        let final_title = name_cleaned
            .split_whitespace()
            .map(|word| {
                let mut c = word.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + &c.as_str().to_lowercase(),
                }
            })
            .collect::<Vec<String>>()
            .join(" ");

        // 4. Ensamblado
        if !season_info.is_empty() {
            format!("{} - {}.{}", final_title, season_info, extension)
        } else if !year_info.is_empty() {
            format!("{} {}.{}", final_title, year_info, extension)
        } else {
            format!("{}.{}", final_title, extension)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rsname_full_logic() {
        let cleaner = NameCleaner::new();

        // Test solicitado: Entrepreneurs [HDTV 720p][Cap.110].mkv -> S01E10
        assert_eq!(
            cleaner.clean("Entrepreneurs [HDTV 720p][Cap.110].mkv"),
            "Entrepreneurs - S01E10.mkv"
        );

        // Test Series: 1x01 -> S01E01
        assert_eq!(
            cleaner.clean("The.Simpsons.3x12.avi"),
            "The Simpsons - S03E12.avi"
        );

        // Test Películas: Año y limpieza de basura técnica
        assert_eq!(
            cleaner.clean("Pulp_Fiction_1994_720p_Bluray.mp4"),
            "Pulp Fiction (1994).mp4"
        );

        // Test Películas: Priorizar el último año (2012 es el título, 2009 es el estreno)
        assert_eq!(
            cleaner.clean("2012.Movie.2009.1080p.mkv"),
            "2012 Movie (2009).mkv"
        );

        // Test Formato español sin punto
        assert_eq!(
            cleaner.clean("Chernobyl Cap 103 1080p.mkv"),
            "Chernobyl - S01E03.mkv"
        );
    }
    #[test]
    fn test_rsname_diverse_battery() {
        let cleaner = NameCleaner::new();

        // 1. Caso: El año es parte del título pero también hay año de estreno
        assert_eq!(
            cleaner.clean("Blade.Runner.2049.2017.1080p.mkv"),
            "Blade Runner 2049 (2017).mkv"
        );

        // 2. Caso: Serie con "Capitulo" completo y número de 3 cifras (S02E15)
        assert_eq!(
            cleaner.clean("Better.Call.Saul.Capitulo.215.720p.h264.mp4"),
            "Better Call Saul - S02E15.mp4"
        );

        // 3. Caso: Archivo con múltiples puntos y guiones bajos mezclados
        assert_eq!(
            cleaner.clean("The_Last_of_Us.S01E03.Long.Long.Time.1080p.mkv"),
            "The Last Of Us - S01E03.mkv"
        );

        // 4. Caso: Documentales o series con "Ep" y espacio
        assert_eq!(
            cleaner.clean("Cosmos Ep 01 [Bluray].mkv"),
            "Cosmos - S01E01.mkv"
        );

        // 5. Caso: Película antigua sin año (debe quedar solo el nombre limpio)
        assert_eq!(
            cleaner.clean("Casablanca.Bluray.Rip.x264.mkv"),
            "Casablanca.mkv"
        );

        // 6. Caso: Formato de temporada con 'x' (2x01 -> S02E01)
        assert_eq!(
            cleaner.clean("Succession.2x01.HDTV.mkv"),
            "Succession - S02E01.mkv"
        );

        // 7. Caso: Anime con corchetes de grupo al principio y Cap. de 3 cifras
        assert_eq!(
            cleaner.clean("[SubsCastellano] One Piece Cap 1045.mkv"),
            "One Piece - S10E45.mkv"
        );

        // 8. Caso: Nombre de archivo que es puro ruido pero tiene un año
        assert_eq!(
            cleaner.clean("www.descargaslocas.com_Avatar.2.2022.English.x264.mkv"),
            "Www Descargaslocas Com Avatar 2 (2022).mkv"
        );
    }
}
