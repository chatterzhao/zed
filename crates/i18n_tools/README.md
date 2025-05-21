### I18n

Zed支持通过扩展实现多语言。我们提供了以下工具来帮助您创建和验证国际化扩展，特别是翻译文件:
- i18n-scan: 根据代码中硬编码生成 defaults.rs 文件
- i18n-scan-app-menus: 用于菜单文件的国际化，包含scan和replace两个子命令
- i18n-new: 创建国际化扩展项目
- i18n-validate: 检查国际化扩展项目的完整性
- i18n-reorganize: 重组翻译文件，使其与 `defaults.rs` 一致，翻译文件（是否缺键，是否多键，键顺序是否与 defaults.rs 一致）

#### 构建 i18n-tools
```bash
# 因为在 Zed 项目中使用了工作空间（workspace）的依赖管理方式，并且这些依赖项是在根目录的 Cargo.toml 中统一声明的。当你在子目录中直接运行 cargo build 时，Cargo 无法正确解析工作空间的依赖关系。
cd /Users/zhaoyu/Downloads/codding/zed && cargo build --package zed-i18n-tools
```


#### Finding Hardcoded Strings

要扫描代码库中应该被翻译的硬编码字符串:

```bash
# - cargo run : 使用Cargo（Rust的包管理器）运行一个项目
# - --package zed-i18n-tools : 指定要运行的包名为"zed-i18n-tools"
# - --bin i18n-scan : 指定要运行的二进制文件名为"i18n-scan"
# - -- : 表示后面的参数将传递给被运行的程序，而不是传递给cargo
# - crates/ : 第一个参数，指定要扫描的源代码目录
# - crates/i18n/core/defaults.rs : 第二个参数，指定生成的输出文件路径
# // defaults.rs
# // 开发者只需要使用 t!(cx, "key") 宏，开发者在写代码时使用 t! 宏和对应的键名
# // 如果这个键在 defaults.rs 中不存在，会在日志中警告
# // 开发者需要在 defaults.rs 中添加对应的默认英文文本，修改默认文本只需要改 defaults.rs 一个地方
# // 当加载语言包时，如果找到对应翻译就使用翻译，否则使用默认文本
# // 可以通过工具自动导出所有需要翻译的文本

# top_menu_bar: crates/zed/src/zed/app_menus.rs
# extensions: crates/extensions_ui/src/extensions_ui.rs
# dock_panels: 
# project_panel: crates/project_panel/src/project_panel.rs
# outline_panel: crates/outline_panel/src/outline_panel.rs 
# terminal_panel: crates/terminal_view/src/terminal_panel.rs
# chat_panel: crates/collab_ui/src/chat_panel.rs
# collab_panel: crates/collab_ui/src/collab_panel.rs
# notification_panel: crates/collab_ui/src/notification_panel.rs
# debug_panel: crates/debugger_ui/src/debugger_panel.rs
# agent_panel: crates/agent/src/agent_panel.rs
# git_panel: crates/git_ui/src/git_panel.rs
# search_panel: crates/search/src/search_panel.rs
# project_search_panel: crates/search/src/buffer_search.rs


cd crates/i18n/i18n_tools
# 扫描源代码目录并生成defaults.rs文件
cargo run --package zed-i18n-tools --bin i18n-scan -- <源代码目录> [输出文件路径]

# 例如：扫描src目录并将结果输出到默认位置
cd crates/i18n/i18n_tools
cargo run --package zed-i18n-tools --bin i18n-scan -- crates/

# 或指定输出路径
cd crates/i18n/i18n_tools
cargo run --package zed-i18n-tools --bin i18n-scan -- crates/ crates/i18n/core/defaults.rs
```

这将帮助识别需要包装在翻译键中的字符串。

#### Creating a New Language Pack

要创建一个新的语言包扩展:

```bash
cd crates/i18n/i18n_tools
cargo run --package zed-i18n-tools --bin i18n-new -- i18n-fr  # 将'fr'替换为您的语言代码
```

这将在`extensions/`目录中创建一个新的扩展，结构如下:
```
extensions/i18n-fr/
├── Cargo.toml
├── README.md
├── extension.toml
├── src/
│   └── lib.rs
└── resources/
    └── translations/
        └── translation.json
```

#### Validating Translations

要验证现有的语言包:

```bash
cd crates/i18n/i18n_tools
cargo run --package zed-i18n-tools --bin i18n-validate -- extensions/i18n-fr
```

这将:
- 检查缺失的翻译
- 验证翻译键格式
- 确保所有必需的文件都存在

#### 重组翻译文件
```bash
cd crates/i18n/i18n_tools
cargo run --package zed-i18n-tools --bin i18n-reorganize -- <翻译文件路径> # 例如: cargo run --package zed-i18n-tools --bin i18n-reorganize -- resources/translations/translation.json
```

#### 处理菜单国际化

要处理菜单文件的国际化，需要分两步进行：

1. 首先扫描菜单文件并生成 defaults-app-menus.rs：
```bash
cd /Users/zhaoyu/Downloads/codding/zed && cargo run --package zed-i18n-tools --bin i18n-scan-app-menus -- scan \
  crates/zed/src/zed/app_menus.rs \
  crates/i18n/core/defaults-app-menus.rs
```

2. 检查生成的文件确保无误后，替换硬编码字符串：
```bash
cd /Users/zhaoyu/Downloads/codding/zed && cargo run --package zed-i18n-tools --bin i18n-scan-app-menus -- replace \
  crates/zed/src/zed/app_menus.rs \
  crates/i18n/core/defaults-app-menus.rs
```