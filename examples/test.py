from pathfinder import *
import time
from math import sin, cos, pi

rgba = Color.rgba

s = Scene()
s.view_box = Rect((0, 0), (100, 100))

bg = rgba(0.5, 1.0, 1.0, 1.0)

w = show(s, background=bg) #, "d3d11", bg)

t0 = time.time()

while True:
    t = time.time() - t0

    b = Path()
    b.rect(Rect((0.0, 0.0), (100., 100.)))
    s.draw(b, bg)

    p = Path()
    p.move_to((10, 20))
    p.line_to((40 + 20 * cos(t), 50 + 20 * sin(t)))
    p.line_to((80, 40))
    p.close()

    s.draw(p, rgba(1.0, 0.0, 0.0, 1.0))
    w.update(s)

    time.sleep(0.02)

