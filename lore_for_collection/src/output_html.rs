use crate::node::{DomainType, Node, NodeType};

// 生成HTML
pub fn generate_html(nodes: Vec<Node>, title: String) -> String {
    let mut html = String::new();

    html.push_str(format!(
        r#"<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{}</title>
  <link rel="stylesheet" href="https://fleetinglore.github.io/collection/collection.css">
</head>
<body>
  <details open>
      <summary>{}</summary>
      <div style="margin-left:17px">
"#,
        title, title
    ).as_str());

    for node in nodes {
        html.push_str(node_to_html(&node, 2).as_str());
    }

    html.push_str(r#"    </div>
  </details>
</body>
</html>"#);

    html
}

// 将节点转换为HTML
fn node_to_html(node: &Node, level: usize) -> String {
    match &node.node_type {
        NodeType::Comment(_) | NodeType::PlaceHolder => String::new(),
        NodeType::Domain(domain_type) => {
            let indent = "  ".repeat(level);
            let mut html = String::new();

            let is_open = if level == 2 { " open" } else { "" };
            html.push_str(format!("{}<details{}>\n", indent, is_open).as_str());

            let title = match domain_type {
                DomainType::Category1(name) => name.clone(),
                DomainType::Category2(name, content) => format!("{} = {}", name, content),
            };

            html.push_str(&format!("{}  <summary>{}</summary>\n", indent, title));

            if !node.rails.is_empty() {
                html.push_str(&format!("{}  <div style=\"margin-left:20px\">\n", indent));
                for child in &node.rails {
                    html.push_str(&node_to_html(child, level + 1));
                }
                html.push_str(&format!("{}  </div>\n", indent));
            }

            html.push_str(&format!("{}</details>\n", indent));
            html
        }
        NodeType::Rail(name, content) => {
            let indent = "  ".repeat(level);
            format!("{}<a href=\"{}\" target=\"_blank\">{}</a>\n", indent, content, name)
        }
        NodeType::Element(content) => {
            let indent = "  ".repeat(level);
            format!("{}<p>{}</p>\n", indent, content)
        }
    }
}
