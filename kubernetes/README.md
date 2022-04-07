# Kubernetes Hands-on

Before starting the exercises, make sure you have [minikube](https://minikube.sigs.k8s.io/docs/start/) installed on your machine.

---

- [Kubernetes Hands-on](#kubernetes-hands-on)
  - [Kubernetes: First steps](#kubernetes-first-steps)
    - [Starting your cluster](#starting-your-cluster)
    - [Listing cluster resources](#listing-cluster-resources)
  - [Your first deployment](#your-first-deployment)
    - [Running an app](#running-an-app)
    - [Exploring your app](#exploring-your-app)
    - [Making your app accessible](#making-your-app-accessible)
    - [Connecting to your webserver](#connecting-to-your-webserver)
    - [Updating your app](#updating-your-app)
    - [Scaling your app](#scaling-your-app)
    - [Testing the load balancing](#testing-the-load-balancing)
  - [Intermediate Kubernetes](#intermediate-kubernetes)
    - [Declarative resource management](#declarative-resource-management)
    - [Setting virtual resources](#setting-virtual-resources)
  - [Advanced Kubernetes](#advanced-kubernetes)
    - [Configuring a deployment with data from ConfigMaps](#configuring-a-deployment-with-data-from-configmaps)
    - [Creating a Blue/Green Deployment](#creating-a-bluegreen-deployment)
    - [Share data between apps](#share-data-between-apps)

---

## Kubernetes: First steps

This section is intended to make you familiar with the basic `kubectl` command line tool, used to communicate with a `Kubernetes` cluster's control plane.

In order to solve the exercises in this section, you will probably need to inspect the help pages from kubectl using:

```bash
kubectl --help

# for sub-commands
kubectl <cmd> --help
```

Another useful resource ist the kubectl [cheat sheet](https://kubernetes.io/docs/reference/kubectl/cheatsheet/).

---
---

### Starting your cluster

This exercise is intended to make sure your local Kubernetes cluster is up and running.

<details>
  <summary>Tip</summary>

Check out the `minikube --help` command.

</details>

<details>
  <summary>Solution</summary>

```
$ minikube start
😄  minikube v1.22.0 on Ubuntu 18.04 (amd64)
✨  Using the docker driver based on existing profile
👍  Starting control plane node minikube in cluster minikube
🚜  Pulling base image ...
...
```

</details>

---
---

### Listing cluster resources

To get familiar with Kubernetes, let's first get an overview of all resources we can create on the cluster. For that, run `kubectl api-resources`. This provides a long list of resources known to your Kubernetes cluster. But don't worry, we're only interested in a few of those. Now that you know what resources exist, try to get some information about them from the cluster. Look up the nodes of your cluster.

<details>
  <summary>Tip</summary>

Use the `kubectl get` command.

</details>

<details>
  <summary>Solution</summary>

```
$ kubectl get nodes
NAME           STATUS   ROLES                  AGE    VERSION
minikube       Ready    control-plane,master   219d   v1.21.2
```

</details>

---
---

## Your first deployment

Now that we know your cluster is running, it's time to deploy the first application on it.

### Running an app

In the previous exercises, you learnt about Docker containers. We'll use an `nginx:1.21.6` image to spin up a webserver on our cluster. Create a deployment running the mentioned image.

<details>
  <summary>Tip</summary>

Use the `kubectl create deployment` command.

</details>

<details>
  <summary>Solution</summary>

```
$ kubectl create deployment nginx --image nginx:1.21.6
deployment.apps/nginx created
```

</details>

---
---

### Exploring your app

In this exercise, we'll look at how we can use `kubectl` to check the status of the `nginx` deployment. Use `kubectl` to find out if your deployment is running.

<details>
  <summary>Tip</summary>

Use the `kubectl get` command.

</details>

<details>
  <summary>Solution</summary>

```
$ kubectl get deployments
NAME    READY   UP-TO-DATE   AVAILABLE   AGE
nginx   1/1     1            1           12m
```

Or, for even more information:

```
$ kubectl get deployments -o wide
NAME    READY   UP-TO-DATE   AVAILABLE   AGE   CONTAINERS   IMAGES        SELECTOR
nginx   1/1     1            1           13m   nginx        nginx:1.21.6   app=nginx
```

</details>

---
---

### Making your app accessible

Your server is running but so far we have no way to connect to it. We need to expose the deployment to be able to connect to the server. In Kubernetes this happens with the help of a service. A service is an abstract resource which makes a an application available as network service. Use `kubectl` to expose your webserver. The service should listen on port `8080` and target nginx on port `80`. Verify that the service has been created.

<details>
  <summary>Tip</summary>

Use the `kubectl expose` and `kubectl get` commands.

</details>

<details>
  <summary>Solution</summary>

```
$ kubectl expose deployment nginx --port 8080 --target-port 80
service/nginx exposed
```

To check that the service exists:

```
$ kubectl get service
NAME         TYPE        CLUSTER-IP     EXTERNAL-IP   PORT(S)    AGE
kubernetes   ClusterIP   10.96.0.1      <none>        443/TCP    219d
nginx        ClusterIP   10.96.78.215   <none>        8080/TCP   25s
```

</details>

You can see that we now have a service exposing our nginx webserver and that an IP got assigned to it.

</details>

---
---

### Connecting to your webserver

In this exercise we'll connect to our webserver through the service we've just created. The easiest way to to so, is to use minikube which offers a convenient command for this.
Connect to the service using the `minikube` command and confirm the server is listening.

<details>
  <summary>Tip</summary>

Use the `minikube service` command.

</details>

<details>
  <summary>Solution</summary>

```
$ minikube service nginx
|-----------|-------|-------------|--------------|
| NAMESPACE | NAME  | TARGET PORT |     URL      |
|-----------|-------|-------------|--------------|
| default   | nginx |             | No node port |
|-----------|-------|-------------|--------------|
😿  service default/nginx has no node port
🏃  Starting tunnel for service nginx.
|-----------|-------|-------------|------------------------|
| NAMESPACE | NAME  | TARGET PORT |          URL           |
|-----------|-------|-------------|------------------------|
| default   | nginx |             | http://127.0.0.1:43961 |
|-----------|-------|-------------|------------------------|
🎉  Opening service default/nginx in default browser...
```

</details>

Also check the logs of the deployment to see the request received by the webserver.

<details>
  <summary>Tip</summary>

Use the `kubectl logs` command.

</details>

<details>
  <summary>Solution</summary>

```
$ kubectl logs deployment/nginx
kubectl logs deployment/nginx
192.168.49.2 - - [13/Mar/2022:14:13:34 +0000] "GET / HTTP/1.1" 200 612 "-" "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.71 Safari/537.36"
```

</details>

---
---

### Updating your app
An update is ready for your webserver. The new version has been packaged inside the image `containous/whoami`. Update the existing deployment with this new image.

<details>
  <summary>Tip</summary>

Use the `kubectl set` command.

</details>

<details>
  <summary>Solution</summary>

```
$  kubectl set image deployment/nginx nginx=containous/whoami
deployment.apps/nginx image updated
```

</details>

---
---

### Scaling your app

A great feature of Kubernetes is the ability to easily scale your deployment to match the number of incoming requests. In this exercise you'll scale the existing webserver to accomodate a higher number of requests. Scale the deployment by increasing the number of running server instances to 5.

<details>
  <summary>Tip</summary>

Use the `kubectl scale` command.

</details>

<details>
  <summary>Solution</summary>

```
$ kubectl scale deployment nginx --replicas 5
deployment.apps/nginx scaled
```

</details>

---
---

### Testing the load balancing

Now that we have a deployment with five webserver instances running, we want to make sure that the service we created earlier 1) still works and 2) distributes the load evenly across the instances. Use `curl` to query the service from your command-line and observe that your request gets relayed to different server instances.

<details>
  <summary>Tip</summary>

Use the `minikube service` and `curl` commands. Observe the `Hostname` returned by the server.

</details>

<details>
  <summary>Solution</summary>

If you stopped the forwarding to the nginx service, establish it again:

```
$ minikube service nginx
|-----------|-------|-------------|--------------|
| NAMESPACE | NAME  | TARGET PORT |     URL      |
|-----------|-------|-------------|--------------|
| default   | nginx |             | No node port |
|-----------|-------|-------------|--------------|
😿  service default/nginx has no node port
🏃  Starting tunnel for service nginx.
|-----------|-------|-------------|------------------------|
| NAMESPACE | NAME  | TARGET PORT |          URL           |
|-----------|-------|-------------|------------------------|
| default   | nginx |             | http://127.0.0.1:42833 |
|-----------|-------|-------------|------------------------|
```

Then, use the returned address to query the webservers:

```
$ curl http://127.0.0.1:42833/
Hostname: nginx-6cd7556f4f-sjcj2
IP: 127.0.0.1
IP: 10.244.1.2
RemoteAddr: 192.168.49.2:22343
GET / HTTP/1.1
Host: 127.0.0.1:42833
User-Agent: curl/7.58.0
Accept: */*
```

Observe the reported `Hostname`. It should change when you call the server multiple times. This means that our service load-balances the request successfully.

</details>

---
---

## Intermediate Kubernetes

This section covers some more advanced Kubernetes topics. In particular, you will use a `declarative` approach to manage cluster resources.

### Declarative resource management

So far, we have used Kubernetes purely in `imperative` manner. This means, we have always told Kubernetes exactly what to do. (e.g. create a deployment, update an image, scale the replicas) This requires many commands to be sent with `kubectl`. Wouldn't it be easier if you could just write down what we want and let Kubernetes figure out how to achieve that configuration? This is called a `declarative` approach and is supported by Kubernetes through config files, typically written in `YAML`.

In Kubernetes, every resource has a corresponding config file. Fetch the configuration of your deployment and save it as `YAML` file.

<details>
  <summary>Tip</summary>

Use the `kubectl get` command. Look for a flag you can pass to the command to retrieve the `YAML` config.

</details>

<details>
  <summary>Solution</summary>


```
$ kubectl get deployment nginx -o yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  annotations:
    deployment.kubernetes.io/revision: "2"
  creationTimestamp: "2022-03-13T12:45:41Z"
  generation: 5
  labels:
    app: nginx
  name: nginx
  namespace: default
  ...
```

To save the config:

```
$ kubectl get deploy nginx -o yaml > deployment.yaml
```

</details>

---
---

### Setting virtual resources

Now that we have the config, we're free to edit it to our liking. In this exercise, we'll set virtual resources, CPU and memory, to our deployment. In the `YAML` you just saved, there is a section `resources: {}`. Edit that section, such the instances request `0.5` virtual CPUs and `256M` memory, while the resource limit is `1` CPU and `512M` memory.
Apply the new config to the cluster.

<details>
  <summary>Tip</summary>

Check out [this](https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/) documentation. It explains how to define resources.

In order to apply your changes, you'll need the `kubectl apply` command.

</details>

<details>
  <summary>Solution</summary>

This is what the `resources` section should look like:

```yaml
...
spec:
  containers:
    - image: containous/whoami
    imagePullPolicy: IfNotPresent
    name: nginx
    resources:
      requests:
        memory: "256M"
        cpu: "0.5"
      limits:
        memory: "512M"
        cpu: "1"
  ...
```

To apply your changes (ignore the warning):

```
$ kubectl apply -f deployment.yaml
Warning: resource deployments/nginx is missing the kubectl.kubernetes.io/last-applied-configuration annotation which is required by kubectl apply. kubectl apply should only be used on resources created declaratively by either kubectl create --save-config or kubectl apply. The missing annotation will be patched automatically.
deployment.apps/nginx configured
```

</details>

---
---

## Advanced Kubernetes

### Configuring a deployment with data from ConfigMaps

Another useful resource in Kubernetes is the `ConfigMap`. As the name implies, it can be used to store configurations which can then be made availabe inside deployments. That way, if a config changes, only the ConfigMap needs to be adapted. In this exercise, we'll start a second webserver deployment but instead of the default nginx page, we'll display a custom HTML file configured via ConfigMap.

1. Create a ConfigMap containing the `kubernetes/hello_world.html` file as data. The file can be found in this repository.
2. Create a deployment `nginx-custom` with an `nginx:1.21.6` image, a `volume` populated with data from the ConfigMap, and a `volumeMount`, mounting the HTML file at `/usr/share/nginx/html/index.html` inside the container.
3. Expose the new deployment.
4. Verify that the server shows our custom HTML

<details>
  <summary>Tip 1</summary>

In order to create the ConfigMap, check out the `kubectl create configmap` command.

</details>

<details>
  <summary>Tip 2</summary>

Edit the YAML from the previous exercise. Adapt the name of the deployment. For configuring Volumes and mounting the data, have a look at [this article](https://kubernetes.io/docs/tasks/configure-pod-container/configure-pod-configmap/#populate-a-volume-with-data-stored-in-a-configmap).

</details>

<details>
  <summary>Solution</summary>

Create the ConfigMap:

```
$ kubectl create configmap html-data --from-file index.html=kubernetes/hello_world.html
configmap/html-data created
```

A minimal deployment YAML looks like this:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  labels:
    app: nginx-custom
  name: nginx-custom
  namespace: default
spec:
  replicas: 1
  selector:
    matchLabels:
      app: nginx-custom
  template:
    metadata:
      labels:
        app: nginx-custom
    spec:
      containers:
        - image: nginx:1.21.6
          name: nginx
          volumeMounts:
            - name: html-volume
              mountPath: /usr/share/nginx/html
      volumes:
        - name: html-volume
          configMap:
            name: html-data
```

Apply the depoyment YAML:

```
$ kubectl apply -f nginx-custom.yaml
deployment.apps/nginx-custom created
```

Expose the the deployment:

```
$ kubectl expose deployment/nginx-custom --port 9090 --target-port 80
service/nginx-custom exposed
```

Use `minikube service` to forward the service and verify that our HTML has been set as index page:

```
$ minikube service nginx-custom
|-----------|--------------|-------------|--------------|
| NAMESPACE |     NAME     | TARGET PORT |     URL      |
|-----------|--------------|-------------|--------------|
| default   | nginx-custom |             | No node port |
|-----------|--------------|-------------|--------------|
😿  service default/nginx-custom has no node port
🏃  Starting tunnel for service nginx-custom.
|-----------|--------------|-------------|------------------------|
| NAMESPACE |     NAME     | TARGET PORT |          URL           |
|-----------|--------------|-------------|------------------------|
| default   | nginx-custom |             | http://127.0.0.1:36545 |
|-----------|--------------|-------------|------------------------|
```

With cURL:

```
$ curl http://127.0.0.1:36545
<!DOCTYPE html>
<html>

<head>
    <title>Hello Kubernetes!</title>
</head>

<body>
    <p>This is an example of a simple HTML page with one paragraph.</p>
</body>

</html>
```


</details>

---
---

### Creating a Blue/Green Deployment

In this exercise, we will create and test a [blue/green deployment](https://www.redhat.com/en/topics/devops/what-is-blue-green-deployment). The goal is to have a replicated webserver with a total of `10 replicas`. 80% of traffic should be routed to the old (blue) version, while 20% of traffic should hit our new (green) version.
Create two deployments `nginx-blue` and `nginx-green`, both with image `nginx:1.21.6`, in such a way that the traffic requirements are met. In order to make verification possible, make sure that `nginx-blue` mounts `kubernetes/blue.html` at `/usr/share/nginx/html/index.html`, while `nginx-green` mounts `kubernetes/green.html`.

<details>
  <summary>Tip</summary>

A Kubernetes service uses `selectors` to select the replicas to which traffic is routed. In our case, the selector is a `matchLabels` selector. This means that our service routes traffic to all replicas that have a certain label configured. To solve the excercise, make sure to set the same label on all 10 replicas.

More info:
  - [Labels and Selectors](https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/)
  - [Services](https://kubernetes.io/docs/concepts/services-networking/service/)

</details>

<details>
  <summary>Solution</summary>

The basic idea is to craete two deployments, one with 8 and the other with 2 replicas. Both deployments should add the same labels to the replicas. Then, when we expose one of the deployments using a service, traffic will be routed to both deployments because the label selector of the service matches all 10 replicas. Since we have a 8-to-2 ratio of replicas, 80% of traffic will automatically hit the old (blue) version while the remaining 20% are routed to the new (green) version.

Create the HTML ConfigMaps:

```
$ kubectl create configmap html-blue-data --from-file index.html=kubernetes/blue.html
configmap/html-blue-data created
```

```
$ kubectl create configmap html-green-data --from-file index.html=kubernetes/green.html
configmap/html-green-data created
```

Craete the deployments:

Blue deployment:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  labels:
    app: nginx-blue
  name: nginx-blue
  namespace: default
spec:
  replicas: 8
  selector:
    matchLabels:
      app-version: nginx-blue
  template:
    metadata:
      labels:
        app: nginx-blue-green
        app-version: nginx-blue
    spec:
      containers:
        - image: nginx:1.21.6
          name: nginx
          volumeMounts:
            - name: html-volume
              mountPath: /usr/share/nginx/html
      volumes:
        - name: html-volume
          configMap:
            name: html-blue-data
```

```
$ kubectl apply -f nginx-blue.yaml
deployment.apps/nginx-blue created
```

Green deployment:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  labels:
    app: nginx-green
  name: nginx-green
  namespace: default
spec:
  replicas: 2
  selector:
    matchLabels:
      app-version: nginx-green
  template:
    metadata:
      labels:
        app: nginx-blue-green
        app-version: nginx-green
    spec:
      containers:
        - image: nginx:1.21.6
          name: nginx
          volumeMounts:
            - name: html-volume
              mountPath: /usr/share/nginx/html
      volumes:
        - name: html-volume
          configMap:
            name: html-green-data
```

```
$ kubectl apply -f nginx-green.yaml
deployment.apps/nginx-green created
```
Now, let's create a service that matches both deployments by matching only the `app: nginx-blue-green` label:

```
$ kubectl expose deployment nginx-blue --name nginx-blue-green --port 7070 --tar
get-port 80 --selector app=nginx-blue-green
service/nginx-blue-green exposed
```

Note: we exposed the deployment `nginx-blue` but by giving the appropriate selector with `--selector app=nginx-blue-green` the service matchs both deployments.
We can check this with the following command:

```
$ kubectl get endpoints nginx-blue-green
NAME               ENDPOINTS                                                  AGE
nginx-blue-green   172.17.0.10:80,172.17.0.11:80,172.17.0.12:80 + 7 more...   112s
```

We can see that the service matches all 10 replicas. We are ready to test!

```
$ minikube service nginx-blue-green
|-----------|------------------|-------------|--------------|
| NAMESPACE |       NAME       | TARGET PORT |     URL      |
|-----------|------------------|-------------|--------------|
| default   | nginx-blue-green |             | No node port |
|-----------|------------------|-------------|--------------|
😿  service default/nginx-blue-green has no node port
🏃  Starting tunnel for service nginx-blue-green.
|-----------|------------------|-------------|------------------------|
| NAMESPACE |       NAME       | TARGET PORT |          URL           |
|-----------|------------------|-------------|------------------------|
| default   | nginx-blue-green |             | http://127.0.0.1:43001 |
|-----------|------------------|-------------|------------------------|
```

```
$ curl  http://127.0.0.1:43001
<!DOCTYPE html>
<html>

<head>
    <title>Hello from Blue!</title>
</head>

<body>
    <p>This is the old version speaking.</p>
</body>

</html>
``` 

You'll most likely get a response from one of the blue replicas. By trying again a couple of times, eventually you should see this:

```
$ curl  http://127.0.0.1:43001
<!DOCTYPE html>
<html>

<head>
    <title>Hello from Green!</title>
</head>

<body>
    <p>This is the new, shiny version speaking.</p>
</body>

</html>
```

Congrats! You have successfully created a Blue/Green deployment of your webapp.

</details>

---
---

### Share data between apps

So far, we have only looked at deployments that run a single `nginx` container per instance. However, Kubernetes supports running multiple containers alongside each other to allow for some interesting usecases. For example, one can have a container exporting metrics running alongside the main application container. Another common usecase is running an a proxy container that enforces authentication and rate-limiting in front of a webapp. To the outside, these containers usually appear as a single, cohesive unit. More information about multi-container deployments can be found [here](https://kubernetes.io/docs/concepts/workloads/pods/#how-pods-manage-multiple-containers).

One simple way, how mutliple containers that belong together, can exchange data is by mounting the same volume. That's what we're going to explore in this exercise.

Your task is to create a single [Pod](https://kubernetes.io/docs/concepts/workloads/pods/) with two containers, each based on the `alpine:3.15` image, that share an [`emptyDir` volume](https://kubernetes.io/docs/concepts/storage/volumes/#emptydir). One container will write the current date to a file in the volume every `2s` while the other container will watch the file for changes and write them to standard output. Then, display the logs of the second container to verify that everything works. You should see timestamps appearing in the logs every two seconds.

<details>
  <summary>Tip 1</summary>

Have a look at this [documentation](https://kubernetes.io/docs/tasks/access-application-cluster/communicate-containers-same-pod-shared-volume/) explaining how to share data between two containers in a Pod.

</details>

<details>
  <summary>Tip 2</summary>

For writing the current date, you can use the `date` command.
You'll need to set the `command` and `args` of the container accordingly.
Have a look at the [documentation](https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/).

</details>

<details>
  <summary>Tip 3</summary>

For watching the file contents, have your other container write out the file to stdout using the `tail -f <file>` command. Again, set `command` and `args` of the container to your desired command.

</details>

<details>
  <summary>Tip 4</summary>

Use `kubectl logs` to get the output of the consumer. As we're now running two containers in a pod, the syntax is as follows:

`kubectl logs <pod-name> <container-name>`

Also look at the `-f` option in order to stream the logs in realtime.

</details>

<details>
  <summary>Solution</summary>

First, let's create the Pod using the declarative approach:

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: two-containers
spec:
  restartPolicy: Never
  volumes:
  - name: shared-data
    emptyDir: {}
  containers:
  - name: producer
    image: alpine:3.15
    command: ["/bin/sh"]
    args: ["-c", "while true; do date >> /producer-data/dates.txt ; sleep 2; done"]
    volumeMounts:
    - name: shared-data
      mountPath: /producer-data
  - name: consumer
    image: alpine:3.15
    command: ["/bin/sh"]
    args: ["-c", "sleep 2 && tail -f /consumer-data/dates.txt"]
    volumeMounts:
    - name: shared-data
      mountPath: /consumer-data
```
Let's break this YAML down:

We have defined a single emptyDir volume under the Pod's `volumes` section with the name `shared-data`.

Under `containers` we have defined two containers with the names `consumer` and `producer`. Both are running the image `alpine:3.15`. The containers have each mounted the the `shared-data` volume to a local path in their filesystem. `producer` mounts the volume at `producer-data` while `consumer` mounts it at `consumer-data`. Although the local mount path is different, both locations are backed by the same emptyDir volume.

For each container, we overwrite the command they execute on startup. In both cases we choose to execute the `/bin/sh` shell. However, the arguments passed are different. For the producer, we create a loop that writes the date to `/producer-data/dates.txt`. The consumer waits 2s to give the producer time to create the file, then it starts to watch the file `/consumer-data/dates.txt` for changes and writes them to stdout. Note again that `/producer-data/dates.txt` and `/consumer-data/dates.txt` point to the same file in `shared-data`.

Let's apply the definition:

```
$ kubectl apply -f two-containers.yaml
pod/two-containers created
```

Now we use `kubectl logs` to get the stdout of our consumer:

```
$ kubectl logs two-containers consumer -f
Tue Mar 22 10:33:00 UTC 2022
Tue Mar 22 10:33:02 UTC 2022
Tue Mar 22 10:33:04 UTC 2022
Tue Mar 22 10:33:06 UTC 2022
Tue Mar 22 10:33:08 UTC 2022
Tue Mar 22 10:33:10 UTC 2022
Tue Mar 22 10:33:12 UTC 2022
Tue Mar 22 10:33:14 UTC 2022
Tue Mar 22 10:33:16 UTC 2022
Tue Mar 22 10:33:18 UTC 2022
Tue Mar 22 10:33:20 UTC 2022
```
Every two seconds we get a new timestamp. Both, the producer and consumer are working as expected.

</details>