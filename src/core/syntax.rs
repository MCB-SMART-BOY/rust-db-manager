use super::theme::ThemeColors;
use egui::{text::LayoutJob, Color32, FontFamily, FontId, TextFormat};

/// SQL 语法高亮器
pub struct SqlHighlighter {
    pub colors: HighlightColors,
}

/// 语法高亮颜色配置
#[derive(Clone)]
pub struct HighlightColors {
    pub keyword: Color32,     // SQL 关键字
    pub function: Color32,    // 函数
    pub string: Color32,      // 字符串
    pub number: Color32,      // 数字
    pub operator: Color32,    // 操作符
    pub comment: Color32,     // 注释
    pub identifier: Color32,  // 标识符/列名
    pub punctuation: Color32, // 标点符号
    pub default: Color32,     // 默认文本
}

impl HighlightColors {
    /// 从主题颜色创建高亮配置
    pub fn from_theme(theme: &ThemeColors) -> Self {
        Self {
            keyword: theme.accent,
            function: theme.info,
            string: theme.success,
            number: theme.warning,
            operator: theme.fg_secondary,
            comment: theme.fg_muted,
            identifier: theme.fg_primary,
            punctuation: theme.fg_secondary,
            default: theme.fg_primary,
        }
    }
}

impl Default for HighlightColors {
    fn default() -> Self {
        // Tokyo Night 风格默认颜色
        Self {
            keyword: Color32::from_rgb(122, 162, 247),     // 蓝色
            function: Color32::from_rgb(125, 207, 255),    // 青色
            string: Color32::from_rgb(158, 206, 106),      // 绿色
            number: Color32::from_rgb(255, 158, 100),      // 橙色
            operator: Color32::from_rgb(169, 177, 214),    // 灰蓝
            comment: Color32::from_rgb(86, 95, 137),       // 暗灰
            identifier: Color32::from_rgb(192, 202, 245),  // 浅蓝
            punctuation: Color32::from_rgb(169, 177, 214), // 灰蓝
            default: Color32::from_rgb(192, 202, 245),     // 浅蓝
        }
    }
}

/// SQL 关键字列表
const SQL_KEYWORDS: &[&str] = &[
    // DML
    "SELECT",
    "FROM",
    "WHERE",
    "AND",
    "OR",
    "NOT",
    "IN",
    "LIKE",
    "BETWEEN",
    "IS",
    "NULL",
    "AS",
    "ON",
    "JOIN",
    "LEFT",
    "RIGHT",
    "INNER",
    "OUTER",
    "FULL",
    "CROSS",
    "NATURAL",
    "USING",
    "ORDER",
    "BY",
    "ASC",
    "DESC",
    "LIMIT",
    "OFFSET",
    "GROUP",
    "HAVING",
    "DISTINCT",
    "ALL",
    "UNION",
    "INTERSECT",
    "EXCEPT",
    "INSERT",
    "INTO",
    "VALUES",
    "UPDATE",
    "SET",
    "DELETE",
    "TRUNCATE",
    "MERGE",
    // DDL
    "CREATE",
    "ALTER",
    "DROP",
    "TABLE",
    "INDEX",
    "VIEW",
    "DATABASE",
    "SCHEMA",
    "CONSTRAINT",
    "PRIMARY",
    "KEY",
    "FOREIGN",
    "REFERENCES",
    "UNIQUE",
    "CHECK",
    "DEFAULT",
    "AUTO_INCREMENT",
    "AUTOINCREMENT",
    "IF",
    "EXISTS",
    "CASCADE",
    "RESTRICT",
    "TEMPORARY",
    "TEMP",
    // 数据类型
    "INT",
    "INTEGER",
    "BIGINT",
    "SMALLINT",
    "TINYINT",
    "FLOAT",
    "DOUBLE",
    "DECIMAL",
    "NUMERIC",
    "REAL",
    "BOOLEAN",
    "BOOL",
    "CHAR",
    "VARCHAR",
    "TEXT",
    "BLOB",
    "DATE",
    "TIME",
    "DATETIME",
    "TIMESTAMP",
    "YEAR",
    "JSON",
    "UUID",
    "SERIAL",
    "BYTEA",
    // 其他
    "CASE",
    "WHEN",
    "THEN",
    "ELSE",
    "END",
    "CAST",
    "CONVERT",
    "COALESCE",
    "NULLIF",
    "WITH",
    "RECURSIVE",
    "RETURNING",
    "EXPLAIN",
    "ANALYZE",
    "BEGIN",
    "COMMIT",
    "ROLLBACK",
    "TRANSACTION",
    "SAVEPOINT",
    "GRANT",
    "REVOKE",
    "DENY",
    "TO",
    "ROLE",
    "USER",
    "TRUE",
    "FALSE",
];

