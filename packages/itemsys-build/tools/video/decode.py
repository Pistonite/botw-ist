"""
    Step 2: Decode video into frames
    Usage: python decode.py <object>...

    Objects are defined in Animate.yaml. * means all, foo* means all prefixed with foo

    This will decode the video and crop the item in the row and col (0-based)

    Outputs are in target/decode/<object>/frame_X.png

    Check the quality of the frames before preceeding
"""


import cv2
import os
import sys
import shutil
import yaml
import multiprocessing
import json

def get_home():
    return os.path.dirname(os.path.dirname(os.path.dirname(__file__)))

def load_config():
    config = os.path.join(get_home(), "Animate.yaml")
    with open(config, "r", encoding="utf-8") as f:
        return yaml.safe_load(f)

def main():
    objects = sys.argv[1:]
    expanded_objects = set()

    home = get_home()

    config = load_config()
    if not objects:
        objects = ["*"]
    for obj in objects:
        if obj.endswith("*"):
            prefix = obj[:-1]
            for key in config["objects"]:
                profile, name = key.split(":", 1)
                if not prefix or name.startswith(prefix):
                    expanded_objects.add((profile, name))
        else:
            for key in config["objects"]:
                profile, name = key.split(":", 1)
                if name == obj or profile == obj:
                    expanded_objects.add((profile, name))
    
    args = []
    for profile, name in expanded_objects:
        object_config = config["objects"][profile+":"+name]
        video_file = object_config["file"]
        [row, col] = object_config["row_col"]
        profile_data = config["decoder"]["profiles"][profile]
        top_expand = profile_data["top_expand"]
        expected_frame_count = profile_data["frames"]
        erase_count = profile_data["erase_count"]
        side_expand = 0
        if "side_expand" in profile_data:
            side_expand = profile_data["side_expand"]
        top_clean = 0
        if "top_clean" in profile_data:
            top_clean = profile_data["top_clean"]
        top_clean_frame = -1
        if "top_clean_frame" in object_config:
            top_clean_frame = object_config["top_clean_frame"]
        args.append((config, video_file, profile, name, row, col, top_expand, expected_frame_count, erase_count, side_expand, top_clean, top_clean_frame))

    frame_args = []
    for config, _, _, name, _, _, top_expand, frame_count, erase_count, side_expand, top_clean, _ in args:
        output_dir = os.path.join(home, "target", "decode", name)
        for i in range(frame_count):
            frame_path = os.path.join(output_dir, f"frame_{i}.png")
            frame_args.append((config, frame_path, i, name, erase_count, top_expand, side_expand, top_clean))

    with multiprocessing.Pool() as pool:
        # python is pretty limited and it's not worth to optimize here
        # just decode frames first, then fix frames
        for _ in pool.imap_unordered(decode_frames_shim, args):
            pass
        for _ in pool.imap_unordered(fix_frame_shim, frame_args):
            pass

    print("all done")


def decode_frames_shim(args):
    config, video_file, profile, object_name, row, col, top_expand, expected_frame_count, _erase_count, side_expand, top_clean, top_clean_frame = args
    decode_frames(config, video_file, profile, object_name, row, col, top_expand, expected_frame_count, side_expand, top_clean, top_clean_frame)
def decode_frames(config, video_file, profile, object_name, row, col, top_expand, expected_frame_count, side_expand, top_clean, top_clean_frame):
    print(f"[{video_file}] decoding frames (row={row}, col={col})")
    home = get_home()
    cap = cv2.VideoCapture(os.path.join(home, video_file))
    frame_count = 0
    # Measurements
        
    decoder = config["decoder"]
    measurements = decoder["measurements"]
    profile = decoder["profiles"][profile]
    BORDER_TO_FIRST_INNER_LEFT = measurements["border_to_first_inner_left"]
    BORDER_TO_FIRST_INNER_TOP = measurements["border_to_first_inner_top"]
    CELL_OFFSET = measurements["cell_offset"]
    BASE_SIZE = measurements["base_size"]
    TOP_EXPAND = top_expand

    crop_x = BORDER_TO_FIRST_INNER_LEFT + col * CELL_OFFSET - side_expand
    crop_y = BORDER_TO_FIRST_INNER_TOP + row * CELL_OFFSET - TOP_EXPAND
    crop_x_end = crop_x + BASE_SIZE + side_expand * 2
    crop_y_end = crop_y + BASE_SIZE + TOP_EXPAND

    output_dir = os.path.join(home, "target", "decode", object_name)
    if os.path.exists(output_dir):
        shutil.rmtree(output_dir)
    os.makedirs(output_dir, exist_ok=True)
    try:
        while True:
            ok, frame = cap.read()
            if not ok:
                break

            frame_filename = os.path.join(output_dir, f"frame_{frame_count}.png")
            cropped = frame[crop_y:crop_y_end, crop_x:crop_x_end]
            if frame_count % 30 == 0:
                print(f"[{video_file}] saving frame {frame_count}")
            cv2.imwrite(frame_filename, cropped)

            if frame_count == top_clean_frame:
                # extract clean frame data (y,x)[]
                # these are pixels that are not black but need to be set black
                clean_frame_data = []
                BACKGROUND_THRESHOLD = measurements["background_threshold"]
                _, width, _ = cropped.shape
                for y in range(top_clean):
                    for x in range(width):
                        (r,g,b) = getrgb(cropped, y, x)
                        if r >= BACKGROUND_THRESHOLD or g >= BACKGROUND_THRESHOLD or b >= BACKGROUND_THRESHOLD:
                            clean_frame_data.append((y,x))
                clean_filename = os.path.join(output_dir, f"frame_clean.json")
                with open(clean_filename, "w", encoding="utf-8") as clean_file:
                    json.dump(clean_frame_data, clean_file)
                print(f"[{video_file}] saved clean frame data")

            frame_count += 1
    except Exception as e:
        cap.release()
        raise e

    print(f"[{video_file}] saved {frame_count} frames")

    if frame_count != expected_frame_count:
        raise ValueError(f"[{video_file}] expected {expected_frame_count} frames, got {frame_count} frames")
    
