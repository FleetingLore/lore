use crate::line::{Content, Line};

pub fn parse_line(line: &str) -> Line {
    // 移除 line 的缩进然后提取缩进级别数
    let trimmed = line.trim_start();

    // 计算 line 的缩进数
    let indent = (line.len() - trimmed.len()) / 2;

    // 解析 domain 节点
    if trimmed.starts_with('+') && trimmed.len() > 1 {
        // 生成实例
        let content = Content::Domain(trimmed[1..].trim().to_string());

        // 返回数据
        Line {
            indent,
            content
        }
    } else {
        // 解析 link 节点
        if let Some(pos) = trimmed.find('=') {
            // link 键
            let before_eq = trimmed[..pos].trim();

            // link 值
            let after_eq = trimmed[pos + 1..].trim();

            // 生成实例
            let content = Content::Link(before_eq.to_string(), after_eq.to_string());
            
            // 返回数据
            Line {
                indent,
                content
            }
        } else {
            // 生成实例
            let content = Content::Atom(trimmed.to_string());
            
            // 返回数据
            Line {
                indent,
                content
            }
        }
    }
}
