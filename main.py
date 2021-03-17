import sys
import os
from tkinter import *


yjk_path = sys.argv[1]


def click2():
    yjk_exe = r"C:\YJKS\2.0.3\yjks.exe"
    run(yjk_exe)
    pass


def click3():
    yjk_exe = r"C:\YJKS\3.0.2\YJKS_3_0\yjks.exe"
    run(yjk_exe)
    pass


def run(yjk_exe):
    os.system(f"start {yjk_exe} {yjk_path}")
    root.destroy()


root = Tk()

# root config
root.title("YJK")
screen_w = root.winfo_screenwidth()
screen_h = root.winfo_screenheight()
win_w = 200
win_h = 140
size = f"{win_w}x{win_h}+{int((screen_w-win_w)/2)}+{int(screen_h/2-win_h)}"
root.geometry(size)

# button
btn_height = 2
btn_yjk2 = Button(root,
                  text="2.0.3",
                  height=btn_height,
                  command=click2)

btn_yjk3 = Button(root,
                  text="3.0.2",
                  height=btn_height,
                  command=click3)

# layout
btn_yjk2.pack(fill=X, padx=50,pady=10)
btn_yjk3.pack(fill=X, padx=50,pady=10)

# run
root.mainloop()
