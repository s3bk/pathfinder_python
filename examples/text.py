from pathfinder import *
import time
from math import sin, cos, pi


ctx = Canvas((100, 100))
ctx.translate((20, 20))
ctx.fill_style = Color.rgba(0.9, 0.85, 0.8, 0.75)
ctx.stroke_style = Color.rgba(0.9, 0.85, 0.8, 0.75)
ctx = Canvas((40, 40))
ctx.font = Font.from_file("/usr/share/fonts/truetype/noto/NotoSerif-Regular.ttf")
ctx.font_size = 10
ctx.fill_text("Hello world!", (5, 5))

p = Path()
p.move_to((0, 0))
p.line_to((20, 00))
p.move_to((0, 0))
p.line_to((0, -20))
ctx.stroke_path(p)

w = show(ctx, zoom=True, pan=True)

input()
