from pathfinder import *
import time
from math import sin, cos, pi


def indicator(len, width):
    p = Path()
    p.move_to((0, width/2))
    p.arc((0,0), width/2, pi/2, -pi/2, True)
    p.line_to((len, 0))
    p.close()
    return p

def circle(r):
    p = Path()
    p.circle((0, 0), r)
    p.close()
    return p

def tick(len, thickness):
    p = Path()
    p.rect(Rect((0, -thickness/2), (len, thickness/2)))
    return p

color_hour = Color.rgba(0.5, 0, 0.8, 1.0)
color_minute = Color.rgba(0.6, 0, 0.6, 1.0)
color_second = Color.rgba(0.7, 0, 0.5, 1.0)



def clock():
    ctx = Canvas((200, 200))
    ctx.translate((100, 100))
    ctx.fill_style = Color.rgba(0.9, 0.85, 0.8, 0.75)
    ctx.stroke_style = Color.rgba(0,0,0,1)
    
    c = circle(80)
    ctx.fill_path(c, "winding")
    ctx.stroke_path(c)

    ctx.fill_style = Color.rgba(0,0,0,1)
    for h in range(12):
        ctx.save()
        ctx.rotate(h / 6 * pi)
        ctx.translate((-80, 0))
        ctx.fill_path(tick(10, 2), "winding")
        ctx.restore()
    
    (tm_year,tm_mon,tm_mday,tm_hour,tm_min,tm_sec,tm_wday,tm_yday,tm_isdst) = time.localtime()
    hands = [
        (tm_hour * pi / 6, 50, 10, color_hour),
        (tm_min * pi / 30, 60, 8, color_minute),
        (tm_sec * pi / 30, 70, 5, color_second)
    ]
    for (angle, len, width, color) in hands:
        ctx.save()
        ctx.fill_style = color
        ctx.rotate(angle - pi/2)
        ctx.fill_path(indicator(len, width), "winding")
        ctx.restore()

    return ctx

w = show(clock(), True, True) #, "d3d11", bg)
while True:
    w.update(clock())
    time.sleep(1)
