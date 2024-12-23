import sys
import re
import argparse

FRAME_PER_SECOND = 30
MS_PER_SECOND = 1000
SECOND_PER_MINUTE = 60
MINUTE_PER_HOUR = 60

def is_strh(strh):
    # 0123456789abc
    # hh:mm:ss.mss
    strh = str(strh)
    if len(strh) != 12:
        return False
    return re.search("^[0-9][0-9]+:[0-9][0-9]:[0-9][0-9]\\.[0-9][0-9][0-9]$", strh) is not None

def is_frame(frame):
    return re.search("^-?[0-9]+$", str(frame)) is not None

def frm_to_strh(frames):
    negative = frames < 0
    if negative:
        frames = -frames
    millisecond_frames = frames % FRAME_PER_SECOND
    seconds = (frames - millisecond_frames) / FRAME_PER_SECOND
    minute_seconds = seconds % SECOND_PER_MINUTE
    minutes = (seconds - minute_seconds) / SECOND_PER_MINUTE
    hour_minutes = minutes % MINUTE_PER_HOUR
    hours = (minutes - hour_minutes) / MINUTE_PER_HOUR

    rest_millisecond = millisecond_frames % (FRAME_PER_SECOND/10)
    hundred_millisecond = (millisecond_frames - rest_millisecond) / (FRAME_PER_SECOND/10)

    if rest_millisecond == 0:
        rest_millisecond = 0
    elif rest_millisecond == 1:
        rest_millisecond = 33
    else:
        rest_millisecond = 67

    return format_strh(negative, hours, hour_minutes, minute_seconds, hundred_millisecond*100+rest_millisecond)

def strh_to_frm(strh):
    # 0123456789abc
    # hh:mm:ss.mss
    hours = int(strh[0:2])
    minutes = int(strh[3:5])
    seconds = int(strh[6:8])
    ms_hundred = int(strh[9:10])
    ms_rest = int(strh[10:])
    if ms_rest > 60:
        frames = 2
    elif ms_rest>30:
        frames = 1
    else:
        frames = 0
    frames+=ms_hundred*(FRAME_PER_SECOND/10)
    frames+=seconds*FRAME_PER_SECOND
    frames+=minutes*SECOND_PER_MINUTE*FRAME_PER_SECOND
    frames+=hours*MINUTE_PER_HOUR*SECOND_PER_MINUTE*FRAME_PER_SECOND
    return int(frames)

def ms_to_strh(ms):
    negative = ms < 0
    if negative:
        ms = -ms
    millisecond = ms % MS_PER_SECOND
    seconds = (ms - millisecond) / MS_PER_SECOND
    minute_seconds = seconds % SECOND_PER_MINUTE
    minutes = (seconds - minute_seconds) / SECOND_PER_MINUTE
    hour_minutes = minutes % MINUTE_PER_HOUR
    hours = (minutes - hour_minutes) / MINUTE_PER_HOUR

    return format_strh(negative, hours, hour_minutes, minute_seconds, millisecond)

def strh_to_ms(strh):
    # 0123456789abc
    # hh:mm:ss.mss
    hours = int(strh[0:2])
    minutes = int(strh[3:5])
    seconds = int(strh[6:8])
    ms = int(strh[9:])

    ms+=seconds*MS_PER_SECOND
    ms+=minutes*SECOND_PER_MINUTE*MS_PER_SECOND
    ms+=hours*MINUTE_PER_HOUR*SECOND_PER_MINUTE*MS_PER_SECOND
    return int(ms)

def format_strh(negative, hours, minutes, seconds, ms):
    negative_string = "-" if negative else ""
    second_string = f"{int(seconds)}"
    if seconds < 10:
        second_string = "0" + second_string

    minute_string = f"{int(minutes)}"
    if minutes < 10:
        minute_string = "0" + minute_string

    hour_string = f"{int(hours)}:"
    if hours < 10:
        hour_string = "0" + hour_string

    if ms < 10:
        ms_string = f".00{int(ms)}"
    elif ms < 100:
        ms_string = f".0{int(ms)}"
    else:
        ms_string = f".{int(ms)}"

    return f"{negative_string}{hour_string}{minute_string}:{second_string}{ms_string}"
MODE_CONVERT = 0
MODE_ADD = 1
MODE_SUBTRACT = 2

def run():
    parser = argparse.ArgumentParser(description='Process some times.')
    parser.add_argument('-a', action='store_true', help="Add the arguments")
    parser.add_argument('-s', action='store_true', help="Subtract the arguments")
    parser.add_argument('-c', action='store_true', help="Convert the arguments (default)")
    parser.add_argument('-f', action='store_true', help="Output frames (30 fps)")
    parser.add_argument('-t', action='store_true', help="Output time code")
    parser.add_argument('-m', action='store_true', help="Millisecond mode. Input numbers are treated as ms and will not align to 30 fps")

    parser.add_argument('times', metavar='T', type=str, nargs='+', help='time input, can be frames or time code (hh:mm:ss.mss)')

    args = parser.parse_args()
    mode = MODE_CONVERT
    if args.c:
        mode = MODE_CONVERT
    elif args.a:
        mode = MODE_ADD
    elif args.s:
        mode = MODE_SUBTRACT

    ms_mode = args.m
    
    storage = []
    for t in args.times:
        if is_strh(t):
            if ms_mode:
                current = strh_to_ms(t)
            else:
                current = strh_to_frm(t)
        elif is_frame(t):
            current = int(t) 
        else:
            print(f"Error: not recognized: {t}")
            sys.exit(1)
        if len(storage) == 0 or mode == MODE_CONVERT:
            storage.append(current)
        elif mode == MODE_ADD:
            storage[0] = storage[0] + current
        elif mode == MODE_SUBTRACT:
            storage[0] = storage[0] - current
    
    if args.t:
        for s in storage:
            if ms_mode:
                print(ms_to_strh(s))
            else:
                print(frm_to_strh(s))
    else:
        for s in storage:
            print(s)


if __name__ == "__main__":
    run()

