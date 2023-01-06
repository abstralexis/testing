from pytube import YouTube

yt = YouTube("https://www.youtube.com/watch?v=dQw4w9WgXcQ")
video = yt.streams.get_highest_resolution()
video.download()    # type: ignore
