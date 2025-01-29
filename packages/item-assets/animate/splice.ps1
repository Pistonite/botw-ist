# Preparation:
# Get the items, go to Ketoh Wawai and go to a spot where it's completely dark, look up, open inventory
# Make sure the cursor is not on or next to any item you want to extract
# Use OBS to record ~1 min video to work with. I have 6000 Kbps
# Then, use a tool like avidemux or mpv to find the timecode to splice from the video
#
# Step 1: Splice 
#     ./splice.ps1 <input> <from_time> <to_time> <output>
# Re-encode the video so each frame correspond to one frame of the animation
# Needs mpv and ffmpeg, args for Nvidia GPU only
# Time code formats are HH:MM:SS.mmm

$file = $args[0]
$from = $args[1]
$to = $args[2]
$temp = "temp.mp4"
$output = $args[3]

$from = python tc.py -mt $from
$to = python tc.py -mt $to
while($true) {
    $vframes = python tc.py -s $to $from
    $duration = python tc.py -at $vframes 3
    ./ffmpeg.exe -y -hwaccel nvdec -hwaccel_output_format cuda -ss $from -i $file -framerate 30 -filter_complex "scale_cuda=1920:1080,hwdownload,format=nv12 [base]" -map "[base]" -c:v h264_nvenc -b:v "6M" -fps_mode passthrough -vframes $vframes -to $duration $temp
    mpv $temp --loop=inf --keep-open
    $selection = Read-Host "Enter adjustment <start;end[;m]> or empty to stop"
    if ("" -eq $selection) {
        Write-Output "Final: $from $to"
        break
    }
    $adjustment = $selection -Split ";"
    if ("m" -eq $adjustment[2]){
        $from = python tc.py -mat $from $adjustment[0]
        $to = python tc.py -mat $to $adjustment[1]
    }else{
        $from = python tc.py -at $from $adjustment[0]
        $to = python tc.py -at $to $adjustment[1]
    }
}

Move-Item -Path $temp -Destination $output -Force

