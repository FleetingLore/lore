// Node结构定义
#[derive(Debug, Clone)]
pub struct Node {
    pub node_type: NodeType,
    pub rails: Vec<Node>,
}

#[derive(Debug, Clone)]
pub enum NodeType {
    PlaceHolder,
    Comment(String),
    Element(String),
    Rail(String, String),  // name, content
    Domain(DomainType),
}

#[derive(Debug, Clone)]
pub enum DomainType {
    Category1(String),     // +name
    Category2(String, String), // name -> value
}
