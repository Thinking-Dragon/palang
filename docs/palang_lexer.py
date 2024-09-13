from pygments.lexer import RegexLexer, words, bygroups
from pygments.token import Keyword, String, Comment, Name, Text, Punctuation

class PalangLexer(RegexLexer):
    name = 'Palang'
    aliases = ['palang']
    filenames = ['*.palang']

    keywords = ('module', 'model', 'prompt', 'function', 'return', 'if', 'for', 'in', 'rag')

    tokens = {
        'root': [
            (words(keywords, prefix=r'\b', suffix=r'\b'), Keyword),
            # (r'"', String, 'string'),
            (r'//.*$', Comment.Single),
            (r'\b[a-zA-Z_][a-zA-Z0-9_]*\s*(?=\()', Name.Function),
            (r'\b[a-zA-Z_][a-zA-Z0-9_]*\b', Name.Variable),
            (r'(prompt\s+\w*\s*\([^)]*\)\s*->\s*[^{]*?)(\{)',
             bygroups(Keyword, Punctuation), 'prompt-body'),
            (r'(model\s+\w*\s*)(\{)',
             bygroups(Keyword, Punctuation), 'model-body'),
            (r'\s+', Text),
            (r'[{}()]', Punctuation),
        ],
        'string': [
            (r'\\[\\"]', String.Escape),
            (r'"', String, '#pop'),
            (r'[^\\"\n]+', String),
        ],
        'prompt-body': [
            (r'\}', Punctuation, '#pop'),
            (r'[^}]+', String),
            (r'\}', String),
        ],
        'model-body': [
            (r'\}', Punctuation, '#pop'),
            (r'[^}]+', String),
            (r'\}', String),
        ],
    }
