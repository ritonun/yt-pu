mod cli;
mod dl;
mod models;
mod reader;
mod utils;

fn main() {
    let config = cli::parse_args();

    // get local videos
    let local_videos = reader::get_local_video(&config);
    println!("[yt-pu] Found {} local videos", local_videos.len());

    // get online videos
    let mut online_videos = dl::get_online_videos(&config);
    println!(
        "[yt-pu] Found {} videos in the playlist",
        online_videos.len()
    );

    // remove local video from online playlist
    let old_len = online_videos.len();
    utils::remove_local_videos_from_videos_to_dl(&local_videos, &mut online_videos);
    println!(
        "[yt-pu] {} videos already downloaded, {} videos to download",
        (old_len - online_videos.len()),
        online_videos.len()
    );

    // if --delete-local option, move to trash video that have been removed
    // from online playlist
    utils::remove_local_not_in_playlist(&local_videos, &online_videos);

    // download videos
    match dl::dl_playlist(&online_videos, &config) {
        Ok(_) => println!("[yt-pu] Succesfully downloaded playlist"),
        Err(e) => eprintln!("[yt-pu] Failed to download playlist due to error {}", e),
    }
}
