from setuptools import setup, find_packages

setup(
    name='palang-pygments',
    version='0.1',
    packages=find_packages(),
    entry_points={
        'pygments.lexers': [
            'palang = palang_lexer:PalangLexer',
        ],
    },
)