/// SQL 内置函数
const SQL_FUNCTIONS: &[&str] = &[
    // 聚合函数
    "COUNT",
    "SUM",
    "AVG",
    "MIN",
    "MAX",
    "TOTAL",
    "GROUP_CONCAT",
    // 字符串函数
    "CONCAT",
    "SUBSTRING",
    "SUBSTR",
    "LENGTH",
    "UPPER",
    "LOWER",
    "TRIM",
    "LTRIM",
    "RTRIM",
    "REPLACE",
    "REVERSE",
    "LEFT",
    "RIGHT",
    "LPAD",
    "RPAD",
    "INSTR",
    "LOCATE",
    "POSITION",
    "CHAR_LENGTH",
    "CHARACTER_LENGTH",
    "OCTET_LENGTH",
    "BIT_LENGTH",
    "ASCII",
    "CHAR",
    "FORMAT",
    "QUOTE",
    "SOUNDEX",
    "SPACE",
    "REPEAT",
    // 数学函数
    "ABS",
    "CEIL",
    "CEILING",
    "FLOOR",
    "ROUND",
    "TRUNCATE",
    "MOD",
    "POWER",
    "POW",
    "SQRT",
    "EXP",
    "LOG",
    "LOG10",
    "LOG2",
    "LN",
    "SIGN",
    "RAND",
    "RANDOM",
    "PI",
    "SIN",
    "COS",
    "TAN",
    "ASIN",
    "ACOS",
    "ATAN",
    "ATAN2",
    "COT",
    "DEGREES",
    "RADIANS",
    // 日期时间函数
    "NOW",
    "CURRENT_DATE",
    "CURRENT_TIME",
    "CURRENT_TIMESTAMP",
    "DATE",
    "TIME",
    "DATETIME",
    "YEAR",
    "MONTH",
    "DAY",
    "HOUR",
    "MINUTE",
    "SECOND",
    "DAYOFWEEK",
    "DAYOFMONTH",
    "DAYOFYEAR",
    "WEEK",
    "WEEKDAY",
    "QUARTER",
    "EXTRACT",
    "DATE_ADD",
    "DATE_SUB",
    "DATEDIFF",
    "TIMEDIFF",
    "TIMESTAMPDIFF",
    "DATE_FORMAT",
    "TIME_FORMAT",
    "STR_TO_DATE",
    "MAKEDATE",
    "MAKETIME",
    "LAST_DAY",
    "ADDDATE",
    "SUBDATE",
    "ADDTIME",
    "SUBTIME",
    "PERIOD_ADD",
    "PERIOD_DIFF",
    // 条件函数
    "IF",
    "IFNULL",
    "NULLIF",
    "COALESCE",
    "GREATEST",
    "LEAST",
    "CASE",
    "IIF",
    "NVL",
    "NVL2",
    "DECODE",
    // 类型转换
    "CAST",
    "CONVERT",
    "TYPEOF",
    // 其他
    "ROW_NUMBER",
    "RANK",
    "DENSE_RANK",
    "NTILE",
    "LAG",
    "LEAD",
    "FIRST_VALUE",
    "LAST_VALUE",
    "NTH_VALUE",
    "OVER",
    "PARTITION",
    "ROWID",
    "OID",
    "CTID",
];

#[derive(Debug, Clone, Copy, PartialEq)]
enum TokenType {
    Keyword,
    Function,
    String,
    Number,
    Operator,
    Comment,
    Identifier,
    Punctuation,
    Whitespace,
}

struct Token {
    text: String,
    token_type: TokenType,
}

impl SqlHighlighter {
    pub fn new(colors: HighlightColors) -> Self {
        Self { colors }
    }

    /// 创建带语法高亮的 LayoutJob
    pub fn highlight(&self, text: &str) -> LayoutJob {
        let mut job = LayoutJob::default();
        let tokens = self.tokenize(text);
        let font_id = FontId::new(14.0, FontFamily::Monospace);

        for token in tokens {
            let color = self.get_token_color(&token.token_type);
            job.append(
                &token.text,
                0.0,
                TextFormat {
                    font_id: font_id.clone(),
                    color,
                    ..Default::default()
                },
            );
        }

        job
    }

    fn get_token_color(&self, token_type: &TokenType) -> Color32 {
        match token_type {
            TokenType::Keyword => self.colors.keyword,
            TokenType::Function => self.colors.function,
            TokenType::String => self.colors.string,
            TokenType::Number => self.colors.number,
            TokenType::Operator => self.colors.operator,
            TokenType::Comment => self.colors.comment,
            TokenType::Identifier => self.colors.identifier,
            TokenType::Punctuation => self.colors.punctuation,
            TokenType::Whitespace => self.colors.default,
        }
    }

