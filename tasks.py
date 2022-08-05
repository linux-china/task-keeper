from invoke import task


@task
def build(c):
    """
    build project
    """
    print("Building!")


@task
def deploy(c):
    print("Deploying!")
