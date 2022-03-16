pub struct GitDetails {
    pub active_branch: String,
    pub contributor_count: i32,
    pub contributors: Vec<String>,
    pub remotes: Vec<String>,
    pub commits: i32,
}
pub enum GitEnable {
    Present(GitDetails),
    False,
}
pub enum ProjLang {
    Python,
    C,
    CPP,
    JavaScript,
    Rust,
    Java,
    None,
}

#[non_exhaustive]
pub struct ProjDetails {
    pub name: String,
    pub lines: i32,
    pub git: GitEnable,
    pub size: i32,
    pub language: ProjLang,
}

impl Default for ProjDetails {
    fn default() -> ProjDetails {
        ProjDetails {
            name: String::from(""),
            lines: 0,
            git: GitEnable::False,
            size: 0,
            language: ProjLang::None,
        }
    }
}

impl ProjDetails {
    pub fn SetName(&mut self, proj_name: String) {
        self.name = proj_name;
    }
}
