use super::AppMode;
use super::NameCleaner;
use super::RenameItem;
use walkdir::WalkDir;

pub struct App {
    pub items: Vec<RenameItem>,
    pub selected_index: usize,
    pub cleaner: NameCleaner,
    pub mode: AppMode,
    #[allow(dead_code)]
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        let mut app = Self {
            items: Vec::new(),
            selected_index: 0,
            cleaner: NameCleaner::new(),
            mode: AppMode::Scanning,
            should_quit: false,
        };
        app.scan_current_dir();
        app
    }

    fn is_video_file(filename: &str) -> bool {
        if let Some(extension) = filename.rfind('.').map(|i| &filename[i + 1..]) {
            matches!(
                extension.to_lowercase().as_str(),
                "mkv"
                    | "avi"
                    | "mp4"
                    | "mov"
                    | "wmv"
                    | "flv"
                    | "webm"
                    | "m4v"
                    | "mpg"
                    | "mpeg"
                    | "3gp"
                    | "rmvb"
                    | "ts"
                    | "mts"
                    | "m2ts"
                    | "vob"
                    | "ogv"
                    | "dv"
                    | "divx"
                    | "asf"
            )
        } else {
            false
        }
    }

    pub fn confirm_rename(&mut self) {
        if self.items.iter().any(|i| i.selected) {
            self.mode = AppMode::Confirming;
        }
    }

    pub fn cancel_confirm(&mut self) {
        self.mode = AppMode::Reviewing;
    }

    pub fn scan_current_dir(&mut self) {
        self.mode = AppMode::Scanning;
        let mut new_items = Vec::new();
        for entry in WalkDir::new(".")
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                let filename = entry.file_name().to_string_lossy().to_string();
                let cleaned = self.cleaner.clean(&filename);
                if filename != cleaned {
                    let is_video = Self::is_video_file(&filename);
                    new_items.push(RenameItem {
                        path: entry.path().to_path_buf(),
                        old_name: filename,
                        new_name: cleaned,
                        selected: is_video,
                    });
                }
            }
        }
        self.items = new_items;
        self.mode = AppMode::Reviewing;
    }

    pub fn run_rename(&mut self) {
        self.mode = AppMode::Processing;
        for item in self.items.iter_mut().filter(|i| i.selected) {
            let mut new_path = item.path.clone();
            new_path.set_file_name(&item.new_name);
            if std::fs::rename(&item.path, &new_path).is_ok() {
                item.path = new_path;
                item.old_name = item.new_name.clone();
                item.selected = false;
            }
        }
        self.mode = AppMode::Reviewing;
    }

    pub fn next(&mut self) {
        if !self.items.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.items.len();
        }
    }
    pub fn previous(&mut self) {
        if !self.items.is_empty() {
            self.selected_index = if self.selected_index == 0 {
                self.items.len() - 1
            } else {
                self.selected_index - 1
            };
        }
    }
    pub fn toggle_selection(&mut self) {
        if let Some(item) = self.items.get_mut(self.selected_index) {
            item.selected = !item.selected;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_video_file() {
        // Archivos de video deben retornar true
        assert!(App::is_video_file("movie.mkv"));
        assert!(App::is_video_file("video.avi"));
        assert!(App::is_video_file("film.mp4"));
        assert!(App::is_video_file("show.MOV")); // mayúsculas
        assert!(App::is_video_file("series.flv"));
        assert!(App::is_video_file("episode.webm"));
        assert!(App::is_video_file("clip.wmv"));

        // Archivos que no son video deben retornar false
        assert!(!App::is_video_file("audio.mp3"));
        assert!(!App::is_video_file("document.pdf"));
        assert!(!App::is_video_file("image.jpg"));
        assert!(!App::is_video_file("text.txt"));
        assert!(!App::is_video_file("archivo_sin_extension"));
        assert!(!App::is_video_file("subtitle.srt"));
    }
}
