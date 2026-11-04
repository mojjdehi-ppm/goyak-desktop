"""Inject the data bundle into template.html -> ritm-61.html.  Usage: python3 -I build_page.py data.json"""
import sys, pathlib
here = pathlib.Path(__file__).parent
data = pathlib.Path(sys.argv[1]).read_text(encoding='utf-8').replace('</', '<\\/')
page = (here / 'template.html').read_text(encoding='utf-8').replace('/*__DATA__*/null', data)
(here / 'ritm-61.html').write_text(page, encoding='utf-8')
print('wrote', here / 'ritm-61.html', len(page) // 1024, 'KB')
