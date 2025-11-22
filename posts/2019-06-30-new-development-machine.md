created: 2019-06-30
css: /css/post.css
tags: dev

{{../templates/nav.html}}

# New Development Machine

I just took delivery of my new Razer Blade Pro to take with me on my travels.

## Here's how I set it up

I'm not a huge fan of Windows. It does the job, it's where I started however
Linux automation has been around for decades. Microsoft are slowly catching up
However, I still play computer games from time-to-time and Windows is where it's
at for that.

Fortunately, initiatives such as the Ubuntu Subsystem for Windows, Windows
Terminal, PowerShell and containerization such as Docker is making it easier.

I tend to use Docker to develop Ruby applications and stick to Windows
for Node, React & F# development.

## Install [sync.com](https://sync.com)

So I can grab my password file to setup the other services and sync all
the things with my new machine.

## Auto-install apps with Chocolatey

[Chocolatey](https://chocolatey.org) is a much needed project for Windows
and while it's still maturing, it's got a lot of legacy and custom installs
to deal with. It's what I use to install the majority of apps.

    cinst audacity audacity-lame azure-cli brave calibre ctags docker-desktop `
    dropbox foobar2000 fraps fzf gimp git InkScape keepass neovim nodejs `
    plexmediaplayer plexmediaserver python python2 ripgrep ruby steam `
    treesizefree vlc wps-office-free

## Manually installed apps

With Steam it's fairly easy to copy the game library over with just a bit of
faff to re-verify some of the games.

* Cyberlink PowerDirector 16 (Key in Sync folder, files on backup drive)
* All my games which are on my backup drive thanks to Steam
* Ledger Live
* Reason 10
* reMarkable
* WhatsApp

## Development Setup

* Run Neovim.reg under `code_archive/_windows` (Need to move)
* Run `env_vars.cmd` under `code_archive/_windows` (Need to move)
* Restart after installing everything to get envs set and start docker

Run Neovim then `:PlugInstall`, restart it and due to my sync folder
containing Neovim config (location set in Env Vars XDG_CONFIG) it'll
install all the plugins and have nvim customizations set.

