from invoke import task


@task
def build(c):
    print("Building!")


@task
def deploy(c):
    print("Deploying!")
