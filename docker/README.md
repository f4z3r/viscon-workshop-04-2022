# Docker Hands-On

Before starting the exercises, make sure you have `docker` installed on your machine.

---

* [Foundational Docker](#foundational-docker)
  * [Pulling an Image](#pulling-an-image)
  * [Running a Container](#running-a-container)
  * [List all Containers](#list-all-containers)
  * [Get Logs](#get-logs)
  * [Delete a Container](#delete-a-container)
* [Intermediate Docker](#intermediate-docker)
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


## Intermediate Docker

<!-- TODO(@jakob):
   -
   - - Mount volume
   - - Inject environment variable
   - - Automatic restart
   - - Build from Dockerfile
   - - Write Dockerfile
   -->

## Advanced Docker

<!-- TODO(@jakob):
   -
   - - Write multi-stage / scratch
   - - Named volumes
   - - Networking (redis:6.2.6?)
   - - Trust
   - - Hack your host
   - -->


