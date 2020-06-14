import pathfinder
from pathfinder import *
import time
from math import sin, cos, pi


def indicator(len, width):
    p = pathfinder.Path()
    p.arc((0,0), width/2, pi/2, -pi/2, True)
    p.line_to((len, 0))
    p.close()
    return p

def indicator2(base_r, peak_x, peak_y, length):
    p = Path()
    p.arc((0, 0), base_r, pi, pi/2, False)
    p.bezier_curve_to((base_r, base_r), (peak_x - base_r, 0), (peak_x, peak_y))
    p.bezier_curve_to((peak_x + base_r, 0), (peak_x + base_r, peak_y/2), (length, 0))
    p.mirror_and_close_last()
    return p

def circle(r):
    p = Path()
    p.circle((0, 0), r)
    p.close()
    return p

def tick(len, thickness):
    p = Path()
    p.rect(Rect((0, -thickness/2), (len, thickness)))
    return p

color_hour = Color.rgba(0.5, 0, 0.8, 1.0)
color_minute = Color.rgba(0.6, 0, 0.6, 1.0)
color_second = Color.rgba(0.7, 0, 0.5, 1.0)

font = FontCollection.from_fonts([Font.from_file("/usr/share/fonts/truetype/noto/NotoSerif-Regular.ttf")])


def clock():
    ctx = Canvas((40, 40))
    ctx.font = font
    ctx.font_size = 4
    ctx.translate((20, 20))
    ctx.fill_style = Color.rgba(0.9, 0.85, 0.8, 0.75)
    ctx.stroke_style = Color.rgba(0,0,0,1)
    ctx.line_width = 0.5
    ctx.text_align = "center"

    c = circle(19)
    ctx.fill_path(c, "winding")
    ctx.stroke_path(c)

    ctx.fill_style = Color.rgba(0,0,0,1)
    for h in range(12):
        ctx.save()
        ctx.rotate(h / 6 * pi)
        ctx.translate((-19, 0))
        ctx.fill_path(tick(2, 0.2), "winding")
        ctx.restore()

    ctx.fill_text("Hello Pathfinder", (0, -5))

    (tm_year,tm_mon,tm_mday,tm_hour,tm_min,tm_sec,tm_wday,tm_yday,tm_isdst) = time.localtime()

    ctx.font_size = 6
    ctx.text_align = "right"
    ctx.fill_text("{:02}".format(tm_hour), (-1, 10))
    ctx.text_align = "left"
    ctx.fill_text("{:02}".format(tm_min), (1, 10))
    if tm_sec % 2 == 0:
        ctx.text_align = "center"
        ctx.fill_text(":", (0, 10))

    ctx.line_width = 0.2
    seconds = tm_sec
    minutes = tm_min + seconds / 60
    hours = tm_hour + minutes / 60
    
    hands = [
        (hours * pi / 6, 14, 3, color_hour),
        (minutes * pi / 30, 16, 2.5, color_minute),
        (seconds * pi / 30, 18, 2, color_second)
    ]
    for (angle, len, width, color) in hands:
        ctx.save()
        ctx.fill_style = color
        ctx.rotate(angle - pi/2)
        p = indicator2(width, 0.4*len, width/2, len)
        ctx.fill_path(p, "winding")
        ctx.restore()
    

    return ctx

w = show(clock(), True, False, True, False, Color.rgba(0,0,0,0)) #, "d3d11", bg)
while True:
    w.update(clock())
    time.sleep(1)

