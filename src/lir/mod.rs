//! Lore IR
//!
//! 这个模块提供了将文本格式的 LIR 解析为结构化数据的功能。
//! LIR 使用缩进来表示层次结构，并使用 `+` 前缀表示域定义。
//!
//! ## 子模块
//!
//! - [`mod@content`] - 处理行内容解析，区分普通元素和域定义
//! - [`mod@line`] - 解析单行，处理缩进和内容的组合
//! - [`mod@root`] - 解析多行文本，构建完整的 LIR 结构
//!
//! ## 快速开始
//!
//! ```rust
//! use lir_parser::{parse_root, Root};
//!
//! let input = vec![
//!     "root element",
//!     "  + domain definition",
//!     "    nested element",
//! ];
//!
//! let root = parse_root(input);
//! println!("{}", root);
//! ```
//!
//! ## 解析规则
//!
//! - **缩进**: 每 2 个空格表示一级缩进
//! - **域定义**: 以 `+ ` 开头的行（注意必须有空格）
//! - **普通元素**: 其他所有行
//!
//! ## 示例输入
//!
//! ```text
//! root
//!   + domain
//!     element in domain
//!   another element
//! ```
//!
//! 对应输出结构：
//! - 根级别的普通元素 "root"
//! - 一级缩进的域 "+ domain"
//! - 二级缩进的普通元素 "element in domain"
//! - 一级缩进的普通元素 "another element"

pub mod content;
pub mod line;
pub mod root;