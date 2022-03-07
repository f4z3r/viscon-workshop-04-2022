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

## Intermediate Docker

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


## Advanced Docker

<!-- TODO(@jakob):
   -
   - - Named volumes
   - - BuildKit
   - - Networking (redis:6.2.6?)
   - - Trust
   - - Hack your host
   - -->


