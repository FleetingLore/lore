use crate::node::{DomainType, Node, NodeType};

// 解析原始文本为Line枚举
pub fn parse_lines(input: &str) -> Vec<LineContent> {
    input
        .split('\n')
        .filter(|line| !line.trim().is_empty())
        .map(parse_line)
        .collect()
}

// 解析单行
pub fn parse_line(line: &str) -> LineContent {
    let trimmed = line.trim_start();
    let indent = (line.len() - trimmed.len()) / 2; // 每2个空格算一级缩进

    // 检查是否为注释
    if trimmed.starts_with('#') && trimmed.len() > 1 && !trimmed.starts_with("# ") {
        let content = trimmed[1..].trim();
        return LineContent::Comment(content.to_string(), indent);
    }

    // 检查是否为占位符
    if trimmed == "#" {
        return LineContent::PlaceHolder(indent);
    }

    // 检查是否为域 (格式: + name)
    if trimmed.starts_with('+') && trimmed.len() > 1 {
        let domain_name = trimmed[1..].trim();
        return LineContent::Domain(domain_name.to_string(), indent);
    }

    // 检查是否为链接 (格式: name = value)
    if let Some(pos) = trimmed.find('=') {
        let before_eq = trimmed[..pos].trim();
        let after_eq = trimmed[pos + 1..].trim();

        // 检查是否是引用 (格式: name > value)
        if before_eq.contains(">") {
            let parts: Vec<&str> = before_eq.split(">").collect();
            if parts.len() == 2 {
                let name = parts[0].trim();
                let reference = parts[1].trim();
                return LineContent::Reference(name.to_string(), reference.to_string(), indent);
            }
        }

        // 普通链接
        return LineContent::Link(before_eq.to_string(), after_eq.to_string(), indent);
    }

    // 默认为原子元素
    LineContent::Atom(trimmed.to_string(), indent)
}

// 定义LineContent枚举
#[derive(Debug)]
pub enum LineContent {
    PlaceHolder(usize),
    Atom(String, usize),
    Comment(String, usize),
    Link(String, String, usize),
    Domain(String, usize),
    Reference(String, String, usize),
}