def fix_frame_shim(args):
    config, frame_path, ithframe, object_name, erase_count, top_expand, side_expand, top_clean = args
    fix_frame(config, frame_path, ithframe, object_name, erase_count, top_expand, side_expand, top_clean)
def fix_frame(config, frame_path, ithframe, object_name, erase_count, top_expand, side_expand, top_clean):
    MEASUREMENTS = config["decoder"]["measurements"]
    BACKGROUND_THRESHOLD = MEASUREMENTS["background_threshold"]

    frame = cv2.imread(frame_path)

    def fix_top_clean(frame):
        clean_filename = os.path.join(os.path.dirname(frame_path), "frame_clean.json")
        with open(clean_filename, "r", encoding="utf-8") as clean_file:
            clean_data = json.load(clean_file)
        for y, x in clean_data:
            setblack(frame, y, x)

    def fix_black(frame):
        """Clean up the black pixels"""
        height, width, _ = frame.shape
        for y in range(height):
            for x in range(width):
                (r,g,b) = getrgb(frame, y ,x)
                if is_black(r, g, b):
                    setblack(frame, y, x)
        BORDER_THRESHOLD = MEASUREMENTS["border_erase_threshold"]
        for y in range(top_expand):
            for x in range(width):
                (r,g,b) = getrgb(frame, y ,x)
                if b < BORDER_THRESHOLD and g < BORDER_THRESHOLD and r < BORDER_THRESHOLD:
                    setblack(frame, y, x)
                else:
                    break
            for x in range(width):
                x = width - x - 1
                (r,g,b) = getrgb(frame, y ,x)
                if b < BORDER_THRESHOLD and g < BORDER_THRESHOLD and r < BORDER_THRESHOLD:
                    setblack(frame, y, x)
                else:
                    break
        if side_expand:
            # vertical borders on the sides
            for y in range(height):
                for x in range(side_expand):
                    (r,g,b) = getrgb(frame, y ,x)
                    if b < BORDER_THRESHOLD and g < BORDER_THRESHOLD and r < BORDER_THRESHOLD:
                        setblack(frame, y, x)
                    (r,g,b) = getrgb(frame, y ,width - x - 1)
                    if b < BORDER_THRESHOLD and g < BORDER_THRESHOLD and r < BORDER_THRESHOLD:
                        setblack(frame, y, width - x - 1)


    def fix_orb_count(frame):
        """Cleans up single digit orb count in the lower left corner"""
        height, _, _ = frame.shape
        # Delete the "x"
        ERASE_X_WIDTH = MEASUREMENTS["orb_erase_x"]["width"]
        ERASE_X_HEIGHT = MEASUREMENTS["orb_erase_x"]["height"]
        for y in range(height - ERASE_X_HEIGHT, height):
            for x in range(0, ERASE_X_WIDTH):
                setblack(frame, y, x)

        # Delete the digit
        ERASE_DIGIT = MEASUREMENTS["orb_erase_digit"]
        ERASE_DIGIT_LEFT = ERASE_DIGIT["left"]
        ERASE_DIGIT_BOTTOM = ERASE_DIGIT["bottom"]
        ERASE_DIGIT_WIDTH = ERASE_DIGIT["width"]
        ERASE_DIGIT_HEIGHT = ERASE_DIGIT["height"]

        # Dim the digit until it's not visible
        pixels = erase_digit(
            frame,
            height - ERASE_DIGIT_BOTTOM,
            height - ERASE_DIGIT_BOTTOM + ERASE_DIGIT_HEIGHT,
            ERASE_DIGIT_LEFT,
            ERASE_DIGIT_LEFT + ERASE_DIGIT_WIDTH,
            1,
            72,
        )

        need_fix_pixels = set()
        fix_window = 1
        for (y, x) in pixels:
            for yy in range(y-fix_window, y+fix_window+1):
                for xx in range(x-fix_window, x+fix_window+1):
                    (r,g,b) = getrgb(frame, yy, xx)
                    if not is_black(r,g,b):
                        need_fix_pixels.add((yy,xx))

        FIXUP_X = ERASE_DIGIT["fixup_left"]
        FIXUP_Y = height - ERASE_DIGIT["fixup_bottom"]
        (fix_r, fix_g, fix_b) = getrgb(frame, FIXUP_Y, FIXUP_X)
        FIX_WEIGHT = ERASE_DIGIT["fixup_weight"]
        def add_fix(ct, tt, fix_c):
            return round(float(ct // tt) * (1 - FIX_WEIGHT) + float(fix_c) * FIX_WEIGHT)
        ITERATION = ERASE_DIGIT["fixup_iteration"]
        FIX_CONTEXT = ERASE_DIGIT["fixup_context"]
        for _ in range(ITERATION):
            new_pixels = [] # y, x, r, g, b
            for (y, x) in need_fix_pixels:
                (r, g, b) = getrgb(frame, yy, xx)
                rt = 0
                gt = 0
                bt = 0
                tt = 0
                for yy in range(y-FIX_CONTEXT, y+FIX_CONTEXT+1):
                    for xx in range(x-FIX_CONTEXT,x+FIX_CONTEXT+1):
                        (r, g, b) = getrgb(frame, yy, xx)
                        rt += int(r)
                        gt += int(g)
                        bt += int(b)
                        tt += 1
                
                r = add_fix(rt, tt, fix_r)
                g = add_fix(gt, tt, fix_g)
                b = add_fix(bt, tt, fix_b)
                new_pixels.append((y, x, r, g, b))
            for (y, x, r,g,b) in new_pixels:
                setrgb(frame, y , x,r,g,b)


    def erase_digit(frame, min_y, max_y, min_x, max_x, context, thres):
        """
            Fix pixels in the bound according to luminence
            if dimming = True, pixels will be less bright
            otherwise will be more bright
            black pixels are ignored when un-dimming
        """
        touched_pixels = set()
        while True:
            # (y, x)
            pixels = []
            for y in range(min_y, max_y):
                for x in range(min_x, max_x):
                    (r, g, b) = getrgb(frame, y, x)
                    l = luminence(r, g, b)
                    if l > thres:
                        pixels.append((y, x))
            if not pixels:
                break
            # for each pixel, take the average of its non-matched neighbors-
            # (y, x, r, g, b)
            new_pixels = []
            for (y, x) in pixels:
                # non-black lo lumi
                r_tot = 0
                g_tot = 0
                b_tot = 0
                count = 0
                all_count = 0 # non-black low lumi
                all_black = True # black + non-black low lumi
                
                for yy in range(y-context,y+context+1):
                    for xx in range(x-context,x+context+1):
                        #if yy == y and xx == x:
                        #  continue
                        (r, g, b) = getrgb(frame, yy, xx)
                        l = luminence(r, g, b)
                        if l <= thres:
                            all_count += 1
                            if is_black(r, g, b):
                                continue
                            count += 1
                            all_black = False
                            r_tot += int(r)
                            g_tot += int(g)
                            b_tot += int(b)

                if not all_count:
                    # pixel is surrounded by other pixels
                    continue
                if all_black:
                    # turn pixel into black as well
                    new_pixels.append((y, x, 0, 0, 0))
                    continue
                        
                if count == 0:
                    raise Exception("should not happen")

                r = round(r_tot / count)
                g = round(g_tot / count)
                b = round(b_tot / count)
                new_pixels.append((y, x, r, g, b))
            # apply edits
            if not new_pixels:
                break

            for (y, x, r, g, b) in new_pixels:
                touched_pixels.add((y,x ))
                setrgb(frame, y, x, r, g, b)

        return touched_pixels

    def luminence(r, g, b):
        return 0.299 * r + 0.587 * g + 0.114 * b
    
    def setrgb(frame, y, x, r, g, b):
        if is_black(r,g,b):
            setblack(frame,y,x)
        else:
            frame[y,x]=[b,g,r]

    def is_black(r, g, b):
        return r < BACKGROUND_THRESHOLD and g < BACKGROUND_THRESHOLD and b < BACKGROUND_THRESHOLD


    def setblack(frame, y, x):
        frame[y,x]=[0,0,0]

    if top_clean:
        fix_top_clean(frame)
    if erase_count:
        fix_orb_count(frame)
    fix_black(frame)

    cv2.imwrite(frame_path, frame)
    if ithframe % 30 == 0:
        print(f"[{object_name}] fixed-up frame {ithframe}")

def getrgb(frame, y, x):
    (b,g,r) = frame[y, x]
    return (r,g,b)



if __name__ == "__main__":
    main()    

