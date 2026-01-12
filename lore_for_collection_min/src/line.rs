// 行的数据分为缩进和行内容
pub struct Line {
    pub indent: usize,
    pub content: Content,
}

// 行内容有三种
pub enum Content {
    Atom(String), // 原子
    Link(String, String), // 链接
    Domain(String) // 领域
}
