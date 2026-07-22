# Sample Job

A job contains the following:
- A `job.json` that specifies how the job is supposed to be dispatched
- An archive `job-data.zip` that will be then unzipped in the container at the
  path `/workspace/` and `/workspace/entypoint.sh` will be called.

A `job.json` contains:
- `image` -> full docker image name, with tag included. Note that image names
  may not correspond to names of images found on places like dockerhub. Runners
  may decide that the `ubuntu` image actually corresponds to a custom ubuntu
  image with some pre-installed packages.
- `hardware` -> hardware configuration to run on. In sample this is "dummy" for
  no hardware. Each hardware runner can define its own no-hardware config. It is
  not standard.
- `stdout_artifact` -> Generate stdout of entrypoint as an artifact.
- `stderr_artifact` -> Generate stderr of entrypoint as an artifact.
- `artifacts` -> list of paths (relative to workspace) to output as artifacts

Artifacts are files which are then extracted before the container is destroyed.
These can be job run information, or anything the user desires in file format.
STDOUT and STDERR artifacts are a bit special, in the sense that they are not
backed by a file the user defines, but rather on startup the docker container
will redirect the outputs to reserved file paths, which can then be copied of as
a normal artifact would be.

