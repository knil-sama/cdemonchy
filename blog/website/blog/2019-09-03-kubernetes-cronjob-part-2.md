---
title: Kubernetes Cronjob 101: part 2
author: Clement Demonchy
---
In the first part we focused on how to set up your first cronjob with the proper configuration and manipulate them with the Kubernetes cli. In this second part, we will give you a more detailed explanation of our workflow
<!--truncate-->

The need
========

One issue we faced with our initial workflow was that not all of our jobs could be run concurrently, so when we were working on separate branches, we would create a lock on stateful resource such as a database.

The solution
========

The solution we came up with, relies on using the **\--from** option when creating a job that expects a deployed cronjob

So when we start working on a new cronjob, our workflow is as follows:

1.  Create a **suspended** cronjob
2.  Check cronjob deployment
3.  Create a job from this cronjob with **\--from** and check out the result
4.  Goto 1 until it work as expected
5.  Deploy it in production

cycle deployment cronjob

This help avoids the issue of manually changing schedules after each deployment to start the jobs, or letting bugged job stack up, possibly locking down resources.

The limitation when creating job this way is that you can’t alter it.

Therefore, what will you do if you have to handle occasional operations such as catching up for ETL job or rerun failed job with increased limits (timeout, CPU, …) ?

For example, let’s say you have a cronjob that runs every day and uses “yesterday” as starting date by default. What we do is rely on command line parameters for the script and let it set a default value for which there is no input. This way we can manually edit the cronjob:

Now that you have this code in production if you want to rerun it using previous dates, what you can do is create a duplicate of the cronjob.

Edit its content to reflect your new limitations (CPU, RAM, deadline) and then create a job from your duplicated cronjob (I recommend setting “suspend” to initially true to avoid concurrent runs ;) ) without impacting the one handling the normal run.

As we said, you need to rely on a script to handle the setup of a default value because you can’t call Linux command in you **args**, like **date** for example.

But did you know that you can use variable sets in **env** and **envFrom** in your script? You just need to call them using **$()** instead of **${}**. It can be useful if you don’t want your script to rely on env variables

It’s really convenient to be able to raise an alert when a job fails, sadly as we saw in the previous article this is not always easy due to the job itself, so what we do now is to generate a log at the start and the end on the pod runtime using the lifecycle option in our cronjob definition

But keep in mind that if you use a docker container with an entrypoint you can run into some issue where the lifecycle will be executed [AFTER the entrypoint](https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks).

Another great feature of Kubernetes is the [init containers](https://kubernetes.io/docs/concepts/workloads/pods/init-containers/) option, it consists of a list of ordered containers that will be run sequentially.

Each container waiting for the previous one to finish before launching, this is mainly used with the service API to sync them when deploying.

But this also works with a cronjob, so we can use them to do some sync or clean up before starting a job.

Time to wrap things up and go

So, in conclusion, even if the cronjob API itself didn’t evolve much since part 1, there are many ways to improve your current use by looking at other tools in Kubernetes whether it is kubectl, the API options or even by integrating it to your own CI.

EDIT:

Just found this great article on how to raise an alert if you use prometheus/grafana to analyze your Kubernetes cluster, [here](https://thinklumo.com/alert-failed-long-running-cronjobs-kubernetes-prometheus/)
