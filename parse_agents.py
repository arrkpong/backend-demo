from html.parser import HTMLParser
from pathlib import Path

class TextExtractor(HTMLParser):
    def __init__(self):
        super().__init__()
        self.parts = []

    def handle_data(self, data):
        text = data.strip()
        if text:
            self.parts.append(text)

path = Path('agents.html')
parser = TextExtractor()
parser.feed(path.read_text(encoding='utf-8'))
print('\n'.join(parser.parts[:400]))
