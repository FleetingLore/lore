// src/node.rs

use crate::line::{Content, Line};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct LoreNode<'f> {
    pub content: Content<'f>,
    pub children: Vec<Rc<RefCell<LoreNode<'f>>>>,
}

#[derive(Debug)]
pub struct Root<'f> {
    pub name: String,
    pub nodes: Vec<Rc<RefCell<LoreNode<'f>>>>,
}

pub fn into_root<'f>(name: String, lines: &'f [Line<'f>]) -> Root<'f> {
    if lines.is_empty() {
        return Root {
            name,
            nodes: Vec::new(),
        };
    }

    // 创建所有节点
    let nodes: Vec<Rc<RefCell<LoreNode<'f>>>> = lines
        .iter()
        .map(|line| {
            Rc::new(RefCell::new(LoreNode {
                content: match &line.content {
                    Content::Atom(text) => Content::Atom(text),
                    Content::Link(key, value) => Content::Link(key, value),
                    Content::Domain(domain) => Content::Domain(domain),
                },
                children: Vec::new(),
            }))
        })
        .collect();

    // 使用栈来建立父子关系
    let mut stack: Vec<(usize, Rc<RefCell<LoreNode<'f>>>)> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        // 弹出栈顶元素，直到找到合适的父节点
        while let Some((indent, _)) = stack.last() {
            if *indent >= line.indent {
                stack.pop();
            } else {
                break;
            }
        }

        // 将当前节点添加到父节点的子节点中
        if let Some((_, parent)) = stack.last() {
            parent.borrow_mut().children.push(Rc::clone_from_ref(&nodes[i]));
        }

        // 将当前节点压入栈
        stack.push((line.indent, Rc::clone_from_ref(&nodes[i])));
    }

    // 收集顶层节点（缩进为0）
    let mut top_level_nodes = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if line.indent == 0 {
            top_level_nodes.push(Rc::clone_from_ref(&nodes[i]));
        }
    }

    Root {
        name,
        nodes: top_level_nodes,
    }
}

// 辅助函数：计算节点深度（递归）
pub fn calculate_depth<'f>(node: &Rc<RefCell<LoreNode<'f>>>, current_depth: usize) -> usize {
    let node_ref = node.borrow();
    if node_ref.children.is_empty() {
        current_depth
    } else {
        node_ref.children.iter()
            .map(|child| calculate_depth(child, current_depth + 1))
            .max()
            .unwrap_or(current_depth)
    }
}