// 转换为Node结构
pub fn into_nodes(lines: &Vec<LineContent>) -> Vec<Node> {
    if lines.is_empty() {
        return Vec::new();
    }

    let mut nodes: Vec<Node> = Vec::new();
    let mut stack: Vec<(usize, usize)> = Vec::new(); // (缩进级别, 节点索引)

    for line in lines {
        let current_indent = match line {
            LineContent::PlaceHolder(indent) => *indent,
            LineContent::Atom(_, indent) => *indent,
            LineContent::Comment(_, indent) => *indent,
            LineContent::Link(_, _, indent) => *indent,
            LineContent::Domain(_, indent) => *indent,
            LineContent::Reference(_, _, indent) => *indent,
        };

        // 处理缩进级别变化
        while let Some((stack_indent, _)) = stack.last() {
            if *stack_indent >= current_indent {
                stack.pop();
            } else {
                break;
            }
        }

        // 创建新节点
        let node = match line {
            LineContent::PlaceHolder(_) => Node {
                node_type: NodeType::PlaceHolder,
                rails: Vec::new(),
            },
            LineContent::Atom(content, _) => Node {
                node_type: NodeType::Element(content.clone()),
                rails: Vec::new(),
            },
            LineContent::Comment(content, _) => Node {
                node_type: NodeType::Comment(content.clone()),
                rails: Vec::new(),
            },
            LineContent::Link(name, value, _) => Node {
                node_type: NodeType::Rail(name.clone(), value.clone()),
                rails: Vec::new(),
            },
            LineContent::Domain(name, _) => Node {
                node_type: NodeType::Domain(DomainType::Category1(name.clone())),
                rails: Vec::new(),
            },
            LineContent::Reference(name, value, _) => Node {
                node_type: NodeType::Domain(DomainType::Category2(name.clone(), value.clone())),
                rails: Vec::new(),
            },
        };

        let node_index = nodes.len();
        nodes.push(node);

        // 将节点添加到父节点的rails中
        if let Some((_, parent_index)) = stack.last() {
            // 获取父节点的克隆
            let mut parent_clone = nodes[*parent_index].clone();

            // 检查父节点是否是Domain类型
            if let NodeType::Domain(_) = &parent_clone.node_type {
                // 获取子节点的克隆（如果存在）
                if node_index < nodes.len() {
                    let child_node = nodes[node_index].clone();
                    parent_clone.rails.push(child_node);

                    // 将修改后的父节点克隆写回原位置
                    nodes[*parent_index] = parent_clone;
                }
            }
        } else {
            // 根节点
            nodes.truncate(node_index + 1); // 确保节点被添加
        }

        // 如果新节点是Domain类型，将其推入栈中
        match line {
            LineContent::Domain(_, _) | LineContent::Reference(_, _, _) => {
                stack.push((current_indent, node_index));
            }
            _ => {}
        }
    }

    // 只返回根节点（缩进为0的节点）
    let mut root_nodes = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let indent = match line {
            LineContent::PlaceHolder(indent) => *indent,
            LineContent::Atom(_, indent) => *indent,
            LineContent::Comment(_, indent) => *indent,
            LineContent::Link(_, _, indent) => *indent,
            LineContent::Domain(_, indent) => *indent,
            LineContent::Reference(_, _, indent) => *indent,
        };

        if indent == 0 && i < nodes.len() {
            root_nodes.push(nodes[i].clone());
        }
    }

    root_nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line_atom() {
        let line = "这是一个原子元素";
        let result = parse_line(line);

        match result {
            LineContent::Atom(content, indent) => {
                assert_eq!(content, "这是一个原子元素");
                assert_eq!(indent, 0);
            }
            _ => panic!("Expected Atom, got {:?}", result),
        }
    }

    #[test]
    fn test_parse_line_atom_with_indent() {
        let line = "  缩进的原子元素";
        let result = parse_line(line);

        match result {
            LineContent::Atom(content, indent) => {
                assert_eq!(content, "缩进的原子元素");
                assert_eq!(indent, 1); // 2个空格 = 1级缩进
            }
            _ => panic!("Expected Atom with indent, got {:?}", result),
        }
    }

    #[test]
    fn test_parse_line_comment() {
        let line = "# 这是一条注释";
        let result = parse_line(line);

        match result {
            LineContent::Comment(content, indent) => {
                assert_eq!(content, "这是一条注释");
                assert_eq!(indent, 0);
            }
            _ => panic!("Expected Comment, got {:?}", result),
        }
    }

    #[test]
    fn test_parse_line_placeholder() {
        let line = "#";
        let result = parse_line(line);

        match result {
            LineContent::PlaceHolder(indent) => {
                assert_eq!(indent, 0);
            }
            _ => panic!("Expected PlaceHolder, got {:?}", result),
        }
    }

    #[test]
    fn test_parse_line_domain() {
        let line = "+ 域名标题";
        let result = parse_line(line);

        match result {
            LineContent::Domain(name, indent) => {
                assert_eq!(name, "域名标题");
                assert_eq!(indent, 0);
            }
            _ => panic!("Expected Domain, got {:?}", result),
        }
    }

    #[test]
    fn test_parse_line_link() {
        let line = "链接名称 = http://example.com";
        let result = parse_line(line);

        match result {
            LineContent::Link(name, value, indent) => {
                assert_eq!(name, "链接名称");
                assert_eq!(value, "http://example.com");
                assert_eq!(indent, 0);
            }
            _ => panic!("Expected Link, got {:?}", result),
        }
    }

    #[test]
    fn test_parse_line_reference() {
        let line = "父域名 > 子标题";
        let result = parse_line(line);

        match result {
            LineContent::Reference(name, value, indent) => {
                assert_eq!(name, "父域名");
                assert_eq!(value, "子标题");
                assert_eq!(indent, 0);
            }
            _ => panic!("Expected Reference, got {:?}", result),
        }
    }

    #[test]
    fn test_parse_line_link_with_spaces() {
        let line = "  链接名称 = http://example.com";
        let result = parse_line(line);

        match result {
            LineContent::Link(name, value, indent) => {
                assert_eq!(name, "链接名称");
                assert_eq!(value, "http://example.com");
                assert_eq!(indent, 1); // 2个空格缩进
            }
            _ => panic!("Expected Link with indent, got {:?}", result),
        }
    }

    #[test]
    fn test_parse_lines_simple() {
        let input = "原子元素\n# 注释\n+ 域名\n  子元素";
        let results = parse_lines(input);

        assert_eq!(results.len(), 4);

        match &results[0] {
            LineContent::Atom(content, indent) => {
                assert_eq!(content, "原子元素");
                assert_eq!(*indent, 0);
            }
            _ => panic!("First should be Atom"),
        }

        match &results[1] {
            LineContent::Comment(content, indent) => {
                assert_eq!(content, "注释");
                assert_eq!(*indent, 0);
            }
            _ => panic!("Second should be Comment"),
        }

        match &results[2] {
            LineContent::Domain(name, indent) => {
                assert_eq!(name, "域名");
                assert_eq!(*indent, 0);
            }
            _ => panic!("Third should be Domain"),
        }

        match &results[3] {
            LineContent::Atom(content, indent) => {
                assert_eq!(content, "子元素");
                assert_eq!(*indent, 1);
            }
            _ => panic!("Fourth should be Atom with indent"),
        }
    }

    #[test]
    fn test_parse_lines_empty_lines() {
        let input = "\n原子元素\n\n# 注释\n\n";
        let results = parse_lines(input);

        assert_eq!(results.len(), 2); // 只应该解析非空行
    }

    #[test]
    fn test_into_nodes_simple_structure() {
        let lines = vec![
            LineContent::Domain("主域名".to_string(), 0),
            LineContent::Atom("子元素".to_string(), 1),
            LineContent::Link("链接".to_string(), "http://example.com".to_string(), 1),
        ];

        let nodes = into_nodes(&lines);

        assert_eq!(nodes.len(), 1); // 应该只有一个根节点

        let root = &nodes[0];
        match &root.node_type {
            NodeType::Domain(domain_type) => {
                match domain_type {
                    DomainType::Category1(name) => assert_eq!(name, "主域名"),
                    _ => panic!("Expected Category1"),
                }
            }
            _ => panic!("Root should be Domain"),
        }

        assert_eq!(root.rails.len(), 2); // 应该有两个子节点

        match &root.rails[0].node_type {
            NodeType::Element(content) => assert_eq!(content, "子元素"),
            _ => panic!("First child should be Element"),
        }

        match &root.rails[1].node_type {
            NodeType::Rail(name, value) => {
                assert_eq!(name, "链接");
                assert_eq!(value, "http://example.com");
            }
            _ => panic!("Second child should be Rail"),
        }
    }

    #[test]
    fn test_into_nodes_nested_structure() {
        let lines = vec![
            LineContent::Domain("一级域名".to_string(), 0),
            LineContent::Domain("二级域名".to_string(), 1),
            LineContent::Atom("三级元素".to_string(), 2),
            LineContent::Reference("父级".to_string(), "引用".to_string(), 1),
            LineContent::Link("链接".to_string(), "http://test.com".to_string(), 2),
        ];

        let nodes = into_nodes(&lines);

        assert_eq!(nodes.len(), 1); // 一个根节点

        let root = &nodes[0];
        assert_eq!(root.rails.len(), 2); // 根节点有两个直接子节点

        // 第一个子节点是二级域名
        let child1 = &root.rails[0];
        match &child1.node_type {
            NodeType::Domain(domain_type) => {
                match domain_type {
                    DomainType::Category1(name) => assert_eq!(name, "二级域名"),
                    _ => panic!("Expected Category1"),
                }
            }
            _ => panic!("Child1 should be Domain"),
        }

        // 二级域名应该有一个子节点
        assert_eq!(child1.rails.len(), 1);

        let grandchild = &child1.rails[0];
        match &grandchild.node_type {
            NodeType::Element(content) => assert_eq!(content, "三级元素"),
            _ => panic!("Grandchild should be Element"),
        }

        // 第二个子节点是引用
        let child2 = &root.rails[1];
        match &child2.node_type {
            NodeType::Domain(domain_type) => {
                match domain_type {
                    DomainType::Category2(name, value) => {
                        assert_eq!(name, "父级");
                        assert_eq!(value, "引用");
                    }
                    _ => panic!("Expected Category2"),
                }
            }
            _ => panic!("Child2 should be Domain with Category2"),
        }

        // 引用应该有一个子节点
        assert_eq!(child2.rails.len(), 1);

        let ref_child = &child2.rails[0];
        match &ref_child.node_type {
            NodeType::Rail(name, value) => {
                assert_eq!(name, "链接");
                assert_eq!(value, "http://test.com");
            }
            _ => panic!("Reference child should be Rail"),
        }
    }

    #[test]
    fn test_into_nodes_with_comments_and_placeholder() {
        let lines = vec![
            LineContent::Comment("开始".to_string(), 0),
            LineContent::Domain("测试域".to_string(), 0),
            LineContent::PlaceHolder(1),
            LineContent::Comment("中间注释".to_string(), 1),
            LineContent::Atom("有效元素".to_string(), 1),
        ];

        let nodes = into_nodes(&lines);

        assert_eq!(nodes.len(), 1); // 注释和占位符被过滤，只有域名节点

        let root = &nodes[0];
        match &root.node_type {
            NodeType::Domain(domain_type) => {
                match domain_type {
                    DomainType::Category1(name) => assert_eq!(name, "测试域"),
                    _ => panic!("Expected Category1"),
                }
            }
            _ => panic!("Root should be Domain"),
        }

        // 域名应该只有一个子节点（占位符和注释被过滤）
        assert_eq!(root.rails.len(), 1);

        match &root.rails[0].node_type {
            NodeType::Element(content) => assert_eq!(content, "有效元素"),
            _ => panic!("Should be Element"),
        }
    }

    #[test]
    fn test_into_nodes_multiple_roots() {
        let lines = vec![
            LineContent::Domain("域名1".to_string(), 0),
            LineContent::Atom("元素1".to_string(), 1),
            LineContent::Domain("域名2".to_string(), 0),
            LineContent::Atom("元素2".to_string(), 1),
        ];

        let nodes = into_nodes(&lines);

        assert_eq!(nodes.len(), 2); // 两个根节点

        // 检查第一个根节点
        match &nodes[0].node_type {
            NodeType::Domain(domain_type) => {
                match domain_type {
                    DomainType::Category1(name) => assert_eq!(name, "域名1"),
                    _ => panic!("Expected Category1"),
                }
            }
            _ => panic!("First should be Domain"),
        }
        assert_eq!(nodes[0].rails.len(), 1);

        // 检查第二个根节点
        match &nodes[1].node_type {
            NodeType::Domain(domain_type) => {
                match domain_type {
                    DomainType::Category1(name) => assert_eq!(name, "域名2"),
                    _ => panic!("Expected Category1"),
                }
            }
            _ => panic!("Second should be Domain"),
        }
        assert_eq!(nodes[1].rails.len(), 1);
    }

    #[test]
    fn test_into_nodes_empty_input() {
        let lines: Vec<LineContent> = vec![];
        let nodes = into_nodes(&lines);

        assert!(nodes.is_empty());
    }

    #[test]
    fn test_indent_calculation() {
        // 测试不同缩进级别的解析
        let test_cases = vec![
            ("无缩进", 0),
            ("  2空格", 1),
            ("    4空格", 2),
            ("      6空格", 3),
            ("        8空格", 4),
        ];

        for (input, expected_indent) in test_cases {
            let result = parse_line(input);
            let indent = match result {
                LineContent::Atom(_, indent) => indent,
                _ => panic!("Expected Atom for input: {}", input),
            };
            assert_eq!(indent, expected_indent, "Failed for input: {}", input);
        }
    }
}
