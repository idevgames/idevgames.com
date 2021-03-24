# iDevGames.com

This is the code which runs iDevGames.com, a games programming community. We
share links to articles which interest us and periodically run games development
jams to encourage positivity and productivity.

## Hacking

This site is open-source software, and we welcome pull-requests. If you'd like
to start working on this site, either to improve it or learn from it for your
own projects, these instructions should get you going with a local copy you can
work with.

By the way of toolchain dependencies, to work on this site you need Rust,
NodeJS, and a Github OAuth application.

First, clone the site down to your own machine.

```bash
git clone git@github.com:idevgames/idevgames.com
```

### NodeJS

Install NodeJS using a method appropriate to your operating system and
preferences. Then install Brunch, a JS tool that compiles JS/SASS to things that
are static and can be used by the browser.

```bash
npm i -g brunch # this may require sudo
```

Now install Brunch's dependencies.

```bash
npm i
```

That's the NodeJS part done.

### Rust

Make sure you have a working Rust compiler.

```bash
rustc --version
```

Ensure that your system is configured for the `openssl-sys` crate:
https://docs.rs/openssl/0.10.29/openssl/.

To manage the database migrations install the Diesel CLI.

```bash
# or equivalent on your os
sudo apt-get install -y libssl-dev libsqlite3-dev pkg-config build-essential
cargo install diesel_cli --no-default-features --features sqlite
diesel setup
```

With that done you are ready to compile the website, both client-side assets and
the server-side daemon. There's an easy shorthand script for this, called
`watch.sh`. It starts Brunch and the server daemon at the same time, so you can
iterate on server-side assets without having to manage multiple terminal windows
or anything. If you're running one-off Rust processes without the need to modify
client-side assets such as JavaScript or CSS it is unnecessary.

```bash
./watch.sh serve
```

### Github

Managing the OAuth lifecycle with Github was done from these reference docs:
https://docs.github.com/en/rest/guides/basics-of-authentication.

To log into iDevGames.com locally, first configure an application with Github:
https://github.com/settings/applications/new

**name**: iDevGames-your-github-alias  
**homepage url**: https://www.idevgames.com/  
**description**: A games development community.  
**callback**: http://localhost:4000/gh_callback  

Note that if you use WSL2 you will have to update that callback to your VM's
current IP address. You can get that address with `hostname -I`. There is a
script called `wsl.sh` which makes this easier, setting the bind address. Call
it with the program arguments you desire, such as `./wsl.sh serve` or
`./wsl.sh migrate`.

Finally, you'll need to configure the application. See `dotenv`, copying that
locally to a `.env` file and filling it in per the instruction in the file.

### Permissions

Now that you have all that set up you can give yourself permission to change
the resources on your local site! You can grant yourself admin permissions with
the following:

```bash
cargo run permission grant -u your_github_user_name -p admin
```

To undo that and see what the site looks like to everyone else again, use the
revoke command.

```bash
cargo run permission revoke -u your_github_user_name -p admin
```

Happy hacking!

## Deploying

Put the built program on a machine and run it. If you're patching it for
iDevGames use, then the process of deploying is very complicated: poke
mysteriouspants until he does it (he hasn't set up automation because things
seldom change).

If you're, for some reason, doing this yourself, these are mysteriouspants'
notes on the matter. They may be of some help for you!

Prepare the server

```sh
ssh mysteriouspants.com:
    # create user
    sudo adduser --disabled-password idevgames
    # enable ssh for this user - this is how we rsync new releases up
    sudo mkdir -p ~idevgames/.ssh
    sudo cp ~/.ssh/authorized_keys  ~idevgames/.ssh/authorized_keys
    sudo chown idevgames:idevgames ~idevgames/.ssh/authorized_keys
    sudo vim /etc/systemd/system/idevgames.service
```

The service file ought to look like this:

```
[Unit]
Description=iDevGames Web Server
After=network.target

[Service]
type=simple
User=idevgames
WorkingDirectory=/home/idevgames
ExecStart=/home/idevgames/idevgames serve
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

Add necessary environment variables.

```sh
ssh idevgames@mysteriouspants.com:
  vim .env
```

The necessary environment variables are this:

```
DATABASE_URL=db/app.sqlite
IDG_WORKERS=2
IDG_PORT=4000
IDG_MAXDBCONS=4
IDG_ADDRESS=12.0.0.1
IDG_COOKIE_SECRET=$(openssl rand -base64 32)
GH_CLIENT_ID=
GH_CLIENT_SERET=
```

Finally, run `deploy.sh` locally to build a release build and rsync the results
up, and kick the service on. You should mark the service to start on boot as
well. Because wild reboots happen.

```sh
local:
    ./deploy.sh
ssh mysteriouspants.com:
    sudo systemctl enable idevgames
```

From here, the service can be served behind a reverse proxy. I use nginx.

```
# /etc/nginx/sites-available/www.idevgames.com
server {
  server_name         www.idevgames.com;
  access_log          off;
  location / {
    proxy_pass http://127.0.0.1:4000;
  }
}
```

## Modification/Licensing

We want you to be able to use this software regardless of who you may be, what
you are working on, or the environment in which you are working on it - we hope
you'll use it for good and not evil! To this end, the iDevGames website source
code is licensed under the [2-clause BSD][2cbsd] license, with other licenses
available by request. Happy coding!

[2cbsd]: https://opensource.org/licenses/BSD-2-Clause
