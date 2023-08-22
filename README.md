# Net Top Box

Yet another IPTV proxy for Plex DVR. Use this to allow Plex DVR to support IPTV via an M3U playlist.

## Usage

Net Top Box is currently published as a Docker image. If you choose to build and run a binary, ensure you have ffmpeg installed.

Run `nettopbox` where Plex will be able to connect to it (e.g. on the same machine). At a minimum it will need your M3U URL.

For example, if you're running Plex with docker-compose, you could set it up like this:

```yaml
version: '3.7'

services:
    
    plex:
        image: plexinc/pms-docker
        # etc

    nettopbox:
        image: ghcr.io/jonohill/nettopbox
        restart: unless-stopped
        environment:
            - NTB_IPTV_URL=https://i.mjh.nz/nz/raw-tv.m3u8
            # The URL from Plex's perspective
            - NTB_BASE_URL=http://nettopbox:8080
```

See [Configuration](#configuration) for more options.

Once it's running, add a new tuner to Plex DVR. Click *Don't see your HDHomeRun device? Enter its network address manually* and enter the `<ip>:<port>` for nettopbox - in the docker-compose example it would be `nettopbox:8080`.

From there the set up should be the same as a physical tuner. Note that guide data is not handled, so you'll need to use Plex's data, or XMLTV.

## Configuration

Configuration is via environment variables or a config.yaml file, or both. Environment variables take precedence.

The config file will be read from the working directory, or pass the path as the first argument (e.g. `nettopbox /etc/nettopbox.yaml`).

| Environment Variable | Config Key | Description | Default |
| --- | --- | --- | --- |
| NTB_IPTV_URL | iptv_url | The URL to the M3U playlist. | (required) |
| NTB_BASE_URL | base_url | The URL that Plex will use to connect to nettopbox. | http://localhost:8080 |
| NTB_PORT | port | The port to listen on. | 8080 |
| NTB_TUNER_COUNT | tuner_count | The number of virtual tuners to expose to Plex. | 10 |

## Limitations

Only tested with [one provider](https://i.mjh.nz/nz/raw-tv.m3u8) (that supplies HLS streams).

If this doesn't work, try a more mature solution like [TVHeadEnd](https://tvheadend.org/)/[Antennas](https://github.com/jfarseneau/antennas).

## Motivation

For whatever reason, other proxies I tried were too buggy.

## Contributing

This was a weekend project to fill a particular need. You can create a PR if you like, but it may not be noticed for a while.
