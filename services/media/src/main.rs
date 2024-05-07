use ffmpeg_sidecar::event::{FfmpegEvent, FfmpegProgress};

fn main() {

}


fn ffmpeg_experiment(){
    ffmpeg_sidecar::download::auto_download()
        .expect("Failed to download ffmpeg");

    let version = ffmpeg_sidecar::version::ffmpeg_version()
        .expect("Failed to get ffmpeg version");
    println!("ffmpeg version: {}", version);


    let mut command = ffmpeg_sidecar::command::FfmpegCommand::new();
    command.hwaccel("auto")
        .format("lavfi")
        .input("testsrc=size=1920x1080:rate=60:duration=30");

    for (video_codec, audio_codec, crf) in [("h264", "aac", 23)] {
        for (height, fps) in [("1080", 60.0),("1080", 30.0), ("720", 30.0), ("480", 25.0), ("360", 25.0), ("240", 10.0), ("144", 10.0)] {
            let output = format!("stream/test-{video_codec}-{height}-{fps}.mkv");
            let resize_filter = format!("scale=-2:{}", height);
            command.codec_video(video_codec)
                .codec_audio(audio_codec)
                .crf(crf)
                .rate(fps)
                .preset("slow")
                .args(["-vf", &resize_filter])
                .overwrite()
                .output(output);
        }
    }

    let mut command = command.spawn()
        .expect("Failed to spawn ffmpeg");

    command.iter().unwrap().for_each(|e| {
        match e {
            FfmpegEvent::Progress(FfmpegProgress { frame, time, fps, .. }) =>
                println!("[Progress]: {frame} {fps}/s ({time})"),
            FfmpegEvent::Log(_level, msg) =>
                println!("[ffmpeg] {msg}"),
            _ => {}
        }
    });
    command.wait().expect("Failed to wait for ffmpeg");
}