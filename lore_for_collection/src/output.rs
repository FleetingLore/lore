use std::fs;
use std::path::Path;
use crate::line::{Content, Line};

// 生成 html 文件
pub fn output_html(title: &str, lines: Vec<Line>, path: &Path) {
    let mut html = String::new();

    html.push_str(
        format!(
            r##"<!DOCTYPE html>
<html>
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>{}</title>
<styles>
/* 全局样式重置：统一浏览器默认样式，消除边距、内边距差异 */
* {{
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}}

/* 页面基础样式：设置背景、字体、最小高度，保证页面铺满视口 */
body {{
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
    background-color: #f8f9fa; /* 浅灰色背景，柔和不刺眼 */
    color: #212529; /* 深灰色文字，比纯黑更易读 */
    min-height: 100vh; /* 最小高度为视口高度，避免内容过少时页面塌陷 */
    padding: 2rem; /* 页面内边距，让内容与浏览器边缘有间距 */
    line-height: 1.6; /* 行高，提升文字可读性 */
}}

/* 段落样式：统一间距，避免多个p标签紧挨 */
p {{
    margin-bottom: 1rem; /* 段落底部间距，区分多个链接 */
}}

/* 链接基础样式：去除下划线，设置基础颜色和过渡效果 */
a {{
    text-decoration: none; /* 消除默认下划线 */
    color: #0d6efd; /* 经典蓝色，符合通用链接视觉习惯 */
    font-size: 1.1rem; /* 字体稍大，提升点击体验 */
    font-weight: 500; /* 字体中等加粗，突出链接 */
    transition: all 0.2s ease; /* 悬停效果过渡，更丝滑 */
}}

/* 链接悬停样式：鼠标移入时的交互效果 */
a:hover {{
    color: #0a58ca; /* 加深蓝色，视觉反馈 */
    text-decoration: underline; /* 显示下划线，明确可点击 */
}}

/* 链接激活样式：点击瞬间的效果 */
a:active {{
    color: #084298; /* 更深的蓝色，强化点击反馈 */
}}

</styles>
</head>
<body>
"##,
            title,
        ).as_str()
    );

    for line in lines {
        html.push_str(line_to_html(&line).as_str());
    }

    html.push_str(
        r#"
</body>
</html>"#
    );

    fs::write(path, html).unwrap();
}

// 以行为单位的转换
fn line_to_html(line: &Line) -> String {
    // 缩进参数
    let margin_left = line.indent * 20;

    // 构建返回标签
    match &line.content {
        // 原子
        Content::Atom(atom) => {
            format!(
                r#"<p style="margin-left: {}">{}</p>"#,
                margin_left,
                atom
            )
        },

        // 链接
        Content::Link(key, value) => {
            format!(
                r#"<p style="margin-left: {}"><a href="{}" target="_blank">{}</a></p>"#,
                margin_left,
                value,
                key
            )
        },

        // 领域
        Content::Domain(domain) => {
            format!(
                r#"<p style="margin-left: {}"><strong>+ {}</strong></p>"#,
                margin_left,
                domain
            )
        }
    }
}