    fn tokenize(&self, text: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let chars: Vec<char> = text.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let c = chars[i];

            // 空白字符
            if c.is_whitespace() {
                let start = i;
                while i < chars.len() && chars[i].is_whitespace() {
                    i += 1;
                }
                tokens.push(Token {
                    text: chars[start..i].iter().collect(),
                    token_type: TokenType::Whitespace,
                });
                continue;
            }

            // 单行注释 --
            if c == '-' && i + 1 < chars.len() && chars[i + 1] == '-' {
                let start = i;
                while i < chars.len() && chars[i] != '\n' {
                    i += 1;
                }
                tokens.push(Token {
                    text: chars[start..i].iter().collect(),
                    token_type: TokenType::Comment,
                });
                continue;
            }

            // 多行注释 /* */
            if c == '/' && i + 1 < chars.len() && chars[i + 1] == '*' {
                let start = i;
                i += 2;
                while i + 1 < chars.len() && !(chars[i] == '*' && chars[i + 1] == '/') {
                    i += 1;
                }
                if i + 1 < chars.len() {
                    i += 2;
                }
                tokens.push(Token {
                    text: chars[start..i].iter().collect(),
                    token_type: TokenType::Comment,
                });
                continue;
            }

            // 字符串 (单引号)
            if c == '\'' {
                let start = i;
                i += 1;
                while i < chars.len() {
                    if chars[i] == '\'' {
                        if i + 1 < chars.len() && chars[i + 1] == '\'' {
                            // 转义的单引号
                            i += 2;
                        } else {
                            i += 1;
                            break;
                        }
                    } else {
                        i += 1;
                    }
                }
                tokens.push(Token {
                    text: chars[start..i].iter().collect(),
                    token_type: TokenType::String,
                });
                continue;
            }

            // 字符串 (双引号 - 用于标识符)
            if c == '"' {
                let start = i;
                i += 1;
                while i < chars.len() && chars[i] != '"' {
                    i += 1;
                }
                if i < chars.len() {
                    i += 1;
                }
                tokens.push(Token {
                    text: chars[start..i].iter().collect(),
                    token_type: TokenType::Identifier,
                });
                continue;
            }

            // 反引号标识符 (MySQL)
            if c == '`' {
                let start = i;
                i += 1;
                while i < chars.len() && chars[i] != '`' {
                    i += 1;
                }
                if i < chars.len() {
                    i += 1;
                }
                tokens.push(Token {
                    text: chars[start..i].iter().collect(),
                    token_type: TokenType::Identifier,
                });
                continue;
            }

            // 数字
            if c.is_ascii_digit()
                || (c == '.' && i + 1 < chars.len() && chars[i + 1].is_ascii_digit())
            {
                let start = i;
                let mut has_dot = c == '.';
                i += 1;
                while i < chars.len() {
                    let ch = chars[i];
                    if ch.is_ascii_digit() {
                        i += 1;
                    } else if ch == '.' && !has_dot {
                        has_dot = true;
                        i += 1;
                    } else if ch == 'e' || ch == 'E' {
                        i += 1;
                        if i < chars.len() && (chars[i] == '+' || chars[i] == '-') {
                            i += 1;
                        }
                    } else {
                        break;
                    }
                }
                tokens.push(Token {
                    text: chars[start..i].iter().collect(),
                    token_type: TokenType::Number,
                });
                continue;
            }

            // 标识符或关键字
            if c.is_alphabetic() || c == '_' {
                let start = i;
                while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                let word: String = chars[start..i].iter().collect();
                let upper = word.to_uppercase();

                let token_type = if SQL_KEYWORDS.contains(&upper.as_str()) {
                    TokenType::Keyword
                } else if SQL_FUNCTIONS.contains(&upper.as_str()) {
                    TokenType::Function
                } else {
                    TokenType::Identifier
                };

                tokens.push(Token {
                    text: word,
                    token_type,
                });
                continue;
            }

            // 操作符
            if "+-*/%=<>!&|^~".contains(c) {
                let start = i;
                i += 1;
                // 处理多字符操作符
                if i < chars.len() {
                    let next = chars[i];
                    if (c == '<' && (next == '=' || next == '>' || next == '<'))
                        || (c == '>' && (next == '=' || next == '>'))
                        || (c == '!' && next == '=')
                        || (c == '|' && next == '|')
                        || (c == '&' && next == '&')
                        || (c == ':' && next == ':')
                    {
                        i += 1;
                    }
                }
                tokens.push(Token {
                    text: chars[start..i].iter().collect(),
                    token_type: TokenType::Operator,
                });
                continue;
            }

            // 标点符号
            if "(),;.:[]{}".contains(c) {
                tokens.push(Token {
                    text: c.to_string(),
                    token_type: TokenType::Punctuation,
                });
                i += 1;
                continue;
            }

            // 其他字符
            tokens.push(Token {
                text: c.to_string(),
                token_type: TokenType::Identifier,
            });
            i += 1;
        }

        tokens
    }
}

/// 用于 egui TextEdit 的语法高亮功能
pub fn highlight_sql(text: &str, colors: &HighlightColors) -> LayoutJob {
    let highlighter = SqlHighlighter::new(colors.clone());
    highlighter.highlight(text)
}
